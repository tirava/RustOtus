use crate::smart_device::SmartDevice;
use std::fmt;

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

impl SmartDevice for SmartThermometer {}
