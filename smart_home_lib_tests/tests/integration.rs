use crate::common::*;
use smart_home_lib_tests::prelude::*;

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
#[test]
fn test_house_report() {
    let house = new_house();

    let (sockets, thermometers, switches) = init_devices(&house);

    let info_provider_1 = OwningDeviceInfoProvider { sockets };
    let report1 = house.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        thermometers: &thermometers,
        switches: &switches,
    };
    let report2 = house.create_report(&info_provider_2);

    assert_eq!(report1,  "\n \
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

    assert_eq!(report2, "\n \
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
