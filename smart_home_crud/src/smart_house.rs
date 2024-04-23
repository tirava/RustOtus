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

pub enum SmartHouseError {
    RoomsNotFoundError,
    RoomNotFoundError(String),
    RoomAlreadyExistsError(String),
    DevicesNotFoundError,
    DeviceNotFoundError(String, String),
    DeviceAlreadyExistsError(String, String),
    IoError(io::Error),
}

impl fmt::Debug for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHouseError::RoomsNotFoundError => {
                write!(f, "rooms not found")
            }
            SmartHouseError::RoomNotFoundError(s) => write!(f, "room not found: {}", s),
            SmartHouseError::RoomAlreadyExistsError(s) => write!(f, "room already exists: {}", s),
            SmartHouseError::DevicesNotFoundError => write!(f, "devices not found"),
            SmartHouseError::DeviceNotFoundError(room, device) => {
                write!(f, "device '{}' not found in room '{}'", device, room)
            }
            SmartHouseError::DeviceAlreadyExistsError(room, device) => {
                write!(f, "device '{}' already exists in room '{}'", device, room)
            }
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
            SmartHouseError::RoomNotFoundError(s) => write!(f, "комната не найдена: {}", s),
            SmartHouseError::RoomAlreadyExistsError(s) => {
                write!(f, "комната уже существует: {}", s)
            }
            SmartHouseError::DevicesNotFoundError => write!(f, "устройства не найдены"),
            SmartHouseError::DeviceNotFoundError(room, device) => write!(
                f,
                "устройство '{}' не найдено в комнате '{}' ",
                device, room
            ),
            SmartHouseError::DeviceAlreadyExistsError(room, device) => {
                write!(
                    f,
                    "устройство '{}' уже существует в комнате '{}'",
                    device, room
                )
            }
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
