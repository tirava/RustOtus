use crate::prelude::{
    DeviceStatus, SmartDeviceInfo, SmartHouseError, SmartHouseStorage, SmartHouseStorageMemory,
    SmartHouseStorageMongoDB,
};
use async_trait::async_trait;
use dashmap::DashMap;

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
        self.devices_info.insert("qqq".to_string(), DashMap::new());

        self.devices_info.get_mut("qqq").unwrap().insert(
            "111".to_string(),
            SmartDeviceInfo {
                name: "111".to_string(),
                status: DeviceStatus::On.to_string(),
                power: 111.11,
                temp: 11.1,
            },
        );

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

        let room = match self.devices_info.get(room) {
            Some(room) => room,
            None => {
                return Err(SmartHouseError::DeviceInfoProviderError(
                    SmartHouseError::RoomNotFoundError(room.to_string()).to_string(),
                ))
            }
        };

        let device = match room.get(device) {
            Some(device) => device,
            None => {
                return Err(SmartHouseError::DeviceInfoProviderError(
                    SmartHouseError::DeviceNotFoundError(
                        room.key().to_string(),
                        device.to_string(),
                    )
                    .to_string(),
                ))
            }
        };

        Ok(device.clone())
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
