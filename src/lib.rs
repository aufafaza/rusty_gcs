// let's define the message struct here
//

use mavlink::dialects::common::{MavMessage, MavModeFlag};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::{Duration, Instant};
/*
* Message struct for testing:
*   sysid,
*   connection,
*   latitude
*   longitude
*   altitude
* */

#[derive(Serialize, Deserialize)]
pub struct DroneState {
    pub system_id: u32,
    pub connected: bool,
    pub altitude_m: Option<f64>,
    pub latitude_deg: Option<f64>,
    pub longitude_deg: Option<f64>,
    #[serde(skip)]
    pub last_heartbeat: Option<std::time::Instant>,
}

impl fmt::Display for DroneState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Drone {} | connected: {} | lat: {:?} | lon: {:?} | altitude: {:?}",
            self.system_id, self.connected, self.latitude_deg, self.longitude_deg, self.altitude_m,
        )
    }
}
impl DroneState {
    pub fn new(sys_id: u32) -> Self {
        DroneState {
            system_id: sys_id,
            connected: false,
            altitude_m: None,
            latitude_deg: None,
            longitude_deg: None,
            last_heartbeat: None,
        }
    }

    pub fn update(&mut self, message: &MavMessage) {
        match message {
            MavMessage::GLOBAL_POSITION_INT(position) => {
                self.latitude_deg = Some(position.lat as f64 / 10_000_000.0);
                self.longitude_deg = Some(position.lon as f64 / 10_000_000.0);
                self.altitude_m = Some(position.alt as f64 / 1_000.0);
            }
            MavMessage::HEARTBEAT(_) => {
                self.last_heartbeat = Some(Instant::now());
                self.check_connection(Duration::from_secs(1)); // hardcode timeout to 1 sec 
            }
            _ => {}
        }

        // print current state
        println!("{self}");
    }

    pub fn heartbeat_message() -> mavlink::dialects::common::MavMessage {
        mavlink::dialects::common::MavMessage::HEARTBEAT(
            mavlink::dialects::common::HEARTBEAT_DATA {
                custom_mode: 0,
                mavtype: mavlink::dialects::common::MavType::MAV_TYPE_GCS,
                autopilot: mavlink::dialects::common::MavAutopilot::MAV_AUTOPILOT_INVALID,
                base_mode: MavModeFlag::empty(),
                system_status: mavlink::dialects::common::MavState::MAV_STATE_STANDBY,
                mavlink_version: 0x3,
            },
        )
    }

    pub fn check_connection(&mut self, timeout: std::time::Duration) {
        self.connected = self
            .last_heartbeat
            .is_some_and(|last| last.elapsed() < timeout);
    }
}
