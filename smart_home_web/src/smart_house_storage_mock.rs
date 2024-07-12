use crate::prelude::{
    SmartDeviceInfo, SmartHouseError, SmartHouseStorage, SmartHouseStorageMemory,
    SmartHouseStorageMongoDB,
};
use async_trait::async_trait;
use dashmap::DashMap;

#[async_trait]
pub trait MockDeviceInfoProvider: SmartHouseStorage {
    async fn init(
        &mut self,
        devices_info: DashMap<String, DashMap<String, SmartDeviceInfo>>,
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
        devices_info: DashMap<String, DashMap<String, SmartDeviceInfo>>,
    ) -> Result<(), SmartHouseError> {
        self.devices_info = devices_info;

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
    async fn init(
        &mut self,
        _devices_info: DashMap<String, DashMap<String, SmartDeviceInfo>>,
    ) -> Result<(), SmartHouseError> {
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
