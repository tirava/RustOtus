use crate::prelude::{SmartHouseError, SmartHouseStorage};
use async_trait::async_trait;

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
