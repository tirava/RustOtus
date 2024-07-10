use crate::prelude::{SmartHouse, SmartHouseError};

#[derive(Clone)]
pub struct AppData {
    pub house: SmartHouse,
}

impl AppData {
    pub fn new(house: SmartHouse) -> Self {
        Self { house }
    }

    pub fn rooms(&self) -> Vec<&str> {
        self.house.rooms().unwrap_or_default()
    }

    pub fn devices(&self, room: &str) -> Option<Vec<&str>> {
        self.house.devices(room)
    }

    pub fn add_room(&mut self, room: &str) -> Result<(), SmartHouseError> {
        self.house.add_room(room)
    }

    pub fn remove_room(&mut self, room: &str) -> Result<(), SmartHouseError> {
        self.house.remove_room(room)
    }
}
