use std::collections::{HashMap, HashSet};
use std::fmt;

pub mod prelude {
    pub use super::SmartHouse;
    pub use super::SmartSocket;
    pub use super::SmartSwitch;
    pub use super::SmartThermometer;
    pub use super::DeviceStatus;
    pub use super::OwningDeviceInfoProvider;
    pub use super::BorrowingDeviceInfoProvider;
}

pub struct SmartHouse {
    name: String,
    address: String,
    devices: HashMap<String, Vec<String>>,
}

impl SmartHouse {
    pub fn new(name: String, address: String, devices: HashMap<&str, Vec<&str>>) -> Self {
        Self {
            name,
            address,
            devices: devices
                .iter()
                .map(|(k, v)| {
                    (k.to_string(), {
                        // dedup
                        v.iter()
                            .collect::<HashSet<_>>()
                            .iter()
                            .map(|s| s.to_string())
                            .collect()
                    })
                })
                .collect(),
        }
    }

    pub fn rooms(&self) -> Vec<&String> {
        self.devices.keys().collect()
    }

    pub fn devices(&self, room: &str) -> Vec<&str> {
        match self.devices.get(room) {
            Some(devices) => devices.iter().map(|s| s.as_str()).collect(),
            None => Vec::new(),
        }
    }

    pub fn create_report(&self, info_provider: &impl DeviceInfoProvider) -> String {
        let mut report = format!(
            "\n {:13}: {}\n {:13}: {}\n\n",
            "Дом", self.name, "Адрес", self.address
        );

        let mut rooms = self.rooms();
        rooms.sort();

        for room in rooms {
            report += format!(" {:13}: {}\n {:13}:\n", "Комната", room, "Устройства").as_str();
            let mut devices = self.devices(room);
            devices.sort();

            for device in devices {
                let info = info_provider
                    .get_device_info(room, device)
                    .unwrap_or_else(|| {
                        "в источнике информации это устроство не обнаружено".to_string()
                    });

                report +=
                    format!("              : {}, {}: {}\n", device, "состояние", info).as_str();
            }
            report += "\n";
        }

        report
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String>;
}

pub struct SmartSocket {
    name: String,
    room: String,
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

pub struct SmartThermometer {
    name: String,
    room: String,
    pub temp: f32,
}

impl SmartThermometer {
    pub fn new(name: String, room: String, temp: f32) -> Self {
        Self { name, room, temp }
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "температура - {:.2} °С", self.temp)
    }
}

pub struct SmartSwitch {
    name: String,
    room: String,
    pub status: DeviceStatus,
}

impl SmartSwitch {
    pub fn new(name: String, room: String, status: DeviceStatus) -> Self {
        Self { name, room, status }
    }
}

impl fmt::Display for SmartSwitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "статус - {}", self.status)
    }
}

pub struct OwningDeviceInfoProvider {
    pub sockets: Vec<SmartSocket>,
}

pub struct BorrowingDeviceInfoProvider<'a> {
    pub thermometers: &'a Vec<SmartThermometer>,
    pub switches: &'a Vec<SmartSwitch>,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String> {
        self.sockets
            .iter()
            .find(|s| s.name == device && s.room == room)?
            .to_string()
            .into()
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_> {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String> {
        if let Some(thermometer) = self
            .thermometers
            .iter()
            .find(|s| s.name == device && s.room == room)
        {
            return thermometer.to_string().into();
        } else if let Some(switch) = self
            .switches
            .iter()
            .find(|s| s.name == device && s.room == room)
        {
            return switch.to_string().into();
        }

        None
    }
}

pub enum DeviceStatus {
    Off,
    On,
    Unknown,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceStatus::Off => write!(f, "выключено"),
            DeviceStatus::On => write!(f, "включено"),
            DeviceStatus::Unknown => write!(f, "неизвестно"),
        }
    }
}
