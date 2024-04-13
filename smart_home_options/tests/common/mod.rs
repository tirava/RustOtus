use smart_home_options::prelude::*;
use std::collections::HashMap;

pub const HOUSE_NAME: &str = "Мой умный дом";
pub const HOUSE_ADDRESS: &str = "ул. Умных домов, д.1, кв.2";
pub const KITCHEN: &str = "Кухня";
pub const LIVING_ROOM: &str = "Гостинная";
pub const BEDROOM: &str = "Спальня";
pub const THERMOMETER_1: &str = "Термометр-1";
pub const THERMOMETER_2: &str = "Термометр-2";
pub const SOCKET_1: &str = "Розетка-1";
pub const SOCKET_2: &str = "Розетка-2";
pub const SWITCH_1: &str = "Выключатель-1";
pub const SWITCH_2: &str = "Выключатель-2";

pub(crate) fn new_house() -> SmartHouse {
    SmartHouse::new(
        HOUSE_NAME.to_string(),
        HOUSE_ADDRESS.to_string(),
        HashMap::from([
            (KITCHEN, vec![SOCKET_1, SOCKET_2, SWITCH_1, SWITCH_1]), // has double switch
            (
                LIVING_ROOM,
                vec![THERMOMETER_1, SOCKET_1, SWITCH_2, SOCKET_1], // has double socket
            ),
            (
                BEDROOM,
                vec![THERMOMETER_2, SWITCH_1, SWITCH_2, THERMOMETER_2], // has double thermometer
            ),
            (
                BEDROOM,                                                // has double room
                vec![THERMOMETER_2, SWITCH_1, SWITCH_2, THERMOMETER_2], // has double thermometer
            ),
        ]),
    )
}

pub(crate) fn init_devices(
    house: &SmartHouse,
) -> (Vec<SmartSocket>, Vec<SmartThermometer>, Vec<SmartSwitch>) {
    let mut sockets = vec![];
    let mut thermometers = vec![];
    let mut switches = vec![];

    let rooms = house.rooms();
    assert!(rooms.is_some());

    for room in rooms.unwrap() {
        let devices = house.devices(room);
        assert!(devices.is_some());
        for device in devices.unwrap() {
            match device {
                SOCKET_1 | SOCKET_2 => {
                    let mut socket = SmartSocket::new(
                        device.to_string(),
                        room.to_string(),
                        DeviceStatus::Unknown,
                        0.0,
                    );
                    if device == SOCKET_1 {
                        socket.status = DeviceStatus::On;
                        socket.power = 111.222;
                    } else {
                        socket.status = DeviceStatus::Off;
                    }
                    sockets.push(socket);
                }
                THERMOMETER_1 | THERMOMETER_2 => {
                    let mut thermometer =
                        SmartThermometer::new(device.to_string(), room.to_string(), 0.0);
                    thermometer.temp = 22.33;
                    thermometers.push(thermometer);
                }
                SWITCH_1 | SWITCH_2 => {
                    let mut switch = SmartSwitch::new(
                        device.to_string(),
                        room.to_string(),
                        DeviceStatus::Unknown,
                    );
                    if device == SWITCH_1 {
                        switch.status = DeviceStatus::On;
                    } else {
                        switch.status = DeviceStatus::Off;
                    }
                    switches.push(switch);
                }
                _ => {}
            }
        }
    }

    (sockets, thermometers, switches)
}
