use crate::smart_device::{DeviceStatus, SmartDevice};
use std::{fmt, io};

pub struct SmartSocket {
    pub(crate) name: String,
    pub(crate) room: String,
    pub status: DeviceStatus,
    pub power: f32,
}

impl SmartSocket {
    pub fn new(name: String, room: String, status: DeviceStatus, power: f32) -> Self {
        Self {
            name,
            room,
            status,
            power,
        }
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "статус - {}, мощность {:.2} pW", self.status, self.power)
    }
}

impl SmartDevice for SmartSocket {
    fn exec_command(&self, command: &str) -> Result<String, io::Error> {
        println!("Command: {command:#?}");

        let result = match command {
            "info" => format!(
                "name: {}, room: {}, status: {}, power: {:.2} pW",
                self.name, self.room, self.status, self.power
            ),
            _ => "command no found".to_string(),
        };
        let result = format!("Result: {result:#?}\r\n");

        Ok(result)
    }
}
