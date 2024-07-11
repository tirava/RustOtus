use crate::prelude::{SmartHouse, SmartHouseError};

#[derive(Clone)]
pub struct AppData {
    pub house: SmartHouse,
}

impl AppData {
    pub fn new(house: SmartHouse) -> Self {
        Self { house }
    }

    pub async fn rooms(&self) -> Result<Vec<&str>, SmartHouseError> {
        Ok(self.house.rooms().unwrap_or_default())
    }

    pub async fn devices(&self, room: &str) -> Result<Vec<&str>, SmartHouseError> {
        match self.house.devices(room) {
            Some(devices) => Ok(devices),
            None => {
                return Err(SmartHouseError::RoomNotFoundError(room.to_string()));
            }
        }
    }

    pub async fn add_room(&mut self, room: &str) -> Result<(), SmartHouseError> {
        self.house.add_room(room)
    }

    pub async fn remove_room(&mut self, room: &str) -> Result<(), SmartHouseError> {
        self.house.remove_room(room)
    }

    pub async fn add_device(&mut self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        self.house.add_device(room, device)
    }

    pub async fn remove_device(&mut self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        self.house.remove_device(room, device)
    }
}
