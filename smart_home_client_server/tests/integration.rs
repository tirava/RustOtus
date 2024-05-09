use std::{thread::sleep, time};

use crate::common::*;
use smart_home_client_server::prelude::*;

mod common;

// Дом имеет название и содержит несколько помещений.
// Библиотека позволяет запросить список помещений в доме.
#[test]
fn test_house_new() {
    let house = new_house();

    assert_eq!(house.name(), HOUSE_NAME);

    let rooms = house.rooms();
    assert!(rooms.is_some());

    assert!(rooms.unwrap().len() > 1);
}

// Помещение имеет уникальное название и содержит названия нескольких устройств.
// Библиотека также позволяет добавлять и удалять помещения.
#[test]
fn test_house_rooms() {
    let mut house = new_house();

    let rooms = house.rooms();
    assert!(rooms.is_some());
    let mut rooms = rooms.unwrap();
    rooms.sort();

    assert_eq!(rooms, vec![LIVING_ROOM, KITCHEN, BEDROOM]);

    rooms.into_iter().for_each(|room| {
        let devices = house.devices(room);
        assert!(devices.is_some());
        assert!(devices.unwrap().len() > 1)
    });

    assert!(house.add_room(HALLWAY).is_ok());
    let rooms = house.rooms();
    assert!(rooms.is_some());
    let mut rooms = rooms.unwrap();
    rooms.sort();

    assert_eq!(rooms, vec![LIVING_ROOM, KITCHEN, HALLWAY, BEDROOM]);

    assert!(house.remove_room(HALLWAY).is_ok());
    let rooms = house.rooms();
    assert!(rooms.is_some());
    let mut rooms = rooms.unwrap();
    rooms.sort();

    assert_eq!(rooms, vec![LIVING_ROOM, KITCHEN, BEDROOM]);
}

// Устройство имеет уникальное в рамках помещения имя.
// Библиотека позволяет получать список устройств в помещении,
// а также добавлять и удалять устройства.
#[test]
fn test_house_devices() {
    let mut house = new_house();

    let rooms = house.rooms();
    assert!(rooms.is_some());

    rooms.unwrap().into_iter().for_each(|room| {
        let devices = house.devices(room);
        assert!(devices.is_some());
        let mut devices = devices.unwrap();
        devices.sort();

        match room {
            LIVING_ROOM => assert_eq!(devices, vec![SWITCH_2, SOCKET_1, THERMOMETER_1]),
            KITCHEN => assert_eq!(devices, vec![SWITCH_1, SOCKET_1, SOCKET_2]),
            BEDROOM => assert_eq!(devices, vec![SWITCH_1, SWITCH_2, THERMOMETER_2]),
            _ => panic!("Unknown room: {}", room),
        }
    });

    assert!(house.add_device(LIVING_ROOM, SOCKET_3).is_ok());
    let devices = house.devices(LIVING_ROOM);
    assert!(devices.is_some());
    let mut devices = devices.unwrap();
    devices.sort();

    assert_eq!(devices, vec![SWITCH_2, SOCKET_1, SOCKET_3, THERMOMETER_1]);

    assert!(house.remove_device(LIVING_ROOM, SOCKET_3).is_ok());
    let devices = house.devices(LIVING_ROOM);
    assert!(devices.is_some());
    let mut devices = devices.unwrap();
    devices.sort();

    assert_eq!(devices, vec![SWITCH_2, SOCKET_1, THERMOMETER_1])
}

// Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
// Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
// о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
// для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
// Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
#[test]
fn test_house_report() {
    let house = new_house();

    let (sockets, thermometers, switches) = init_devices(&house);

    let info_provider_1 = OwningDeviceInfoProvider { sockets };
    let report1 = house.create_report(&info_provider_1);
    assert!(report1.is_ok());

    let info_provider_2 = BorrowingDeviceInfoProvider {
        thermometers: &thermometers,
        switches: &switches,
    };
    let report2 = house.create_report(&info_provider_2);
    assert!(report2.is_ok());

    assert_eq!(report1.unwrap(),  "\n \
        Дом          : ".to_owned() +HOUSE_NAME+ "\n \
        Адрес        : "+HOUSE_ADDRESS+ "\n\n \
        Комната      : Гостинная\n \
        Устройства   :\n              \
                     : Выключатель-2, состояние: в источнике информации это устроство не обнаружено\n              \
                     : Розетка-1, состояние: статус - включено, мощность 111.22 pW\n              \
                     : Термометр-1, состояние: в источнике информации это устроство не обнаружено\n\n \
        Комната      : Кухня\n \
        Устройства   :\n              \
                     : Выключатель-1, состояние: в источнике информации это устроство не обнаружено\n              \
                     : Розетка-1, состояние: статус - включено, мощность 111.22 pW\n              \
                     : Розетка-2, состояние: статус - выключено, мощность 0.00 pW\n\n \
        Комната      : Спальня\n \
        Устройства   :\n              \
                     : Выключатель-1, состояние: в источнике информации это устроство не обнаружено\n              \
                     : Выключатель-2, состояние: в источнике информации это устроство не обнаружено\n              \
                     : Термометр-2, состояние: в источнике информации это устроство не обнаружено\n\n");

    assert_eq!(report2.unwrap(), "\n \
        Дом          : ".to_owned() +HOUSE_NAME+ "\n \
        Адрес        : "+HOUSE_ADDRESS+ "\n\n \
        Комната      : Гостинная\n \
        Устройства   :\n              \
                     : Выключатель-2, состояние: статус - выключено\n              \
                     : Розетка-1, состояние: в источнике информации это устроство не обнаружено\n              \
                     : Термометр-1, состояние: температура - 22.33 °С\n\n \
        Комната      : Кухня\n \
        Устройства   :\n              \
                     : Выключатель-1, состояние: статус - включено\n              \
                     : Розетка-1, состояние: в источнике информации это устроство не обнаружено\n              \
                     : Розетка-2, состояние: в источнике информации это устроство не обнаружено\n\n \
        Комната      : Спальня\n \
        Устройства   :\n              \
                     : Выключатель-1, состояние: статус - включено\n              \
                     : Выключатель-2, состояние: статус - выключено\n              \
                     : Термометр-2, состояние: температура - 22.33 °С\n\n");
}

// тест клиент-сервер для розетки
#[test]
fn test_socket_client_server() {
    run_socket_server(SOCKET_ADDR);

    sleep(time::Duration::from_secs(1));

    let result = SmartSocket::send_command(SOCKET_ADDR, "info");
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        format!(
            "name: {SOCKET_1}, room: {LIVING_ROOM}, status: {}, power: 111.22 pW",
            &DeviceStatus::On.to_string(),
        )
    );

    let result = SmartSocket::send_command(SOCKET_ADDR, "on");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "device is now ON");

    let result = SmartSocket::send_command(SOCKET_ADDR, "power");
    assert!(result.is_ok());
    let power = result.unwrap().parse::<f64>();
    assert!(power.is_ok());
    assert!(power.unwrap() > 0.0);

    let result = SmartSocket::send_command(SOCKET_ADDR, "off");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "device is now OFF");

    let result = SmartSocket::send_command(SOCKET_ADDR, "power");
    assert!(result.is_ok());
    let power = result.unwrap().parse::<f64>();
    assert!(power.is_ok());
    assert_eq!(power.unwrap(), 0.0);

    let result = SmartSocket::send_command(SOCKET_ADDR, "qqq");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "unknown command");
}
