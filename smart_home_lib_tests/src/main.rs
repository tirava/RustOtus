use rand::Rng;
use smart_home_2s::{
    BorrowingDeviceInfoProvider, DeviceStatus, OwningDeviceInfoProvider, SmartHouse, SmartSocket,
    SmartSwitch, SmartThermometer,
};
use std::collections::HashMap;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SWITCH_1: &str = "Выключатель-1";
const SWITCH_2: &str = "Выключатель-2";

fn main() {
    // Инициализация дома
    let house = SmartHouse::new(
        "Мой умный дом".to_string(),
        "ул. Умных домов, д.1, кв.2".to_string(),
        HashMap::from([
            (KITCHEN, vec![SOCKET_1, SOCKET_2, SWITCH_1]),
            (LIVING_ROOM, vec![THERMOMETER_1, SOCKET_1, SWITCH_2]),
            (BEDROOM, vec![THERMOMETER_2, SWITCH_1, SWITCH_2]),
        ]),
    );

    // Инициализация устройств в доме со случайными показателями
    let mut sockets = vec![];
    let mut thermometers = vec![];
    let mut switches = vec![];

    for room in house.rooms() {
        for device in house.devices(room) {
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
                        socket.power = rand::thread_rng().gen_range(10.0..3000.0);
                    } else {
                        socket.status = DeviceStatus::Off;
                    }
                    sockets.push(socket);
                }
                THERMOMETER_1 | THERMOMETER_2 => {
                    let mut thermometer =
                        SmartThermometer::new(device.to_string(), room.to_string(), 0.0);
                    thermometer.temp = rand::thread_rng().gen_range(20.0..30.0);
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

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { sockets };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        thermometers: &thermometers,
        switches: &switches,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("--------------------");
    println!("Report #2: {report2}");
}
