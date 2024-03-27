use rand::Rng;
use std::fmt;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";

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
            LIVING_ROOM => [THERMOMETER_1, SOCKET_1],
            BEDROOM => [THERMOMETER_2, SOCKET_2],
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

struct SmartSocket {}
struct SmartThermometer {}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a> {
    socket: &'a SmartSocket,
    thermometer: &'a SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_state(&self, room: &str, device: &str) -> Option<String> {
        let mut status = DeviceStatus::Unknown;
        let mut power = 0.0;

        match device {
            SOCKET_1 => match room {
                KITCHEN => {
                    status = DeviceStatus::On;
                    power = rand::thread_rng().gen_range(1000.0..2500.0);
                }
                LIVING_ROOM => {
                    status = DeviceStatus::Off;
                }
                BEDROOM => {}
                _ => {
                    return None;
                }
            },
            SOCKET_2 => match room {
                KITCHEN => {
                    status = DeviceStatus::On;
                    power = rand::thread_rng().gen_range(500.0..1000.0);
                }
                LIVING_ROOM => {}
                BEDROOM => {
                    status = DeviceStatus::Off;
                }
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        }

        format!("статус - {}, мощность {:.2} pW", status, power).into()
    }
}
impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_> {
    fn get_device_state(&self, room: &str, device: &str) -> Option<String> {
        let mut status = DeviceStatus::Unknown;
        let mut power = 0.0;
        let mut themp = 0.0;

        match device {
            SOCKET_1 => match room {
                KITCHEN => {
                    status = DeviceStatus::Off;
                }
                LIVING_ROOM => {
                    status = DeviceStatus::On;
                    power = rand::thread_rng().gen_range(100.0..500.0);
                }
                BEDROOM => {}
                _ => {
                    return None;
                }
            },
            SOCKET_2 => match room {
                KITCHEN => {
                    status = DeviceStatus::Off;
                }
                LIVING_ROOM => {}
                BEDROOM => {
                    status = DeviceStatus::On;
                    power = rand::thread_rng().gen_range(100.0..500.0);
                }
                _ => {
                    return None;
                }
            },
            THERMOMETER_1 => match room {
                KITCHEN => {}
                LIVING_ROOM => {
                    themp = rand::thread_rng().gen_range(25.0..30.0);
                }
                BEDROOM => {}
                _ => {
                    return None;
                }
            },
            THERMOMETER_2 => match room {
                KITCHEN => {}
                LIVING_ROOM => {}
                BEDROOM => {
                    themp = rand::thread_rng().gen_range(20.0..25.0);
                }
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        }

        format!(
            "статус - {}, мощность {:.2} pW, температура {:.2} tC",
            status, power, themp
        )
        .into()
    }
}

enum DeviceStatus {
    Off,
    On,
    Unknown,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceStatus::Off => write!(f, "выключено"),
            DeviceStatus::On => write!(f, "включено"),
            DeviceStatus::Unknown => write!(f, "неизвестно"),
        }
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {};
    let socket2 = SmartSocket {};
    let thermometer = SmartThermometer {};

    // Инициализация дома
    let house = SmartHouse::new(
        "Мой умный дом".to_string(),
        "ул. Умных домов, д.1, кв.2".to_string(),
    );

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermometer: &thermometer,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    // println!("Report #1: {report1}");
    // println!("--------------------");
    println!("Report #2: {report2}");
}
