use smart_home_web::prelude::*;
use std::collections::HashMap;

pub const HOUSE_NAME: &str = "Мой умный дом";
pub const HOUSE_ADDRESS: &str = "ул. Умных домов, д.1, кв.2";
pub const KITCHEN: &str = "Кухня";
pub const LIVING_ROOM: &str = "Гостинная";
pub const BEDROOM: &str = "Спальня";
pub const HALLWAY: &str = "Прихожая";
pub const THERMOMETER_1: &str = "Термометр-1";
pub const THERMOMETER_2: &str = "Термометр-2";
pub const SOCKET_1: &str = "Розетка-1";
pub const SOCKET_2: &str = "Розетка-2";
pub const SOCKET_3: &str = "Розетка-3";
pub const SWITCH_1: &str = "Выключатель-1";
pub const SWITCH_2: &str = "Выключатель-2";
pub const SOCKET_ADDR: &str = "127.0.0.1:54321";
pub const THERMOMETER_ADDR: &str = "127.0.0.1:12345";
pub const SWITCH_ADDR: &str = "127.0.0.1:31254";

pub(crate) fn new_house() -> SmartHouse {
    SmartHouse::new(
        HOUSE_NAME.to_string(),
        HOUSE_ADDRESS.to_string(),
        HashMap::from([
            (KITCHEN, &[SOCKET_1, SOCKET_2, SWITCH_1, SWITCH_1][..]), // has double switch
            (
                LIVING_ROOM,
                &[THERMOMETER_1, SOCKET_1, SWITCH_2, SOCKET_1], // has double socket
            ),
            (
                BEDROOM,
                &[THERMOMETER_2, SWITCH_1, SWITCH_2, THERMOMETER_2], // has double thermometer
            ),
            (
                BEDROOM,                                             // has double room
                &[THERMOMETER_2, SWITCH_1, SWITCH_2, THERMOMETER_2], // has double thermometer
            ),
        ]),
    )
}

pub fn run_socket_server(addr: &str) {
    let addr = addr.to_string();
    let smart_socket = SmartSocket::new(
        SOCKET_1.to_string(),
        LIVING_ROOM.to_string(),
        DeviceStatus::Off,
        0.0,
    );
    tokio::spawn(async move {
        assert!(smart_socket.listen(&addr).await.is_err());
    });
}

pub fn run_thermometer_server(addr: &str) {
    let addr = addr.to_string();
    let smart_thermometer =
        SmartThermometer::new(THERMOMETER_1.to_string(), BEDROOM.to_string(), 22.33);
    tokio::spawn(async move {
        assert!(smart_thermometer.listen(&addr).await.is_err());
    });
}

pub fn run_switch_server(addr: &str) {
    let addr = addr.to_string();
    let smart_switch =
        SmartSwitch::new(SOCKET_2.to_string(), KITCHEN.to_string(), DeviceStatus::Off);
    tokio::spawn(async move {
        assert!(smart_switch.listen(&addr).await.is_err());
    });
}
