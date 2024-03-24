use smart_home_2::*;
use std::collections::{HashMap, HashSet};

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER: &str = "Термометр";
const SOCKET: &str = "Розетка";

fn main() -> Result<(), SmartHomeError> {
    // --------------------------------------------------------------------------------------------
    // Пример #1: использование библиотеки для управления устройствами
    // --------------------------------------------------------------------------------------------

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
        Some("127.0.0.1/api/thermometer1".to_string()),
    );
    thermometer1.connect()?;

    let thermometer2 = SmartThermometer::new(
        THERMOMETER.to_string(),
        Some("127.0.0.1/api/thermometer2".to_string()),
    );
    thermometer2.connect()?;

    let mut socket1 = SmartSocket::new(
        SOCKET.to_string(),
        Some("127.0.0.1/api/socket1".to_string()),
    );
    socket1.connect()?;
    socket1.set_state(DeviceState::On)?;

    let mut socket2 = SmartSocket::new(
        SOCKET.to_string(),
        Some("127.0.0.1/api/socket2".to_string()),
    );
    socket2.connect()?;
    socket2.set_state(DeviceState::On)?;

    println!("{:?}", home.rooms());
    println!("{KITCHEN} - {:?}", home.devices(KITCHEN));
    println!("{LIVING_ROOM} - {:?}", home.devices(LIVING_ROOM));
    println!("{BEDROOM} - {:?}", home.devices(BEDROOM));
    println!(
        "{}1 - {:.2} tC",
        thermometer1.name(),
        thermometer1.temperature()?
    );
    println!(
        "{}2 - {:.2} tC",
        thermometer2.name(),
        thermometer2.temperature()?
    );
    println!(
        "{}1 - {} - {:.2} pW",
        socket1.name(),
        socket1.get_state()?,
        socket1.power()?
    );
    println!(
        "{}2 - {} - {:.2} pW",
        socket2.name(),
        socket2.get_state()?,
        socket2.power()?
    );

    println!("------------------------------------------------------------------------------");
    // --------------------------------------------------------------------------------------------
    // Пример #2: использование библиотеки для построения отчётов
    // --------------------------------------------------------------------------------------------

    // Инициализация устройств
    let socket1 = SmartSocket::new(SOCKET.to_string(), None);
    let socket2 = SmartSocket::new(SOCKET.to_string(), None);
    let thermo = SmartThermometer::new(THERMOMETER.to_string(), None);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = home.create_report(Box::new(info_provider_1))?;

    // // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    // let info_provider_2 = BorrowingDeviceInfoProvider {
    //     socket: &socket2,
    //     thermo: &thermo,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report2 = house.create_report(/* &info_provider_2 */);
    //
    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    // println!("Report #2: {report2}");

    Ok(())
}
