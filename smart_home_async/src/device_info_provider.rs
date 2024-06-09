use crate::smart_device::prelude::*;

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String>;
}

pub struct OwningDeviceInfoProvider {
    pub sockets: Vec<&'static SmartSocket>,
}

pub struct BorrowingDeviceInfoProvider<'a> {
    pub thermometers: &'a Vec<&'static SmartThermometer>,
    pub switches: &'a Vec<&'static SmartSwitch>,
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
