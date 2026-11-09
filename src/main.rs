use gcs::DroneState;
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

    // init dronestate struct
    let mut state = DroneState::new(1);
    let mut heartbeat_timer = tokio::time::interval(Duration::from_millis(100));
    heartbeat_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
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
    // we want to update the state
    // -> then serialize the state to publish the payload
    loop {
        // select -> task racing
        tokio::select! {
            // task 1: updating
            received = mavconn.recv() => {
                let (header, message) = received?;

                if header.system_id == state.system_id {
                    state.update(&message);
                }
            }
            // task 2: heartbeat tick to mqtt
            _ = heartbeat_timer.tick() => {
                    state.check_connection(Duration::from_secs(2));

                    let payload = serde_json::to_vec(&state)?;
                    let topic = format!("mav/{}/state", state.system_id);
                    client
                        .publish(topic, QoS::AtMostOnce, false, payload).await?;
                }
        }
        println!("{state}");
    }
}
