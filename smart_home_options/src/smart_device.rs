use std::fmt;

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

pub struct SmartThermometer {
    pub(crate) name: String,
    pub(crate) room: String,
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
    pub(crate) name: String,
    pub(crate) room: String,
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
