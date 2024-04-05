use crate::smart_house::{SmartSocket, SmartSwitch, SmartThermometer};

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String>;
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
