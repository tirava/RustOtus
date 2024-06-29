use crate::device_info_provider::DeviceInfoProvider;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub struct SmartHouse {
    name: String,
    address: String,
    devices: HashMap<String, HashSet<String>>,
}

impl SmartHouse {
    pub fn r#use() {}

    pub fn new(name: String, address: String, devices: HashMap<&str, &[&str]>) -> Self {
        Self {
            name,
            address,
            devices: devices
                .iter()
                .map(|(k, v)| (k.to_string(), v.iter().map(|s| s.to_string()).collect()))
                .collect(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn rooms(&self) -> Option<Vec<&str>> {
        let rooms: Vec<_> = self.devices.keys().map(|s| s.as_str()).collect();
        match rooms.is_empty() {
            false => Some(rooms),
            true => None,
        }
    }

    pub fn add_room(&mut self, room: &str) -> Result<(), SmartHouseError> {
        if self.devices.contains_key(room) {
            return Err(SmartHouseError::RoomAlreadyExistsError(room.to_string()));
        }

        self.devices.insert(room.to_string(), HashSet::new());

        Ok(())
    }

    pub fn remove_room(&mut self, room: &str) -> Result<(), SmartHouseError> {
        if !self.devices.contains_key(room) {
            return Err(SmartHouseError::RoomNotFoundError(room.to_string()));
        }

        self.devices.remove(room);

        Ok(())
    }

    pub fn devices(&self, room: &str) -> Option<Vec<&str>> {
        self.devices
            .get(room)
            .map(|devices| devices.iter().map(|s| s.as_str()).collect())
    }

    pub fn add_device(&mut self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        let device_room = match self.devices.get_mut(room) {
            Some(device_room) => device_room,
            None => return Err(SmartHouseError::RoomNotFoundError(room.to_string())),
        };

        if !device_room.insert(device.to_string()) {
            return Err(SmartHouseError::DeviceAlreadyExistsError(
                room.to_string(),
                device.to_string(),
            ));
        }

        Ok(())
    }

    pub fn remove_device(&mut self, room: &str, device: &str) -> Result<(), SmartHouseError> {
        let device_room = match self.devices.get_mut(room) {
            Some(device_room) => device_room,
            None => return Err(SmartHouseError::RoomNotFoundError(room.to_string())),
        };

        if !device_room.remove(device) {
            return Err(SmartHouseError::DeviceNotFoundError(
                room.to_string(),
                device.to_string(),
            ));
        }

        Ok(())
    }

    pub fn create_report(
        &self,
        info_provider: &impl DeviceInfoProvider,
    ) -> Result<String, SmartHouseError> {
        let mut report = format!(
            "\n {:13}: {}\n {:13}: {}\n\n",
            "Дом", self.name, "Адрес", self.address
        );

        let mut rooms = match self.rooms() {
            Some(rooms) => rooms,
            None => return Err(SmartHouseError::RoomsNotFoundError),
        };
        rooms.sort();

        for room in rooms {
            report += format!(" {:13}: {}\n {:13}:\n", "Комната", room, "Устройства").as_str();
            let mut devices = match self.devices(room) {
                Some(devices) => devices,
                None => return Err(SmartHouseError::DevicesNotFoundError),
            };
            devices.sort();

            for device in devices {
                let info = info_provider
                    .get_device_info(room, device)
                    .unwrap_or_else(|| {
                        "в источнике информации это устроство не обнаружено".to_string()
                    });

                report +=
                    format!("              : {}, {}: {}\n", device, "состояние", info).as_str();
            }
            report += "\n";
        }

        Ok(report)
    }
}

#[derive(Debug, Error)]
pub enum SmartHouseError {
    #[error("комнаты не найдены")]
    RoomsNotFoundError,
    #[error("комната не найдена: {0}")]
    RoomNotFoundError(String),
    #[error("комната уже существует: {0}")]
    RoomAlreadyExistsError(String),
    #[error("устройства не найдены")]
    DevicesNotFoundError,
    #[error("устройство '{0}' не найдено в комнате '{1}' ")]
    DeviceNotFoundError(String, String),
    #[error("устройство '{0}' уже существует в комнате '{1}' ")]
    DeviceAlreadyExistsError(String, String),
    #[error("ошибка ввода-вывода: {0}")]
    IoError(#[from] std::io::Error),
}
