use ratatui::crossterm::event::{Event, KeyCode};
use tokio;

use std::time::Duration;

use crossterm::event::{EventStream, MouseButton, MouseEventKind};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use tokio::sync::mpsc::Sender;

pub async fn await_input(tx: Sender<()>) {
    let mut reader = EventStream::new();
    loop {
        let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
        let mut event = reader.next().fuse();
        select! {
            _ = delay => { },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        match event {
                            Event::FocusGained => todo!(),
                            Event::FocusLost => todo!(),
                            Event::Key(k) => {
                                if k == KeyCode::Esc.into() || k == KeyCode::Char('q').into(){
                                    let send_err  = tx.send(()).await;
                                    match send_err {
                                        Ok(_) => break,
                                        Err(e) => panic!("{:?}",e)
                                    }
                                }
                            },
                            Event::Mouse(m) => {
                              match m.kind{
                                  MouseEventKind::Down(d) => {
                                    if d == MouseButton::Left {
                                    let send_err  = tx.send(()).await;
                                    match send_err {
                                        Ok(_) => break,
                                        Err(e) => panic!("{:?}",e)
                                    }
                                  }},
                                  MouseEventKind::Up(_u) => {},
                                 _ => {},
                              }

                            },
                            Event::Paste(_) => todo!(),
                            _ => {},
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        };
    }
}
