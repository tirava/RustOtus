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

    pub fn create_report(
        &self,
        info_provider: Box<dyn DeviceInfoProvider>,
    ) -> Result<String, SmartHomeError> {
        // todo!("перебор комнат и устройств в них для составления отчёта")
        for room in self.rooms() {
            for device in self.devices(room) {
                let info = info_provider.state(room, device)?;
                println!(
                    " name: {}\n address: {}\n room: {}\n device: {}\n info: {}",
                    self.name, self.address, room, device, info
                );
            }
        }
        Ok(String::from("OK"))
    }
}

pub trait Device {
    fn new(name: String, connect_url: Option<String>) -> Self;
    fn name(&self) -> &str;
    fn connect(&self) -> Result<(), SmartHomeError> {
        Ok(())
    }
}

pub struct SmartDevice {
    name: String,
    connect_url: String,
}

impl Device for SmartDevice {
    fn new(name: String, connect_url: Option<String>) -> Self {
        Self {
            name,
            connect_url: connect_url.unwrap_or_default(),
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn connect(&self) -> Result<(), SmartHomeError> {
        let _ = self.connect_url;
        Ok(())
    }
}

pub trait Thermometer: Device {
    fn temperature(&self) -> Result<f64, SmartHomeError>;
}

pub struct SmartThermometer {
    device: SmartDevice,
    temperature: f64,
}

impl Device for SmartThermometer {
    fn new(name: String, connect_url: Option<String>) -> Self {
        Self {
            device: SmartDevice::new(name, connect_url),
            // fake device logic - delete after we have real device
            temperature: rand::thread_rng().gen_range(20.0..25.0),
        }
    }

    fn name(&self) -> &str {
        self.device.name()
    }
}

impl Thermometer for SmartThermometer {
    fn temperature(&self) -> Result<f64, SmartHomeError> {
        // in real will be request from device with error handling
        Ok(self.temperature)
    }
}

pub trait Socket: Device {
    fn power(&self) -> Result<f64, SmartHomeError>;
    fn get_state(&self) -> Result<&DeviceState, SmartHomeError>;
    fn set_state(&mut self, state: DeviceState) -> Result<(), SmartHomeError>;
}

pub struct SmartSocket {
    device: SmartDevice,
    state: DeviceState,
    power: f64,
}

impl Device for SmartSocket {
    fn new(name: String, connect_url: Option<String>) -> Self {
        Self {
            device: SmartDevice::new(name, connect_url),
            state: DeviceState::Unknown,
            power: 0.0,
        }
    }

    fn name(&self) -> &str {
        self.device.name()
    }
}

impl Socket for SmartSocket {
    fn power(&self) -> Result<f64, SmartHomeError> {
        // in real will be request power from device with error handling
        Ok(self.power)
    }

    fn get_state(&self) -> Result<&DeviceState, SmartHomeError> {
        // in real will be request from device with error handling
        Ok(&self.state)
    }

    fn set_state(&mut self, state: DeviceState) -> Result<(), SmartHomeError> {
        // in real will be request to device with error handling
        self.state = state;

        // fake device logic - delete after we have real device
        match self.state {
            DeviceState::On => self.power = rand::thread_rng().gen_range(100.0..3500.0),
            _ => self.power = 0.0,
        }

        Ok(())
    }
}

pub trait DeviceInfoProvider {
    fn state(&self, room_name: &str, device_name: &str) -> Result<DeviceInfo, SmartHomeError>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermometer: &'b SmartThermometer,
}

pub enum DeviceInfo {
    OwningDeviceInfoProvider(OwningDeviceInfoProvider),
    BorrowingDeviceInfoProvider(BorrowingDeviceInfoProvider<'static, 'static>),
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn state(&self, room_name: &str, device_name: &str) -> Result<DeviceInfo, SmartHomeError> {
        // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
        todo!()
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn state(&self, room_name: &str, device_name: &str) -> Result<DeviceInfo, SmartHomeError> {
        // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
        todo!()
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceInfo::OwningDeviceInfoProvider(info) => {
                write!(f, "111")
            }
            DeviceInfo::BorrowingDeviceInfoProvider(info) => {
                write!(f, "222")
            }
        }
    }
}

pub enum SmartHomeError {
    ErrDeviceNotFound { device_name: String },
    UnknownError,
}

impl fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHomeError::ErrDeviceNotFound { device_name } => {
                write!(f, "устройство не найдено: {device_name}")
            }
            SmartHomeError::UnknownError => write!(f, "неизвестная ошибка"),
        }
    }
}

impl fmt::Debug for SmartHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
