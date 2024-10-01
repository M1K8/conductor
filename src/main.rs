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
pub mod bambu;
pub mod commands;
pub mod input;
pub mod moonraker;
pub mod ui;

#[tokio::main]
async fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;

    let cmd = match commands::Cmd::try_parse() {
        Ok(cmd) => cmd,
        Err(e) => {
            return cleanup(Some(e.to_string()));
        }
    };

    match &cmd.mode {
        commands::Mode::Bambu { nested } => {
            match &cmd.mode.validate_necessary_args(&cmd) {
                None => {}
                Some(e) => panic!("{:?}", e),
            }
            let ip = match cmd.dev_ip {
                Some(i) => i,
                None => {
                    println!("missing ip");
                    return cleanup(None);
                }
            };

            let user = match cmd.ftp_user {
                Some(i) => i,
                None => {
                    println!("missing user");
                    return cleanup(None);
                }
            };

            let pw = match cmd.ftp_pw {
                Some(i) => i,
                None => {
                    println!("missing pw");
                    return cleanup(None);
                }
            };

            let devid = match cmd.bbu_dev_id {
                Some(i) => i,
                None => {
                    println!("missing dev_id");
                    return cleanup(None);
                }
            };

            let (kill_tx, kill_recv) = tokio::sync::mpsc::channel(1);

            let mut bbu = match bambu::new(&user, &pw, &ip, &devid, kill_recv).await {
                Ok(b) => b,
                Err(e) => {
                    let ee = e.to_string(); //todo - error wrapping
                    return cleanup(Some(format!("while dialing mqtt server: {ee}")));
                }
            };

            match nested {
                Command::Interactive => todo!(),
                Command::Ping => {
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_secs(8)).await;
                        _ = kill_tx.send(Some(())).await;
                    });

                    bbu.handle(nested).await;
                    return cleanup(None);
                }
                Command::Print => todo!(),
                Command::Upload => todo!(),
            }
        }
        commands::Mode::Klipper => {
            match &cmd.mode.validate_necessary_args(&cmd) {
                None => {}
                Some(e) => {
                    _ = cleanup(Some(format!("{:?}", e)));
                    panic!("{:?}", e)
                }
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
            println!("💥‼️💥 Conductor exited with error: {s}")
        }
        None => {}
    }
    Ok(())
}
