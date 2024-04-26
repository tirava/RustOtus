use std::fmt;

pub mod prelude {
    pub use crate::smart_device::DeviceStatus;
    pub use crate::smart_socket::SmartSocket;
    pub use crate::smart_switch::SmartSwitch;
    pub use crate::smart_thermometer::SmartThermometer;
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
