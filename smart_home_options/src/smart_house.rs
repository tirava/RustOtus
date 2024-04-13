use crate::device_info_provider::DeviceInfoProvider;
use std::collections::{HashMap, HashSet};
use std::fmt;

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

    pub fn rooms(&self) -> Vec<&str> {
        self.devices.keys().map(|s| s.as_str()).collect()
    }

    pub fn devices(&self, room: &str) -> Vec<&str> {
        match self.devices.get(room) {
            Some(devices) => devices.iter().map(|s| s.as_str()).collect(),
            None => Vec::new(),
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

        let mut rooms = self.rooms();
        rooms.sort();

        for room in rooms {
            report += format!(" {:13}: {}\n {:13}:\n", "Комната", room, "Устройства").as_str();
            let mut devices = self.devices(room);
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
    ErrRoomsNotFound,
    ErrDevicesNotFound,
    ErrUnknown,
}

impl fmt::Debug for SmartHouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHouseError::ErrRoomsNotFound => {
                write!(f, "комнаты не найдены")
            }
            SmartHouseError::ErrDevicesNotFound => write!(f, "устройства не найдены"),
            SmartHouseError::ErrUnknown => write!(f, "неизвестная ошибка"),
        }
    }
}
