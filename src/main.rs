use mavlink::AsyncMavConnection;
use mavlink::dialects::common::MavMessage;
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use std::time::Duration;
use std::{env, thread};
use tokio::{task, time};
// use mavlink::error::MessageReadError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(address) = std::env::args().nth(1) else {
        eprintln!("usage: (udpin)|127.0.0.1|14550");
        return Ok(());
    };
    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1884);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    // create event loop poll
    let mqtt_task = tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(event) => {
                    println!("mqtt! {event:?}",)
                }
                Err(error) => {
                    eprintln!("error {}", error);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });

    /*
     * note to self:
     * await? returns
     * Ok(val) => val
     * Err(errval) => errval
     * */
    // publish mavconn as a client publishing to a mav topic

    let mut mavconn = mavlink::connect_async::<MavMessage>(&address).await?;
    mavconn.set_allow_recv_any_version(true);

    loop {
        // just read
        let (header, message) = mavconn.recv().await?;
        let topic = format!("mav/{}/data", header.system_id);
        let payload = format!("{message:?}");

        client
            .publish(topic, QoS::AtMostOnce, false, payload)
            .await?;
    }
}

async fn request(client: AsyncClient) {
    client
        .subscribe("mav/1/data", QoS::AtMostOnce)
        .await
        .unwrap();
}

async fn publish(client: AsyncClient) {
    // thread::sleep(Duration::from_secs(1));
    client
        .subscribe("mav/1/data", QoS::AtMostOnce)
        .await
        .unwrap();

    client
        .publish("mav/1/data", QoS::AtLeastOnce, true, vec![1, 2, 3])
        .await
        .unwrap();

    // thread::sleep(Duration::from_secs(1));
}
