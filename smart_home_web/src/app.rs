use crate::prelude::{SmartHouseError, SmartHouseStorage};

pub struct AppData {
    pub storage: Box<dyn SmartHouseStorage + Send + Sync>,
}

impl AppData {
    pub fn new(storage: Box<dyn SmartHouseStorage + Send + Sync>) -> Self {
        Self { storage }
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

    // pub async fn devices(&self, room: &str) -> Result<Vec<&str>, SmartHouseError> {
    //     match self.house.devices(room) {
    //         Some(devices) => Ok(devices),
    //         None => Err(SmartHouseError::RoomNotFoundError(room.to_string())),
    //     }
    // }
    //
    // pub async fn add_device(&mut self, room: &str, device: &str) -> Result<(), SmartHouseError> {
    //     self.house.add_device(room, device)
    // }
    //
    // pub async fn remove_device(&mut self, room: &str, device: &str) -> Result<(), SmartHouseError> {
    //     self.house.remove_device(room, device)
    // }

    // pub async fn device_info(
    //     &self,
    //     room: &str,
    //     device: &str,
    // ) -> Result<SmartDeviceInfo, SmartHouseError> {
    //     Ok(SmartDeviceInfo {
    //         name: format!("{} - {}", room, device),
    //         status: "Vkl".to_string(),
    //         power: 111.222,
    //         temp: 333.444,
    //     })
    // }
    //
    // pub async fn house_report(&self) -> Result<SmartHouseReport, SmartHouseError> {
    //     Ok(SmartHouseReport {
    //         name: "qqq".to_string(),
    //         address: "www".to_string(),
    //         devices: HashMap::new(),
    //     })
    // }
}
