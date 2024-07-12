use crate::prelude::{SmartDeviceInfo, SmartHouseError, SmartHouseReport};
use crate::smart_house_storage::SmartHouseDeviceStorage;
use std::collections::HashMap;

pub struct AppData {
    name: String,
    address: String,
    pub storage: Box<dyn SmartHouseDeviceStorage + Send + Sync>,
}

impl AppData {
    pub fn new(
        name: String,
        address: String,
        storage: Box<dyn SmartHouseDeviceStorage + Send + Sync>,
    ) -> Self {
        Self {
            name,
            address,
            storage,
        }
    }

    pub async fn rooms(&self) -> Result<Vec<String>, SmartHouseError> {
        self.storage.rooms().await
    }

    pub async fn add_room(&self, room: &str) -> Result<(), SmartHouseError> {
        self.storage.add_room(room).await
    }

    pub async fn remove_room(&self, room: &str) -> Result<(), SmartHouseError> {
        self.storage.remove_room(room).await
    }

    pub async fn devices(&self, room: &str) -> Result<Vec<String>, SmartHouseError> {
        self.storage.devices(room).await
    }

    pub async fn add_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        self.storage.add_device(room, device).await
    }

    pub async fn remove_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        self.storage.remove_device(room, device).await
    }

    pub async fn device_info(
        &self,
        room: &str,
        device: &str,
    ) -> Result<SmartDeviceInfo, SmartHouseError> {
        self.storage.device_info(room, device).await
    }

    pub async fn house_report(&self) -> Result<SmartHouseReport, SmartHouseError> {
        let rooms = self.rooms().await?;
        let mut devices_info: HashMap<String, Vec<SmartDeviceInfo>> = HashMap::new();

        for room in rooms {
            let devices = self.devices(&room).await?;
            for device in devices {
                let info = self.device_info(&room, &device).await?;
                devices_info.entry(room.clone()).or_default().push(info);
            }
        }

        let report = SmartHouseReport {
            name: self.name.clone(),
            address: self.address.clone(),
            devices: devices_info,
        };

        Ok(report)
    }
}
