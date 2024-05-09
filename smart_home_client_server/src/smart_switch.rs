use crate::smart_device::{DeviceStatus, SmartDevice};
use std::fmt;

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

impl SmartDevice for SmartSwitch {}
