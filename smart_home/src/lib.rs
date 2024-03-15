use rand::Rng;
use std::fmt;
use std::io::Error;

pub struct SmartHome {
    name: String,
    address: String,
}

impl SmartHome {
    pub fn new(name: String, address: String) -> SmartHome {
        SmartHome { name, address }
    }
}

pub struct Room<'a> {
    name: String,
    home: &'a SmartHome,
}

impl Room<'_> {
    pub fn new(name: String, home: &SmartHome) -> Room {
        Room { name, home }
    }
}

pub struct SmartDevice<'a> {
    name: String,
    room: &'a Room<'a>,
    connect_url: String,
}

impl SmartDevice<'_> {
    pub fn new<'a>(name: String, room: &'a Room<'a>, connect_url: String) -> SmartDevice<'a> {
        SmartDevice {
            name,
            room,
            connect_url,
        }
    }

    pub fn connect(&self) -> Result<(), Error> {
        let _ = &self.connect_url;

        Ok(())
    }

    pub fn info(&self) -> String {
        format!(
            "'{}' находится в комнате '{}' в доме '{}' по адресу '{}'",
            self.name, self.room.name, self.room.home.name, self.room.home.address
        )
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct SmartPowerSocket<'a> {
    pub device: SmartDevice<'a>,
    state: DeviceState,
    power: f64,
}

impl SmartPowerSocket<'_> {
    pub fn new(device: SmartDevice) -> SmartPowerSocket {
        SmartPowerSocket {
            device,
            state: DeviceState::Unknown,
            power: 0.0,
        }
    }

    pub fn get_state(&self) -> Result<&DeviceState, Error> {
        // todo get state from device with error handling
        Ok(&self.state)
    }

    pub fn set_state(&mut self, state: DeviceState) -> Result<&SmartPowerSocket, Error> {
        // todo upload to device with error handling
        self.state = state;

        // todo delete after we have real device
        // fake device logic
        match self.state {
            DeviceState::On => self.power = rand::thread_rng().gen_range(100.0..3500.0),
            _ => self.power = 0.0,
        }

        Ok(self)
    }

    pub fn power(&self) -> Result<f64, Error> {
        // todo get power from device
        Ok(self.power)
    }
}

pub struct SmartThermometer<'a> {
    pub device: SmartDevice<'a>,
    temperature: f64,
}

impl SmartThermometer<'_> {
    pub fn new(device: SmartDevice) -> SmartThermometer {
        SmartThermometer {
            device,
            // fake device logic
            temperature: rand::thread_rng().gen_range(20.0..25.0),
        }
    }

    pub fn temperature(&self) -> Result<f64, Error> {
        // todo get temperature from device
        Ok(self.temperature)
    }
}

#[derive(PartialEq)]
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
