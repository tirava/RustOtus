// Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
// Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
// о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
// для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
// Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
// Привести пример типа, предоставляющего текстовую информацию об устройствах в доме для составления отчёта.

use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fmt;

pub struct SmartHome {
    name: String,
    address: String,
    devices: HashMap<String, HashSet<String>>, // <key: room, value: HashSet<device>
}

impl SmartHome {
    pub fn new(name: String, address: String, room_devices: HashMap<&str, HashSet<&str>>) -> Self {
        let devices = room_devices
            .into_iter()
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.insert(
                    k.to_string(),
                    v.into_iter().map(|s| s.to_string()).collect(),
                );
                acc
            });
        Self {
            name,
            address,
            devices,
        }
    }

    pub fn rooms(&self) -> Vec<&String> {
        self.devices.keys().collect()
    }

    pub fn devices(&self, room: &str) -> Vec<&String> {
        self.devices[room].iter().collect()
    }

    // fn create_report(
    //     &self,
    //     /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    // ) -> String {
    //     todo!("перебор комнат и устройств в них для составления отчёта")
    // }
}

pub trait Device {
    fn new(name: String, connect_url: String) -> Self;
    fn name(&self) -> &str;
    fn connect(&self) -> Result<(), SmartHomeError> {
        Ok(())
    }
}

pub trait Thermometer: Device {
    fn temperature(&self) -> Result<f64, SmartHomeError>;
}

pub trait Socket: Device {
    fn power(&self) -> Result<f64, SmartHomeError>;
    fn get_state(&self) -> Result<&DeviceState, SmartHomeError>;
    fn set_state(&mut self, state: DeviceState) -> Result<(), SmartHomeError>;
}

pub struct SmartDevice {
    name: String,
    connect_url: String,
}

impl Device for SmartDevice {
    fn new(name: String, connect_url: String) -> Self {
        Self { name, connect_url }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn connect(&self) -> Result<(), SmartHomeError> {
        let _ = self.connect_url;
        Ok(())
    }
}

pub struct SmartThermometer {
    device: SmartDevice,
    temperature: f64,
}

impl Device for SmartThermometer {
    fn new(name: String, connect_url: String) -> Self {
        Self {
            device: SmartDevice::new(name, connect_url),
            temperature: rand::thread_rng().gen_range(20.0..25.0),
        }
    }

    fn name(&self) -> &str {
        self.device.name()
    }

    fn connect(&self) -> Result<(), SmartHomeError> {
        self.device.connect()
    }
}

impl Thermometer for SmartThermometer {
    fn temperature(&self) -> Result<f64, SmartHomeError> {
        // in real will be request from device with error handling
        Ok(self.temperature)
    }
}

// pub struct SmartSocket {
//     pub device: BaseDevice,
//     state: DeviceState,
//     power: f64,
// }
//
// impl SmartSocket {
//     pub fn new(device: BaseDevice) -> SmartSocket {
//         SmartSocket {
//             device,
//             state: DeviceState::Unknown,
//             power: 0.0,
//         }
//     }
// }
//
// impl SmartDevice for SmartSocket {
//     fn get_name(&self) -> &str {
//         &self.device.name
//     }
//
//     fn power(&self) -> Result<f64, SmartHomeError> {
//         // in real will be request power from device
//         Ok(self.power)
//     }
//
//     fn get_state(&self) -> Result<&DeviceState, SmartHomeError> {
//         // in real will be request from device with error handling
//         Ok(&self.state)
//     }
//
//     fn set_state(&mut self, state: DeviceState) -> Result<(), SmartHomeError> {
//         // in real will be request to device with error handling
//         self.state = state;
//
//         // fake device logic - delete after we have real device
//         match self.state {
//             DeviceState::On => self.power = rand::thread_rng().gen_range(100.0..3500.0),
//             _ => self.power = 0.0,
//         }
//
//         Ok(())
//     }
// }

// trait DeviceInfoProvider {
//     // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
// }
//
// // ***** Пример использования библиотеки умный дом:
//
// // Пользовательские устройства:
// struct SmartSocket {}
// struct SmartThermometer {}
//
// // Пользовательские поставщики информации об устройствах.
// // Могут как хранить устройства, так и заимствывать.
// struct OwningDeviceInfoProvider {
//     socket: SmartSocket,
// }
// struct BorrowingDeviceInfoProvider<'a, 'b> {
//     socket: &'a SmartSocket,
//     thermo: &'b SmartThermometer,
// }
//
// // todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

pub enum SmartHomeError {
    ErrRoomNotFound { room_name: String },
    ErrDeviceNotFound { device_name: String },
    UnknownError,
}

impl fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeError::ErrRoomNotFound { room_name } => {
                write!(f, "помещение не найдено: {room_name}")
            }
            SmartHomeError::ErrDeviceNotFound { device_name } => {
                write!(f, "устройство не найдено: {device_name}")
            }
            SmartHomeError::UnknownError => write!(f, "неизвестная ошибка"),
        }
    }
}

impl fmt::Debug for SmartHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

pub enum DeviceState {
    Off,
    On,
    Unknown,
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceState::Off => write!(f, "Выключено"),
            DeviceState::On => write!(f, "Включено"),
            DeviceState::Unknown => write!(f, "Неизвестно"),
        }
    }
}
