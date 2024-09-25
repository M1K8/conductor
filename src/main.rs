use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Layout},
    widgets::Block,
    Frame, Terminal,
};
use tokio;

use std::{
    io::{self, stdout},
    time::Duration,
};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};

use tokio::sync::mpsc;

pub mod input;

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
                    None =>  terminal.draw(ui)?,
                };
            },
            _ = ticker.tick() => {
                terminal.draw(ui)?;
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(DisableMouseCapture)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame) {
    let [title_area, main_area, status_area] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ])
    .areas(frame.area());
    let [left_area, right_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
