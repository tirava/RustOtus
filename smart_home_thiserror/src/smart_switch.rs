use crate::smart_device::{AtomicDeviceStatus, DeviceStatus, SmartDevice};
use std::fmt;
use std::sync::atomic::Ordering::SeqCst;

pub struct SmartSwitch {
    pub(crate) name: String,
    pub(crate) room: String,
    pub status: AtomicDeviceStatus,
}

impl SmartSwitch {
    pub fn new(name: String, room: String, status: DeviceStatus) -> &'static Self {
        Box::leak(Box::new(Self {
            name,
            room,
            status: AtomicDeviceStatus::new(status),
        }))
    }
}

impl fmt::Display for SmartSwitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "статус - {}", self.status.load(SeqCst))
    }
}

impl SmartDevice for SmartSwitch {
    fn name(&self) -> &str {
        &self.name
    }

    fn exec_command(&self, command: &str) -> String {
        print!("SMART_SWITCH: command '{command}' -> ");

        match command {
            "on" => {
                self.status.store(DeviceStatus::On, SeqCst);
                "device is now ON".to_string()
            }
            "off" => {
                self.status.store(DeviceStatus::Off, SeqCst);
                "device is now OFF".to_string()
            }
            "info" => {
                format!(
                    "name: {}, room: {}, status: {}",
                    self.name,
                    self.room,
                    self.status.load(SeqCst),
                )
            }
            _ => "unknown command".to_string(),
        }
    }
}
