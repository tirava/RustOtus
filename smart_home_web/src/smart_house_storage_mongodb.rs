use crate::prelude::{SmartDeviceInfo, SmartHouseError, SmartHouseStorage};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};

pub struct SmartHouseStorageMongoDB {
    pub(crate) collection_rooms: Collection<CollectionRoom>,
    pub(crate) collection_devices: Collection<CollectionDevice>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CollectionRoom {
    pub(crate) name: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct CollectionDevice {
    pub(crate) room_name: String,
    pub(crate) device: SmartDeviceInfo,
}

impl SmartHouseStorageMongoDB {
    pub async fn new(uri: &str) -> Result<Self, SmartHouseError> {
        let client = Client::with_uri_str(uri).await?;
        let db = match client.default_database() {
            Some(db) => db,
            None => {
                return Err(SmartHouseError::OtherError(
                    "no default database found in uri string".to_string(),
                ))
            }
        };

        Ok(Self {
            collection_rooms: db.collection("rooms"),
            collection_devices: db.collection("devices"),
        })
    }
}

#[async_trait]
impl SmartHouseStorage for SmartHouseStorageMongoDB {
    async fn rooms(&self) -> Result<Vec<String>, SmartHouseError> {
        let cursor = self.collection_rooms.find(doc! {}).await?;

        let rooms = cursor
            .try_collect::<Vec<CollectionRoom>>()
            .await?
            .into_iter()
            .map(|room| room.name)
            .collect();

        Ok(rooms)
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
