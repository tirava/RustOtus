use rand::Rng;
use smart_home_web::prelude::*;
use std::collections::HashMap;
use std::sync::atomic::Ordering::SeqCst;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const HALLWAY: &str = "Прихожая";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SOCKET_3: &str = "Розетка-3";
const SWITCH_1: &str = "Выключатель-1";
const SWITCH_2: &str = "Выключатель-2";

fn main() -> Result<(), SmartHouseError> {
    // Инициализация дома
    let mut house = SmartHouse::new(
        "Мой умный дом".to_string(),
        "ул. Умных домов, д.1, кв.2".to_string(),
        HashMap::from([
            (KITCHEN, &[SOCKET_1, SOCKET_2, SWITCH_1][..]),
            (LIVING_ROOM, &[THERMOMETER_1, SOCKET_1, SWITCH_2]),
            (BEDROOM, &[THERMOMETER_2, SWITCH_1, SWITCH_2]),
        ]),
    );

    // Добавление помещений и устройств в них
    house.add_room(HALLWAY)?;
    house.add_device(HALLWAY, SOCKET_3)?;

    // Инициализация устройств в доме со случайными показателями
    let mut sockets = vec![];
    let mut thermometers = vec![];
    let mut switches = vec![];

    let rooms = match house.rooms() {
        Some(rooms) => rooms,
        None => return Err(SmartHouseError::RoomsNotFoundError),
    };

    for room in rooms {
        let devices = match house.devices(room) {
            Some(devices) => devices,
            None => return Err(SmartHouseError::DevicesNotFoundError),
        };
        for device in devices {
            match device {
                SOCKET_1 | SOCKET_2 | SOCKET_3 => {
                    let socket = SmartSocket::new(
                        device.to_string(),
                        room.to_string(),
                        DeviceStatus::Unknown,
                        0.0,
                    );
                    if device == SOCKET_1 {
                        socket.status.store(DeviceStatus::On, SeqCst);
                        socket
                            .power
                            .store(rand::thread_rng().gen_range(10.0..3000.0), SeqCst);
                    } else if device == SOCKET_2 {
                        socket.status.store(DeviceStatus::Off, SeqCst);
                    } else {
                        socket.status.store(DeviceStatus::On, SeqCst);
                        socket
                            .power
                            .store(rand::thread_rng().gen_range(1.0..10.0), SeqCst);
                    }
                    sockets.push(socket);
                }
                THERMOMETER_1 | THERMOMETER_2 => {
                    let thermometer =
                        SmartThermometer::new(device.to_string(), room.to_string(), 0.0);
                    thermometer
                        .temp
                        .store(rand::thread_rng().gen_range(20.0..30.0), SeqCst);
                    thermometers.push(thermometer);
                }
                SWITCH_1 | SWITCH_2 => {
                    let switch = SmartSwitch::new(
                        device.to_string(),
                        room.to_string(),
                        DeviceStatus::Unknown,
                    );
                    if device == SWITCH_1 {
                        switch.status.store(DeviceStatus::On, SeqCst);
                    } else {
                        switch.status.store(DeviceStatus::Off, SeqCst);
                    }
                    switches.push(switch);
                }
                _ => {}
            }
        }
    }

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { sockets };
    let report1 = house.create_report(&info_provider_1)?;

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        thermometers: &thermometers,
        switches: &switches,
    };
    let report2 = house.create_report(&info_provider_2)?;

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("--------------------");
    println!("Report #2: {report2}");

    // Удаление помещений и устройств
    house.remove_device(HALLWAY, SOCKET_3)?;
    house.remove_room(HALLWAY)
}
