use std::env;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use mavlink::MavConnection;
use mavlink::dialects::common::MavModeFlag;
use mavlink::error::MessageReadError;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: gcs (udpout|udpin):(ip):(port|baud)");
        return;
    }

    let mut mavconn = mavlink::connect::<mavlink::dialects::common::MavMessage>(&args[1]).unwrap();

    // protocol is Mavlink2,
    mavconn.set_protocol_version(mavlink::MavlinkVersion::V2);

    // get the vehicle per se
    // we use arc for shared ownership
    let vehicle = Arc::new(mavconn);

    vehicle
        .send(&mavlink::MavHeader::default(), &request_stream())
        .unwrap();

    // try synchronous loop to get heartbeat
    loop {
        // receive message through .recv
        match vehicle.recv() {
            Ok((_header, msg)) => {
                println!("received: {msg:?}")
            }

            Err(MessageReadError::Io(e)) => {
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    // wwait
                    thread::sleep(Duration::from_secs(1));
                    continue;
                } else {
                    println!("recv error : {e:?}");
                    break;
                }
            }
            _ => {}
        }
    }
}

pub fn heartbeat_message() -> mavlink::dialects::common::MavMessage {
    mavlink::dialects::common::MavMessage::HEARTBEAT(mavlink::dialects::common::HEARTBEAT_DATA {
        custom_mode: 0,
        mavtype: mavlink::dialects::common::MavType::MAV_TYPE_QUADROTOR,
        autopilot: mavlink::dialects::common::MavAutopilot::MAV_AUTOPILOT_GENERIC,
        base_mode: MavModeFlag::empty(),
        system_status: mavlink::dialects::common::MavState::MAV_STATE_STANDBY,
        mavlink_version: 0x3,
    })
}

pub fn request_stream() -> mavlink::dialects::common::MavMessage {
    #[expect(deprecated)]
    mavlink::dialects::common::MavMessage::REQUEST_DATA_STREAM(
        mavlink::dialects::common::REQUEST_DATA_STREAM_DATA {
            target_system: 0,
            target_component: 0,
            req_stream_id: mavlink::dialects::common::MavDataStream::MAV_DATA_STREAM_ALL,
            req_message_rate: 10,
            start_stop: 1,
        },
    )
}
