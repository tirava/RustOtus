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

    async fn add_room(&self, room: &str) -> Result<(), SmartHouseError> {
        if self
            .collection_rooms
            .count_documents(doc! {"name": room})
            .await?
            > 0
        {
            return Err(SmartHouseError::RoomAlreadyExistsError(room.to_string()));
        }

        self.collection_rooms
            .insert_one(CollectionRoom {
                name: room.to_string(),
            })
            .await?;

        Ok(())
    }

    async fn remove_room(&self, room: &str) -> Result<(), SmartHouseError> {
        if self
            .collection_rooms
            .count_documents(doc! {"name": room})
            .await?
            == 0
        {
            return Err(SmartHouseError::RoomNotFoundError(room.to_string()));
        }

        self.collection_rooms
            .delete_one(doc! {"name": room})
            .await?;

        Ok(())
    }

    async fn devices(&self, room: &str) -> Result<Vec<String>, SmartHouseError> {
        if self
            .collection_rooms
            .count_documents(doc! {"name": room})
            .await?
            == 0
        {
            return Err(SmartHouseError::RoomNotFoundError(room.to_string()));
        }

        let cursor = self
            .collection_devices
            .find(doc! {"room_name": room})
            .await?;

        let devices = cursor
            .try_collect::<Vec<CollectionDevice>>()
            .await?
            .into_iter()
            .map(|device| device.device.name)
            .collect();

        Ok(devices)
    }

    async fn add_device(&self, _room: &str, _device: &str) -> Result<(), SmartHouseError> {
        todo!()
    }

    async fn remove_device(&self, _room: &str, _device: &str) -> Result<(), SmartHouseError> {
        todo!()
    }
}
