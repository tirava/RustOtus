use crate::http_handler::SmartDeviceInfo;
use crate::prelude::{SmartHouseError, SmartHouseStorage};
use async_trait::async_trait;
use dashmap::{DashMap, DashSet};
use std::collections::HashMap;

pub struct SmartHouseStorageMemory {
    pub(crate) devices: DashMap<String, DashSet<String>>,
    pub(crate) devices_info: HashMap<&'static str, HashMap<&'static str, SmartDeviceInfo>>,
}

impl SmartHouseStorageMemory {
    pub fn new() -> Self {
        Self {
            devices: DashMap::new(),
            devices_info: HashMap::new(),
        }
    }
}

impl Default for SmartHouseStorageMemory {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SmartHouseStorage for SmartHouseStorageMemory {
    async fn rooms(&self) -> Result<Vec<String>, SmartHouseError> {
        let rooms = self.devices.iter().map(|s| s.key().clone()).collect();

        Ok(rooms)
    }

    async fn add_room(&self, room: &str) -> Result<(), SmartHouseError> {
        if self.devices.contains_key(room) {
            return Err(SmartHouseError::RoomAlreadyExistsError(room.to_string()));
        }

        self.devices.insert(room.to_string(), DashSet::new());

        Ok(())
    }

    async fn remove_room(&self, room: &str) -> Result<(), SmartHouseError> {
        match self.devices.contains_key(room) {
            true => self.devices.remove(room),
            false => return Err(SmartHouseError::RoomNotFoundError(room.to_string())),
        };

        Ok(())
    }

    async fn devices(&self, room: &str) -> Result<Vec<String>, SmartHouseError> {
        match self.devices.get(room) {
            Some(devices) => Ok(devices.iter().map(|s| s.to_string()).collect()),
            None => Err(SmartHouseError::RoomNotFoundError(room.to_string())),
        }
    }

    async fn add_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        let device_room = match self.devices.get_mut(room) {
            Some(device_room) => device_room,
            None => return Err(SmartHouseError::RoomNotFoundError(room.to_string())),
        };

        if !device_room.insert(device.to_string()) {
            return Err(SmartHouseError::DeviceAlreadyExistsError(
                room.to_string(),
                device.to_string(),
            ));
        }

        Ok(())
    }

    async fn remove_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        let device_room = match self.devices.get_mut(room) {
            Some(device_room) => device_room,
            None => return Err(SmartHouseError::RoomNotFoundError(room.to_string())),
        };

        if device_room.remove(device).is_none() {
            return Err(SmartHouseError::DeviceNotFoundError(
                room.to_string(),
                device.to_string(),
            ));
        }

        Ok(())
    }
}
