use crate::prelude::SmartHouseError;
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
        if !self.devices.contains_key(room) {
            return Err(SmartHouseError::RoomNotFoundError(room.to_string()));
        }

        self.devices.remove(room);

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
}
