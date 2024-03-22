// Дом имеет название и содержит несколько помещений.
// Библиотека позволяет запросить список помещений в доме.
// Помещение имеет уникальное название и содержит названия нескольких устройств.
// Устройство имеет уникальное в рамках помещения имя.

// Библиотека позволяет получать список устройств в помещении.

// Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
// Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
// о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
// для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
// Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
// Привести пример типа, предоставляющего текстовую информацию об устройствах в доме для составления отчёта.

use std::fmt::{Debug, Display, Formatter};
use std::io::Error;

pub struct SmartHome {
    name: String,
    address: String,
    rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(name: String, address: String) -> Self {
        Self {
            name,
            address,
            rooms: vec![],
        }
    }

    pub fn add_room(&mut self, room: Room) -> Result<(), SmartHomeError> {
        // Проверить имена помещений на уникальность
        if self
            .rooms
            .iter()
            .any(|r| r.name.to_lowercase() == room.name.to_lowercase())
        {
            return Err(SmartHomeError::ErrRoomsMustBeUnique {
                room_name: room.name,
            });
        }

        self.rooms.push(room);

        Ok(())
    }

    pub fn get_room(&mut self, room_name: &str) -> Result<&mut Room, SmartHomeError> {
        self.rooms
            .iter_mut()
            .find(|r| r.name == room_name)
            .ok_or(SmartHomeError::ErrRoomNotFound {
                room_name: room_name.to_string(),
            })
    }

    pub fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    // fn create_report(
    //     &self,
    //     /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    // ) -> String {
    //     todo!("перебор комнат и устройств в них для составления отчёта")
    // }
}

pub struct Room {
    name: String,
    devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name,
            devices: vec![],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_device(&mut self, device: SmartDevice) -> Result<(), SmartHomeError> {
        // Проверить имена устройств на уникальность в помещении
        if self
            .devices
            .iter()
            .any(|r| r.name.to_lowercase() == device.name.to_lowercase())
        {
            return Err(SmartHomeError::ErrDevicesInRoomMustBeUnique {
                device_name: device.name,
            });
        }

        self.devices.push(device);

        Ok(())
    }

    pub fn devices(&self) -> &Vec<SmartDevice> {
        &self.devices
    }
}

pub struct SmartDevice {
    name: String,
    connect_url: String,
}

impl SmartDevice {
    pub fn new(name: String, connect_url: String) -> Self {
        SmartDevice { name, connect_url }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn connect(&self) -> Result<(), SmartHomeError> {
        let _ = &self.connect_url;

        Ok(())
    }
}

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
    ErrRoomsMustBeUnique { room_name: String },
    ErrDevicesInRoomMustBeUnique { device_name: String },
    ErrRoomNotFound { room_name: String },
    UnknownError,
}

impl Display for SmartHomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeError::ErrRoomsMustBeUnique { room_name } => {
                write!(f, "помещение должно быть уникальным: {room_name}")
            }
            SmartHomeError::ErrDevicesInRoomMustBeUnique { device_name } => {
                write!(
                    f,
                    "устройство в помещении должно быть уникальным: {device_name}"
                )
            }
            SmartHomeError::ErrRoomNotFound { room_name } => {
                write!(f, "помещение не найдено: {room_name}")
            }
            SmartHomeError::UnknownError => write!(f, "неизвестная ошибка"),
        }
    }
}

impl Debug for SmartHomeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}
