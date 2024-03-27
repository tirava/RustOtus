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

use std::fmt;
use std::fmt::format;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SOCKET_3: &str = "Розетка-3";
const SOCKET_4: &str = "Розетка-4";

struct SmartHouse {
    name: String,
    address: String,
}

impl SmartHouse {
    fn new(name: String, address: String) -> Self {
        Self { name, address }
    }

    fn get_rooms(&self) -> [&str; 3] {
        [KITCHEN, LIVING_ROOM, BEDROOM]
    }

    fn devices(&self, room: &str) -> [&str; 2] {
        match room {
            KITCHEN => [SOCKET_1, SOCKET_2],
            LIVING_ROOM => [THERMOMETER_1, SOCKET_3],
            BEDROOM => [THERMOMETER_2, SOCKET_4],
            _ => [""; 2],
        }
    }

    fn create_report(&self, info_provider: &impl DeviceInfoProvider) -> String {
        let mut report = format!(
            "\n {:13}: {}\n {:13}: {}\n\n",
            "Дом", self.name, "Адрес", self.address
        );

        for room in self.get_rooms() {
            report += format!(" {:13}: {}\n {:13}:\n", "Комната", room, "Устройства").as_str();
            for device in self.devices(room) {
                let info = info_provider
                    .get_device_state(room, device)
                    .unwrap_or_else(|| {
                        "в источнике информации это устроство не обнаружено".to_string()
                    });

                report +=
                    format!("              : {}, {}: {}\n", device, "состояние", info).as_str();
            }
            report += "\n";
        }

        report
    }
}

trait DeviceInfoProvider {
    fn get_device_state(&self, room: &str, device: &str) -> Option<String>;
}

struct SmartSocket {
    name: String,
    room: String,
    state: DeviceState,
    power: f64,
}
struct SmartThermometer {
    name: String,
    room: String,
    temperature: f64,
}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermometer: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_state(&self, room: &str, device: &str) -> Option<String> {
        if room != self.socket.room || device != self.socket.name {
            return None;
        }

        format!(
            "статус - {}, мощность {:.2} pW",
            self.socket.state,
            self.socket.power.to_string()
        )
        .into()
    }
}
impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_, '_> {
    fn get_device_state(&self, room: &str, device: &str) -> Option<String> {
        if room != self.socket.room
            && (device != self.socket.name || device != self.thermometer.name)
        {
            return None;
        }

        // let mut themp = 0.0;
        // if device == THERMOMETER_1 || device == THERMOMETER_2 {
        //     themp = rand::thread_rng().gen_range(20.0..25.0);
        // }
        //
        // let mut power = 0.0;
        // let mut status = DeviceState::Unknown;
        // if device == SOCKET_1 || device == SOCKET_2 || device == SOCKET_3 || device == SOCKET_4 {
        //     status = match rand::thread_rng().gen_range(0..=1) {
        //         1 => {
        //             power = rand::thread_rng().gen_range(100.0..2500.0);
        //             DeviceState::On
        //         }
        //         _ => DeviceState::Off,
        //     }
        // }
        //
        // format!(
        //     "статус - {}, мощность {:.2} pW, температура {:.2} tC",
        //     status, power, themp
        // )
        // .into()
        Some(String::new())
    }
}

enum DeviceState {
    Off,
    On,
    Unknown,
}

impl fmt::Display for DeviceState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceState::Off => write!(f, "выключено"),
            DeviceState::On => write!(f, "включено"),
            DeviceState::Unknown => write!(f, "неизвестно"),
        }
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        name: SOCKET_1.to_string(),
        room: KITCHEN.to_string(),
        state: DeviceState::On,
        power: 111.222,
    };
    let socket2 = SmartSocket {
        name: SOCKET_2.to_string(),
        room: LIVING_ROOM.to_string(),
        state: DeviceState::Off,
        power: 0.333,
    };
    let thermometer = SmartThermometer {
        room: BEDROOM.to_string(),
        name: THERMOMETER_1.to_string(),
        temperature: 22.33,
    };

    // Инициализация дома
    let house = SmartHouse::new(
        "Мой умный дом".to_string(),
        "ул. Умных домов, д.1, кв.2".to_string(),
    );

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    // let info_provider_2 = BorrowingDeviceInfoProvider {
    //     socket: &socket2,
    //     thermometer: &thermometer,
    // };
    // let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("--------------------");
    // println!("Report #2: {report2}");
}
