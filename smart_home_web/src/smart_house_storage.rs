use crate::prelude::SmartHouseError;
use async_trait::async_trait;
use dashmap::{DashMap, DashSet};

pub mod prelude {
    pub use crate::smart_house_storage::{SmartHouseStorage, SmartHouseStorageMemory};
}

#[async_trait]
pub trait SmartHouseStorage {
    async fn rooms(&self) -> Result<Vec<String>, SmartHouseError>;

    async fn add_room(&self, room: &str) -> Result<(), SmartHouseError>;
}

pub struct SmartHouseStorageMemory {
    // name: String,
    // address: String,
    devices: DashMap<String, DashSet<String>>,
}

impl SmartHouseStorageMemory {
    pub fn new(_name: String, _address: String) -> Self {
        Self {
            // name,
            // address,
            devices: DashMap::new(),
        }
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
}
