// Дом имеет название и содержит несколько помещений.
// Библиотека позволяет запросить список помещений в доме.
// Помещение имеет уникальное название и содержит названия нескольких устройств.
// Устройство имеет уникальное в рамках помещения имя.
// Библиотека позволяет получать список устройств в помещении.
// Библиотека имеет функцию, возвращающую текстовый отчёт о состоянии дома.
// Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
// о состоянии устройства, для включения в отчёт. Эта информация должна предоставляться
// для каждого устройства на основе данных о положении устройства в доме: имени комнаты и имени устройства.
// Если устройство не найдено в источнике информации, то вместо текста о состоянии вернуть сообщение об ошибке.
// Привести пример типа, предоставляющего текстовую информацию об устройствах в доме для составления отчёта.

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER: &str = "Термометр";
const SOCKET: &str = "Розетка";

struct SmartHouse {
    name: String,
}

impl SmartHouse {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn get_rooms(&self) -> [&str; 3] {
        [KITCHEN, LIVING_ROOM, BEDROOM]
    }

    fn devices(&self, room: &str) -> [&str; 2] {
        match room {
            KITCHEN => [SOCKET, ""],
            LIVING_ROOM => [THERMOMETER, SOCKET],
            BEDROOM => [THERMOMETER, ""],
            _ => ["", ""],
        }
    }

    fn create_report(&self, info_provider: &impl DeviceInfoProvider) -> String {
        // todo!("перебор комнат и устройств в них для составления отчёта")
        String::new()
    }
}

trait DeviceInfoProvider {
    // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
    fn get_device_state(&self, room: &str, device: &str) -> String;
}

struct SmartSocket {}
struct SmartThermometer {}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации
impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_state(&self, room: &str, device: &str) -> String {
        String::new()
    }
}
impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_device_state(&self, room: &str, device: &str) -> String {
        String::new()
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermo = SmartThermometer {};

    // Инициализация дома
    let house = SmartHouse::new("Мой умный дом".to_string());

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
