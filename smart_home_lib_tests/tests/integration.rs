use crate::common::*;

mod common;

// Дом имеет название и содержит несколько помещений.
// Библиотека позволяет запросить список помещений в доме.
#[test]
fn test_house_new() {
    let house = new_house();

    assert_eq!(house.name(), HOUSE_NAME);
    assert!(house.rooms().len() > 1);
}

// Помещение имеет уникальное название и содержит названия нескольких устройств.
#[test]
fn test_house_rooms() {
    let house = new_house();

    let mut rooms = house.rooms();
    rooms.sort();
    assert_eq!(rooms, vec![LIVING_ROOM, KITCHEN, BEDROOM]);

    rooms
        .into_iter()
        .for_each(|room| assert!(house.devices(room).len() > 1));
}

// Устройство имеет уникальное в рамках помещения имя.
// Библиотека позволяет получать список устройств в помещении.
#[test]
fn test_house_devices() {
    let house = new_house();

    let rooms = house.rooms();
    rooms.into_iter().for_each(|room| {
        let mut devices = house.devices(room);
        devices.sort();
        match room.as_str() {
            LIVING_ROOM => assert_eq!(devices, vec![SWITCH_2, SOCKET_1, THERMOMETER_1]),
            KITCHEN => assert_eq!(devices, vec![SWITCH_1, SOCKET_1, SOCKET_2]),
            BEDROOM => assert_eq!(devices, vec![SWITCH_1, SWITCH_2, THERMOMETER_2]),
            _ => panic!("Unknown room: {}", room),
        }
    });
}

// Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
// Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
// о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
// для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
// Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
