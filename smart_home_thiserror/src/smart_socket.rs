use crate::smart_device::{AtomicDeviceStatus, DeviceStatus, SmartDevice};
use atomic_float::AtomicF32;
use rand::Rng;
use std::fmt;
use std::sync::atomic::Ordering::SeqCst;

pub struct SmartSocket {
    pub(crate) name: String,
    pub(crate) room: String,
    pub status: AtomicDeviceStatus,
    pub power: AtomicF32,
}

impl SmartSocket {
    pub fn new(name: String, room: String, status: DeviceStatus, power: f32) -> &'static Self {
        Box::leak(Box::new(Self {
            name,
            room,
            status: AtomicDeviceStatus::new(status),
            power: AtomicF32::new(power),
        }))
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "статус - {}, мощность {:.2} pW",
            self.status.load(SeqCst),
            self.power.load(SeqCst)
        )
    }
}

impl SmartDevice for SmartSocket {
    fn name(&self) -> &str {
        &self.name
    }

    fn exec_command(&self, command: &str) -> String {
        print!("SMART_SOCKET: command '{command}' -> ");

        match command {
            "on" => {
                self.status.store(DeviceStatus::On, SeqCst);
                self.power
                    .store(rand::thread_rng().gen_range(10.0..3000.0), SeqCst);
                "device is now ON".to_string()
            }
            "off" => {
                self.status.store(DeviceStatus::Off, SeqCst);
                self.power.store(0.0, SeqCst);
                "device is now OFF".to_string()
            }
            "power" => format!("{:.2}", self.power.load(SeqCst)),
            "info" => {
                format!(
                    "name: {}, room: {}, status: {}, power: {:.2} pW",
                    self.name,
                    self.room,
                    self.status.load(SeqCst),
                    self.power.load(SeqCst)
                )
            }
            _ => "unknown command".to_string(),
        }
    }
}
