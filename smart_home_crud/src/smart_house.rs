use crate::device_info_provider::DeviceInfoProvider;
use std::collections::{HashMap, HashSet};
use std::{error, fmt, io};

pub struct SmartHouse {
    name: String,
    address: String,
    devices: HashMap<String, HashSet<String>>,
}

impl SmartHouse {
    pub fn r#use() {}

    pub fn new(name: String, address: String, devices: HashMap<&str, Vec<&str>>) -> Self {
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

    pub fn add_room(&mut self, room: &str) -> Result<Vec<&str>, SmartHouseError> {
        if self.devices.contains_key(room) {
            return Err(SmartHouseError::RoomAlreadyExistsError);
        }
        self.devices.insert(room.to_string(), HashSet::new());

        Ok(self.devices.keys().map(|s| s.as_str()).collect())
    }

    pub fn remove_room(&mut self, room: &str) -> Result<Option<Vec<&str>>, SmartHouseError> {
        if !self.devices.contains_key(room) {
            return Err(SmartHouseError::RoomNotFoundError);
        }
        self.devices.remove(room);

        match self.rooms() {
            Some(rooms) => Ok(Some(rooms)),
            None => Ok(None),
        }
    }

    pub fn devices(&self, room: &str) -> Option<Vec<&str>> {
        self.devices
            .get(room)
            .map(|devices| devices.iter().map(|s| s.as_str()).collect())
    }

    pub fn add_device(&mut self, room: &str, device: &str) -> Result<Vec<&str>, SmartHouseError> {
        if !self.devices.contains_key(room) {
            return Err(SmartHouseError::RoomNotFoundError);
        }
        let device_room = match self.devices.get_mut(room) {
            Some(device_room) => device_room,
            None => return Err(SmartHouseError::RoomNotFoundError),
        };
        device_room.insert(device.to_string());

        Ok(self.devices.keys().map(|s| s.as_str()).collect())
    }

    pub fn remove_device(
        &mut self,
        room: &str,
        device: &str,
    ) -> Result<Option<Vec<&str>>, SmartHouseError> {
        let device_room = match self.devices.get_mut(room) {
            Some(device_room) => device_room,
            None => return Err(SmartHouseError::RoomNotFoundError),
        };
        if !device_room.remove(device) {
            return Err(SmartHouseError::DeviceNotFoundError);
        }

        match self.devices(room) {
            Some(devices) => Ok(Some(devices)),
            None => Ok(None),
        }
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

pub enum SmartHouseError {
    RoomsNotFoundError,
    RoomNotFoundError,
    RoomAlreadyExistsError,
    DevicesNotFoundError,
    DeviceNotFoundError,
    DeviceAlreadyExistsError,
    IoError(io::Error),
}

impl fmt::Debug for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHouseError::RoomsNotFoundError => {
                write!(f, "rooms not found")
            }
            SmartHouseError::RoomNotFoundError => write!(f, "room not found"),
            SmartHouseError::RoomAlreadyExistsError => write!(f, "room already exists"),
            SmartHouseError::DevicesNotFoundError => write!(f, "devices not found"),
            SmartHouseError::DeviceNotFoundError => write!(f, "device not found"),
            SmartHouseError::DeviceAlreadyExistsError => write!(f, "device already exists"),
            SmartHouseError::IoError(err) => {
                write!(f, "i/o error: {}", err)
            }
        }
    }
}

impl fmt::Display for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHouseError::RoomsNotFoundError => {
                write!(f, "комнаты не найдены")
            }
            SmartHouseError::RoomNotFoundError => write!(f, "комната не найдена"),
            SmartHouseError::RoomAlreadyExistsError => write!(f, "комната уже существует"),
            SmartHouseError::DevicesNotFoundError => write!(f, "устройства не найдены"),
            SmartHouseError::DeviceNotFoundError => write!(f, "устройство не найдено"),
            SmartHouseError::DeviceAlreadyExistsError => write!(f, "устройство уже существует"),
            SmartHouseError::IoError(err) => {
                write!(f, "ошибка ввода-вывода: {}", err)
            }
        }
    }
}

impl error::Error for SmartHouseError {}

impl From<io::Error> for SmartHouseError {
    fn from(err: io::Error) -> Self {
        SmartHouseError::IoError(err)
    }
}
