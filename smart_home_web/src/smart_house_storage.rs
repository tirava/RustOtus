use crate::prelude::{SmartDeviceInfo, SmartHouseError};
use async_trait::async_trait;
use dashmap::{DashMap, DashSet};

pub mod prelude {
    pub use crate::smart_house_storage::{
        SmartHouseStorage, SmartHouseStorageMemory, SmartHouseStorageMongoDB,
    };
}

#[async_trait]
pub trait SmartHouseStorage {
    async fn rooms(&self) -> Result<Vec<String>, SmartHouseError>;

    async fn add_room(&self, room: &str) -> Result<(), SmartHouseError>;

    async fn remove_room(&self, room: &str) -> Result<(), SmartHouseError>;

    async fn devices(&self, room: &str) -> Result<Vec<String>, SmartHouseError>;

    async fn add_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError>;

    async fn remove_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError>;
}

pub struct SmartHouseStorageMemory {
    devices: DashMap<String, DashSet<String>>,
}

impl SmartHouseStorageMemory {
    pub fn new() -> Self {
        Self {
            devices: DashMap::new(),
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

pub struct SmartHouseStorageMongoDB {
    _uri: String,
}

impl SmartHouseStorageMongoDB {
    pub fn new(_uri: String) -> Self {
        Self { _uri }
    }

    pub async fn connect(self) -> Result<Self, SmartHouseError> {
        Ok(self)
    }
}

#[async_trait]
impl SmartHouseStorage for SmartHouseStorageMongoDB {
    async fn rooms(&self) -> Result<Vec<String>, SmartHouseError> {
        todo!()
    }

    async fn add_room(&self, _room: &str) -> Result<(), SmartHouseError> {
        todo!()
    }

    async fn remove_room(&self, _room: &str) -> Result<(), SmartHouseError> {
        todo!()
    }

    async fn devices(&self, _room: &str) -> Result<Vec<String>, SmartHouseError> {
        todo!()
    }

    async fn add_device(&self, _room: &str, _device: &str) -> Result<(), SmartHouseError> {
        todo!()
    }

    async fn remove_device(&self, _room: &str, _device: &str) -> Result<(), SmartHouseError> {
        todo!()
    }
}

#[async_trait]
pub trait MockDeviceInfoProvider: SmartHouseStorage {
    async fn init(&self) -> Result<(), SmartHouseError>;

    async fn device_info(
        &self,
        room: &str,
        device: &str,
    ) -> Result<SmartDeviceInfo, SmartHouseError>;
}

#[async_trait]
impl MockDeviceInfoProvider for SmartHouseStorageMemory {
    async fn init(&self) -> Result<(), SmartHouseError> {
        todo!()
    }

    async fn device_info(
        &self,
        _room: &str,
        _device: &str,
    ) -> Result<SmartDeviceInfo, SmartHouseError> {
        todo!()
    }
}

#[async_trait]
impl MockDeviceInfoProvider for SmartHouseStorageMongoDB {
    async fn init(&self) -> Result<(), SmartHouseError> {
        todo!()
    }

    async fn device_info(
        &self,
        _room: &str,
        _device: &str,
    ) -> Result<SmartDeviceInfo, SmartHouseError> {
        todo!()
    }
}

#[async_trait]
pub trait SmartHouseDeviceStorage: SmartHouseStorage + MockDeviceInfoProvider {}

#[async_trait]
impl SmartHouseDeviceStorage for SmartHouseStorageMemory {}

#[async_trait]
impl SmartHouseDeviceStorage for SmartHouseStorageMongoDB {}