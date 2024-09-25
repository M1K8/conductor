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
    time::Duration,
};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use tokio::sync::mpsc;

pub mod input;
pub mod ui;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
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

    disable_raw_mode()?;
    stdout().execute(DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
