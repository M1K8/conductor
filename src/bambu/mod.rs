use rumqttc::{AsyncClient, ClientError, ConnectionError, Event, EventLoop, MqttOptions, QoS};
use std::{net::IpAddr, time::Duration};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;

pub struct Bambu {
    ftp_srv: IpAddr,
    ftp_user: String,
    ftp_pw: String,
    bb_dev_id: String,

    mqtt_client: AsyncClient,

    mqtt_recv: ReceiverStream<Event>,
    err_recv: Receiver<ConnectionError>,
}

pub async fn new(
    ftp_srv: IpAddr,
    ftp_user: String,
    ftp_pw: String,
    ms: String,
    bb_dev_id: String,
) -> Result<Bambu, ClientError> {
    let mut mqttoptions = MqttOptions::new("conductor", ms, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mqtt_client, evtloop) = AsyncClient::new(mqttoptions, 10);
    let (tx, rx) = tokio::sync::mpsc::channel(20);
    let (etx, err_recv) = tokio::sync::mpsc::channel(1);
    let mqtt_recv = ReceiverStream::new(rx);

    // Might aswell subscribe as a test, can always unsub later
    let err = mqtt_client
        .subscribe(format!("device/{bb_dev_id}/report"), QoS::AtMostOnce)
        .await;

    match err {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    let bbu = Bambu {
        ftp_srv,
        ftp_user,
        ftp_pw,
        bb_dev_id,
        mqtt_client,
        mqtt_recv,
        err_recv,
    };

    tokio::spawn(async move {
        let e = poll_mqtt(evtloop, tx).await.unwrap();
        _ = etx.send(e).await;
    });

    Ok(bbu)
}

async fn poll_mqtt(mut evtloop: EventLoop, tx: Sender<Event>) -> Option<ConnectionError> {
    let mut ticker = tokio::time::interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;
        match evtloop.poll().await {
            Ok(v) => {
                _ = tx.send(v).await;
            }
            Err(e) => {
                return Some(e);
            }
        }
    }
}
impl Bambu {}
