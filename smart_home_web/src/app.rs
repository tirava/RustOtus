use crate::prelude::SmartHouse;

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

    pub fn devices(&self, room: &str) -> Vec<&str> {
        self.house.devices(room).unwrap_or_default()
    }
}
