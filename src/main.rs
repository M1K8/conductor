use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use tokio;

use std::{
    io::{self, stdout},
    net::Ipv4Addr,
    time::Duration,
};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use tokio::sync::mpsc;

pub mod bambu;
pub mod input;
pub mod moonraker;
pub mod ui;

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

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let moon = moonraker::Moonraker::new();
    let res = moon
        .test_ping(std::net::IpAddr::V4(Ipv4Addr::new(10, 0, 2, 11)))
        .await;

    match res {
        Ok(_) => {}
        Err(e) => {
            return cleanup(Some(e.to_string()));
        }
    }

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
