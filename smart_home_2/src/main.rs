use smart_home_2::*;
use std::collections::{HashMap, HashSet};

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER: &str = "Термометр";
const SOCKET: &str = "Розетка";

fn main() -> Result<(), SmartHomeError> {
    // Инициализация дома и списка устройств
    let room_devices = HashMap::from([
        (KITCHEN, HashSet::from([SOCKET])),
        (LIVING_ROOM, HashSet::from([THERMOMETER, SOCKET])),
        (BEDROOM, HashSet::from([THERMOMETER])),
    ]);

    let home = SmartHome::new(
        "Умный дом".to_string(),
        "Адрес дома, кв.1".to_string(),
        room_devices,
    );

    // Инициализация устройств
    let thermometer1 = SmartThermometer::new(
        THERMOMETER.to_string(),
        "127.0.0.1/api/thermometer1".to_string(),
    );
    thermometer1.connect()?;

    let thermometer2 = SmartThermometer::new(
        THERMOMETER.to_string(),
        "127.0.0.1/api/thermometer2".to_string(),
    );
    thermometer2.connect()?;

    println!("{:?}", home.rooms());
    println!("{KITCHEN} - {:?}", home.devices(KITCHEN));
    println!("{LIVING_ROOM} - {:?}", home.devices(LIVING_ROOM));
    println!("{BEDROOM} - {:?}", home.devices(BEDROOM));
    println!(
        "{}1 - {:.2}",
        thermometer1.name(),
        thermometer1.temperature()?
    );
    println!(
        "{}2 - {:.2}",
        thermometer2.name(),
        thermometer2.temperature()?
    );

    // let socket1 = Box::new(SmartSocket::new(BaseDevice::new(
    //     SOCKET.to_string(),
    //     "127.0.0.1/api/socket1".to_string(),
    // )));
    // socket1.device.connect()?;
    //
    // let socket2 = Box::new(SmartSocket::new(BaseDevice::new(
    //     SOCKET.to_string(),
    //     "127.0.0.1/api/socket2".to_string(),
    // )));
    // socket2.device.connect()?;
    //
    // // Включение устройств (розетки)
    // home.get_room(KITCHEN)?
    //     .get_device(SOCKET)?
    //     .set_state(DeviceState::On)?;
    // home.get_room(LIVING_ROOM)?
    //     .get_device(SOCKET)?
    //     .set_state(DeviceState::On)?;
    //
    // for room in home.rooms() {
    //     println!("{}:", room.get_name());
    //     for device in room.devices() {
    //         println!(
    //             "  - {} (состояние: {}, tC: {:.2}, pW: {:.2})",
    //             device.get_name(),
    //             device.get_state()?,
    //             device.temperature()?,
    //             device.power()?
    //         );
    //     }
    // }

    // // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    // let info_provider_1 = OwningDeviceInfoProvider {
    //     socket: socket1,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report1 = house.create_report(/* &info_provider_1 */);
    //
    // // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    // let info_provider_2 = BorrowingDeviceInfoProvider {
    //     socket: &socket2,
    //     thermo: &thermo,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report2 = house.create_report(/* &info_provider_2 */);
    //
    // // Выводим отчёты на экран:
    // println!("Report #1: {report1}");
    // println!("Report #2: {report2}");

    Ok(())
}
