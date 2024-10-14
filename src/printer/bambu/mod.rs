use crate::commands;
use core::error;
use paho_mqtt::{self as mqtt, AsyncClient, AsyncReceiver, Message};
use report::Report;
use std::{env, error::Error, time::Duration};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use super::Device;
mod report;
pub(crate) struct Bambu<'a> {
    ftp_user: &'a str,
    ftp_pw: &'a str,
    bb_dev_id: &'a str,

    mqtt_client: AsyncClient,

    mqtt_recv: ReceiverStream<Message>,
    kill_chan: Receiver<Option<()>>,
}

pub(crate) async fn new<'a>(
    ftp_user: &'a str,
    ftp_pw: &'a str,
    ip: &'a str,
    bb_dev_id: &'a str,
    kill_chan: Receiver<Option<()>>,
) -> Result<Bambu<'a>, Box<dyn Error>> {
    const TRUST_STORE: &str = "/etc/ssl/cert.pem";

    let mut trust_store = env::current_dir()?;
    trust_store.push(TRUST_STORE);

    let ms = format!("ssl://{ip}:8883");
    let create_opts = mqtt::CreateOptionsBuilder::new_v3()
        .server_uri(ms)
        .client_id("conductor")
        .finalize();

    let ssl_opts = mqtt::SslOptionsBuilder::new()
        .trust_store(trust_store)?
        .finalize();

    // Create the client connection
    let mut mqtt_client = mqtt::AsyncClient::new(create_opts).unwrap();

    let (tx, rx) = tokio::sync::mpsc::channel(20);
    let mqtt_recv = ReceiverStream::new(rx);

    // Create the connect options, explicitly requesting MQTT v3.x
    let conn_opts = mqtt::ConnectOptionsBuilder::new_v3()
        .keep_alive_interval(Duration::from_secs(120))
        .connect_timeout(Duration::from_secs(5))
        .clean_session(false)
        .user_name(ftp_user)
        .password(ftp_pw)
        .ssl_options(ssl_opts)
        .finalize();
    match mqtt_client.connect(conn_opts).await {
        Ok(_) => {}
        Err(e) => {
            return Err(e.into());
        }
    }

    // Might aswell subscribe as a test, can always unsub later
    let err = mqtt_client
        .subscribe(format!("device/{bb_dev_id}/report"), 0)
        .await;

    match err {
        Ok(_) => {}
        Err(e) => return Err(Box::new(e)),
    }

    let mut str = mqtt_client.get_stream(100);

    let bbu = Bambu {
        ftp_user,
        ftp_pw,
        bb_dev_id,
        mqtt_client,
        mqtt_recv,
        kill_chan,
    };

    tokio::spawn(async move {
        poll_mqtt(&mut str, tx).await;
    });

    Ok(bbu)
}

async fn poll_mqtt(
    evtloop: &mut AsyncReceiver<Option<Message>>,
    tx: Sender<Message>,
) -> Option<()> {
    let mut ticker = tokio::time::interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;
        match evtloop.next().await {
            Some(v) => match tx.send(v?).await {
                Ok(_) => {
                    // If we're subbed to the main /report topic, we'll be getting enough
                    // messages to hog all of the tokio schedulers time, so add a lil yielding break
                    tokio::task::yield_now().await;
                }
                Err(e) => println!("{:?}", e),
            },
            None => {
                return None;
            }
        }
    }
}

impl<'a> Bambu<'a> {
    pub async fn start_mqtt(&mut self, _cmd: &commands::Command) -> Option<Box<dyn error::Error>> {
        loop {
            tokio::select!(evt = self.mqtt_recv.next() => {
                match evt {
                    Some(v) => {
                        let json = v.payload_str().to_string();
                        match serde_json::from_str::<Report>(&json){
                            Ok(o) => {
                                let str =  serde_json::to_string_pretty(&o).unwrap();
                                println!("{str}");
                            }
                            Err(e) => {
                                println!("{:?}",e);
                                return Some(e.into());
                            },
                        }

                    },None => {
                        return None;
                    },
                };
            },kill = self.kill_chan.recv() => {
                match kill {
                    Some(_) => {
                        self.mqtt_client.disconnect(None);
                        return None;
                    }None => {
                        return None;
                    }
                }
            })
        }
    }
}

impl Device for Bambu<'_> {
    fn print(&self, f: &super::PrintFile) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn upload(&self, f: &super::PrintFile) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn get_info(&self) -> std::collections::HashMap<String, String> {
        todo!()
    }
}
