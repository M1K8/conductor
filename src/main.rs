use clap::Parser;
use commands::Command;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use std::{
    io::{self, stdout},
    time::Duration,
};

use tokio::sync::mpsc;
pub mod commands;
pub mod config;
pub mod input;
pub mod printer;
pub mod ui;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;

    let cfg = match config::deser() {
        Ok(c) => c,
        Err(e) => return cleanup(Some(e.to_string())),
    };

    let cmd = match commands::Cmd::try_parse() {
        Ok(cmd) => cmd,
        Err(e) => {
            return cleanup(Some(e.to_string()));
        }
    };

    match &cmd.mode {
        Some(cmd) => match cmd {
            Command::Interactive => {}
            Command::Ping { printer } => {
                let printer_cfg = match config::get_printer(&cfg, printer) {
                    Some(p) => p,
                    None => {
                        return cleanup(Some(format!("printer {printer} not found").to_string()))
                    }
                };
            }
            Command::Print { printer, file } => {
                let printer_cfg = match config::get_printer(&cfg, printer) {
                    Some(p) => p,
                    None => {
                        return cleanup(Some(format!("printer {printer} not found").to_string()))
                    }
                };
            }
            Command::Upload { printer, file } => {
                let printer_cfg = match config::get_printer(&cfg, printer) {
                    Some(p) => p,
                    None => {
                        return cleanup(Some(format!("printer {printer} not found").to_string()))
                    }
                };
            }
        },
        None => {} //default to interactive
    }
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let (tx, mut rx) = mpsc::channel(1);
    tokio::spawn(async move {
        input::await_input(tx).await;
    });

    let mut ticker = tokio::time::interval(Duration::from_millis(500));

    loop {
        tokio::select! {
            evt = rx.recv() => {
                match evt {
                    Some(_) => break,
                    None =>  terminal.draw(ui::ui)?,
                };
            },
            _ = ticker.tick() => {
                terminal.draw(ui::ui)?;
            }
        }
    }

    cleanup(None)
}

fn cleanup(e: Option<String>) -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    match e {
        Some(s) => {
            println!("💥‼️💥 Conductor exited with error: {s}")
        }
        None => {}
    }
    Ok(())
}
