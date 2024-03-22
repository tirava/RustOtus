use smart_home_2::*;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER: &str = "Термометр";
const SOCKET: &str = "Розетка";

fn main() -> Result<(), SmartHomeError> {
    // Инициализация дома
    let mut home = SmartHome::new("Умный дом".to_string(), "Адрес дома, кв.1".to_string());

    // Инициализация помещений
    home.add_room(Room::new(KITCHEN.to_string()))?;
    home.add_room(Room::new(LIVING_ROOM.to_string()))?;
    home.add_room(Room::new(BEDROOM.to_string()))?;

    // Инициализация устройств
    let thermometer1 = Box::new(SmartThermometer::new(BaseDevice::new(
        THERMOMETER.to_string(),
        "127.0.0.1/api/thermometer1".to_string(),
    )));
    thermometer1.device.connect()?;

    let thermometer2 = Box::new(SmartThermometer::new(BaseDevice::new(
        THERMOMETER.to_string(),
        "127.0.0.1/api/thermometer2".to_string(),
    )));
    thermometer2.device.connect()?;

    let socket1 = Box::new(SmartSocket::new(BaseDevice::new(
        SOCKET.to_string(),
        "127.0.0.1/api/socket1".to_string(),
    )));
    socket1.device.connect()?;

    let socket2 = Box::new(SmartSocket::new(BaseDevice::new(
        SOCKET.to_string(),
        "127.0.0.1/api/socket2".to_string(),
    )));
    socket2.device.connect()?;

    // Добавление устройств в помещения
    home.get_room(LIVING_ROOM)?.add_device(thermometer1)?;
    home.get_room(BEDROOM)?.add_device(thermometer2)?;
    home.get_room(KITCHEN)?.add_device(socket1)?;
    home.get_room(LIVING_ROOM)?.add_device(socket2)?;

    // Включение устройств (розетки)
    home.get_room(KITCHEN)?
        .get_device(SOCKET)?
        .set_state(DeviceState::On)?;
    home.get_room(LIVING_ROOM)?
        .get_device(SOCKET)?
        .set_state(DeviceState::On)?;

    for room in home.rooms() {
        println!("{}:", room.get_name());
        for device in room.devices() {
            println!(
                "  - {} (состояние: {}, tC: {:.2}, pW: {:.2})",
                device.get_name(),
                device.get_state()?,
                device.temperature()?,
                device.power()?
            );
        }
    }

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
