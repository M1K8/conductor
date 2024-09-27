use clap::Parser;
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
    net::Ipv4Addr,
    time::Duration,
};
use tokio::sync::mpsc;
pub mod bambu;
pub mod commands;
pub mod input;
pub mod moonraker;
pub mod ui;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cmd = commands::Cmd::parse();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;

    match &cmd.mode {
        commands::Mode::Bambu { nested } => {
            match &cmd.mode.validate_necessary_args(&cmd) {
                None => {}
                Some(e) => panic!("{:?}", e),
            }
            let bbu = bambu::new(
                std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 0, 11)),
                &cmd.ftp_user,
                &cmd.ftp_pw,
                &cmd.mqtt_server,
                &cmd.bbu_dev_id,
            )
            .await;

            bbu.unwrap().handle(nested);
        }
        commands::Mode::Klipper => {
            match &cmd.mode.validate_necessary_args(&cmd) {
                None => {}
                Some(e) => panic!("{:?}", e),
            }
            let _moon = moonraker::Moonraker::new();
        }
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
            println!("💥‼️💥 Conductor exited with error {:?}", s)
        }
        None => {}
    }
    Ok(())
}
