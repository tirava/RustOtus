use crate::prelude::{
    DeviceStatus, SmartDeviceInfo, SmartHouseError, SmartHouseStorage, SmartHouseStorageMemory,
    SmartHouseStorageMongoDB,
};
use crate::smart_house_storage_mongodb::{CollectionDevice, CollectionRoom};
use async_trait::async_trait;
use dashmap::{DashMap, DashSet};
use mongodb::bson::doc;
use std::collections::HashMap;

#[async_trait]
pub trait MockDeviceInfoProvider: SmartHouseStorage {
    async fn init(
        &mut self,
        devices_info: HashMap<&'static str, HashMap<&'static str, SmartDeviceInfo>>,
    ) -> Result<(), SmartHouseError>;

    async fn device_info(
        &self,
        room: &str,
        device: &str,
    ) -> Result<SmartDeviceInfo, SmartHouseError>;
}

#[async_trait]
impl MockDeviceInfoProvider for SmartHouseStorageMemory {
    async fn init(
        &mut self,
        devices_info: HashMap<&'static str, HashMap<&'static str, SmartDeviceInfo>>,
    ) -> Result<(), SmartHouseError> {
        self.devices_info = devices_info;

        self.devices = DashMap::new();
        for (room, devices) in self.devices_info.iter() {
            self.devices.insert(room.to_string(), DashSet::new());
            for device in devices.keys() {
                self.devices
                    .get_mut(*room)
                    .unwrap()
                    .insert(device.to_string());
            }
        }

        Ok(())
    }

    async fn device_info(
        &self,
        room: &str,
        device: &str,
    ) -> Result<SmartDeviceInfo, SmartHouseError> {
        if !match self.devices.get(room) {
            Some(room) => room,
            None => return Err(SmartHouseError::RoomNotFoundError(room.to_string())),
        }
        .contains(device)
        {
            return Err(SmartHouseError::DeviceNotFoundError(
                room.to_string(),
                device.to_string(),
            ));
        };

        let room_device = match self.devices_info.get(room) {
            Some(room_device) => room_device,
            None => {
                return Err(SmartHouseError::DeviceInfoProviderError(
                    SmartHouseError::RoomNotFoundError(room.to_string()).to_string(),
                ))
            }
        };

        let device_info = match room_device.get(device) {
            Some(device_info) => device_info,
            None => {
                return Err(SmartHouseError::DeviceInfoProviderError(
                    SmartHouseError::DeviceNotFoundError(room.to_string(), device.to_string())
                        .to_string(),
                ))
            }
        };

        Ok(device_info.clone())
    }
}

#[async_trait]
impl MockDeviceInfoProvider for SmartHouseStorageMongoDB {
    async fn init(
        &mut self,
        devices_info: HashMap<&'static str, HashMap<&'static str, SmartDeviceInfo>>,
    ) -> Result<(), SmartHouseError> {
        if self.collection_rooms.count_documents(doc! {}).await? > 0
            || self.collection_devices.count_documents(doc! {}).await? > 0
        {
            return Ok(());
        }

        let rooms: Vec<CollectionRoom> = devices_info
            .keys()
            .map(|room| CollectionRoom {
                name: room.to_string(),
            })
            .collect();
        self.collection_rooms.insert_many(rooms).await?;

        let devices: Vec<CollectionDevice> = devices_info
            .iter()
            .flat_map(|(room, devices)| {
                devices
                    .iter()
                    .map(move |(_, device_info)| CollectionDevice {
                        room_name: room.to_string(),
                        device: device_info.clone(),
                    })
            })
            .collect();
        self.collection_devices.insert_many(devices).await?;

        Ok(())
    }

    async fn device_info(
        &self,
        _room: &str,
        _device: &str,
    ) -> Result<SmartDeviceInfo, SmartHouseError> {
        Ok(SmartDeviceInfo {
            name: "111".to_string(),
            status: DeviceStatus::On.to_string(),
            power: 111.222,
            temp: 333.444,
        })
    }
}
