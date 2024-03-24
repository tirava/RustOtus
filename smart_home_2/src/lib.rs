use rand::Rng;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

pub const KITCHEN: &str = "Кухня";
pub const LIVING_ROOM: &str = "Гостинная";
pub const BEDROOM: &str = "Спальня";
pub const THERMOMETER: &str = "Термометр";
pub const SOCKET: &str = "Розетка";

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
        let mut result: Vec<_> = self.devices.keys().collect();
        result.sort();
        result
    }

    pub fn devices(&self, room: &str) -> Vec<&String> {
        let mut result: Vec<_> = self.devices[room].iter().collect();
        result.sort();
        result
    }

    pub fn create_report(&self, info_provider: Box<dyn DeviceInfoProvider>) -> String {
        let mut report = String::new();

        for room in self.rooms() {
            for device in self.devices(room) {
                let info = match info_provider.state(room, device) {
                    Ok(info) => info.to_string(),
                    Err(e) => format!("ошибка: {}", e),
                };

                report += format!(
                    "\n {:13} {}\n {:13} {}\n {:13} {}\n {:13} {}\n {:13} {}\n",
                    "Имя:",
                    self.name,
                    "Адрес:",
                    self.address,
                    "Комната:",
                    room,
                    "Устройство:",
                    device,
                    "Состояние:",
                    info
                )
                .as_str();
            }
        }

        report
    }
}

pub trait Device {
    fn new(name: String, connect_url: Option<String>) -> Self;
    fn name(&self) -> &str;
    fn connect(&self) -> Result<(), SmartHomeError> {
        Ok(())
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

pub struct BorrowingDeviceInfoProvider {
    pub socket: Rc<RefCell<SmartSocket>>,
    pub thermometer: Rc<RefCell<SmartThermometer>>,
}

pub enum DeviceInfo {
    OwningDeviceInfoProvider(OwningDeviceInfoProvider),
    BorrowingDeviceInfoProvider(BorrowingDeviceInfoProvider),
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn state(&self, room_name: &str, device_name: &str) -> Result<DeviceInfo, SmartHomeError> {
        if device_name != self.socket.name() {
            return Err(SmartHomeError::ErrDeviceNotFound {
                room_name: room_name.to_string(),
                device_name: device_name.to_string(),
            });
        }

        let mut socket = self.socket.clone();
        match room_name {
            KITCHEN => {
                socket.set_state(DeviceState::On)?;
            }
            LIVING_ROOM => {
                socket.set_state(DeviceState::Off)?;
            }
            BEDROOM => {
                socket.set_state(DeviceState::Unknown)?;
            }
            _ => {}
        }

        let info = DeviceInfo::OwningDeviceInfoProvider(OwningDeviceInfoProvider { socket });
        Ok(info)
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider {
    fn state(&self, room_name: &str, device_name: &str) -> Result<DeviceInfo, SmartHomeError> {
        if device_name != self.socket.borrow().name()
            && device_name != self.thermometer.borrow().name()
        {
            return Err(SmartHomeError::ErrDeviceNotFound {
                room_name: room_name.to_string(),
                device_name: device_name.to_string(),
            });
        }

        let mut socket = self.socket.clone();
        match room_name {
            KITCHEN => {
                socket.borrow_mut().set_state(DeviceState::Off)?;
            }
            LIVING_ROOM => {
                socket.borrow_mut().set_state(DeviceState::Unknown)?;
            }
            BEDROOM => {
                socket.borrow_mut().set_state(DeviceState::On)?;
            }
            _ => {}
        }
        // todo
        let mut thermometer = self.thermometer.clone();
        match room_name {
            KITCHEN => self.thermometer.borrow_mut().temperature = 21.11,
            LIVING_ROOM => self.thermometer.borrow_mut().temperature = 22.22,
            BEDROOM => self.thermometer.borrow_mut().temperature = 20.21,
            _ => {}
        }

        let info = DeviceInfo::BorrowingDeviceInfoProvider(BorrowingDeviceInfoProvider {
            socket,
            thermometer,
        });
        Ok(info)
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceInfo::OwningDeviceInfoProvider(info) => {
                write!(
                    f,
                    "статус - {}, мощность {:.2} pW",
                    info.socket.get_state().unwrap_or(&DeviceState::Unknown),
                    info.socket.power().unwrap_or_default(),
                )
            }
            DeviceInfo::BorrowingDeviceInfoProvider(info) => {
                write!(
                    f,
                    "статус - {}, мощность {:.2} pW, температура {:.2} tC",
                    info.socket
                        .borrow()
                        .get_state()
                        .unwrap_or(&DeviceState::Unknown),
                    info.socket.borrow().power().unwrap_or_default(),
                    info.thermometer.borrow().temperature().unwrap_or_default(),
                )
            }
        }
    }
}

pub enum SmartHomeError {
    ErrDeviceNotFound {
        room_name: String,
        device_name: String,
    },
    UnknownError,
}

impl fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmartHomeError::ErrDeviceNotFound {
                room_name,
                device_name,
            } => {
                write!(
                    f,
                    "устройство '{device_name}' для помещения '{room_name}' в источнике информации не найдено"
                )
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

#[derive(Clone)]
pub enum DeviceState {
    Off,
    On,
    Unknown,
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceState::Off => write!(f, "выключено"),
            DeviceState::On => write!(f, "включено"),
            DeviceState::Unknown => write!(f, "неизвестно"),
        }
    }
}
