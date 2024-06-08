use crate::smart_device::{DeviceStatus, SmartDevice};
// use rand::Rng;
use std::fmt;

pub struct SmartSocket {
    pub(crate) name: String,
    pub(crate) room: String,
    pub status: DeviceStatus,
    pub power: f32,
}

impl SmartSocket {
    pub fn new(name: String, room: String, status: DeviceStatus, power: f32) -> &'static Self {
        Box::leak(Box::new(Self {
            name,
            room,
            status,
            power,
        }))
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "статус - {}, мощность {:.2} pW", self.status, self.power)
    }
}

impl SmartDevice for SmartSocket {
    fn exec_command(&self, command: &str) -> String {
        print!("SMART_SOCKET: command '{command}' -> ");

        match command {
            "on" => {
                // self.status = DeviceStatus::On;
                // self.power = rand::thread_rng().gen_range(10.0..3000.0);
                "device is now ON".to_string()
            }
            "off" => {
                // self.status = DeviceStatus::Off;
                // self.power = 0.0;
                "device is now OFF".to_string()
            }
            "power" => format!("{:.2}", self.power),
            "info" => {
                format!(
                    "name: {}, room: {}, status: {}, power: {:.2} pW",
                    self.name, self.room, self.status, self.power
                )
            }
            _ => "unknown command".to_string(),
        }
    }
}
