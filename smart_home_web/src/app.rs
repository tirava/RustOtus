use crate::prelude::{DeviceStatus, SmartDeviceInfo, SmartHouseError, SmartHouseReport};
use crate::smart_house_storage::SmartHouseDeviceStorage;
use std::collections::HashMap;

const DEVICE_NOT_FOUND_IN_PROVIDER: &str = " (устройство не найдено в источнике информации)";
pub struct AppData {
    pub name: String,
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
        let mut rooms = self.storage.rooms().await?;
        rooms.sort();

        Ok(rooms)
    }

    pub async fn add_room(&self, room: &str) -> Result<(), SmartHouseError> {
        self.storage.add_room(room).await
    }

    pub async fn remove_room(&self, room: &str) -> Result<(), SmartHouseError> {
        self.storage.remove_room(room).await
    }

    pub async fn devices(&self, room: &str) -> Result<Vec<String>, SmartHouseError> {
        let mut devices = self.storage.devices(room).await?;
        devices.sort();

        Ok(devices)
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
        let info = self
            .storage
            .device_info(room, device)
            .await
            .unwrap_or_else(|_| SmartDeviceInfo {
                name: device.to_string() + DEVICE_NOT_FOUND_IN_PROVIDER,
                status: DeviceStatus::Unknown.to_string(),
                power: 0.0,
                temp: 0.0,
            });

        Ok(info)
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
