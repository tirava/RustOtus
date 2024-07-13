use crate::prelude::{SmartHouseError, SmartHouseStorage};
use async_trait::async_trait;
use mongodb::Client;

pub struct SmartHouseStorageMongoDB {
    pub(crate) client: Client,
    db_name: String,
}

impl SmartHouseStorageMongoDB {
    pub async fn new(uri: &str, db_name: String) -> Result<Self, SmartHouseError> {
        let client = Client::with_uri_str(uri).await?;

        Ok(Self { client, db_name })
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
