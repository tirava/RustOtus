use rand::Rng;
use std::fmt;

const KITCHEN: &str = "Кухня";
const LIVING_ROOM: &str = "Гостинная";
const BEDROOM: &str = "Спальня";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SWITCH_1: &str = "Выключатель-1";
const SWITCH_2: &str = "Выключатель-2";

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

    fn devices(&self, room: &str) -> [&str; 3] {
        match room {
            KITCHEN => [SOCKET_1, SOCKET_2, SWITCH_1],
            LIVING_ROOM => [THERMOMETER_1, SOCKET_1, SWITCH_2],
            BEDROOM => [THERMOMETER_2, SWITCH_1, SWITCH_2],
            _ => [""; 3],
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
                    .get_device_info(room, device)
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
    fn get_device_info(&self, room: &str, device: &str) -> Option<String>;
}

struct SmartSocket {
    name: String,
    room: String,
    status: DeviceStatus,
    power: f32,
}

impl SmartSocket {
    fn new(name: String, room: String, status: DeviceStatus, power: f32) -> Self {
        Self {
            name,
            room,
            status,
            power,
        }
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "статус - {}, мощность {:.2} pW", self.status, self.power)
    }
}

#[derive(Debug)]
struct SmartThermometer {
    name: String,
    room: String,
    temp: f32,
}

impl SmartThermometer {
    fn new(name: String, room: String, temp: f32) -> Self {
        Self { name, room, temp }
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "температура - {:.2} °С", self.temp)
    }
}

#[derive(Debug)]
struct SmartSwitch {
    name: String,
    room: String,
    status: DeviceStatus,
}

impl SmartSwitch {
    fn new(name: String, room: String, status: DeviceStatus) -> Self {
        Self { name, room, status }
    }
}

impl fmt::Display for SmartSwitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "статус - {}", self.status)
    }
}

struct OwningDeviceInfoProvider {
    sockets: Vec<SmartSocket>,
}

struct BorrowingDeviceInfoProvider<'a> {
    thermometers: Vec<&'a SmartThermometer>,
    switches: Vec<&'a SmartSwitch>,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String> {
        self.sockets
            .iter()
            .find(|s| s.name == device && s.room == room)?
            .to_string()
            .into()
    }
}

impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_> {
    fn get_device_info(&self, room: &str, device: &str) -> Option<String> {
        if let Some(thermometer) = self
            .thermometers
            .iter()
            .find(|s| s.name == device && s.room == room)
        {
            return thermometer.to_string().into();
        } else if let Some(switch) = self
            .switches
            .iter()
            .find(|s| s.name == device && s.room == room)
        {
            return switch.to_string().into();
        }

        None
    }
}

#[derive(Debug)]
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
    // Инициализация дома
    let house = SmartHouse::new(
        "Мой умный дом".to_string(),
        "ул. Умных домов, д.1, кв.2".to_string(),
    );

    // Инициализация устройств в доме со случайными показателями
    let mut sockets = vec![];
    let mut thermometers = vec![];
    let mut switches = vec![];

    for room in house.get_rooms() {
        for device in house.devices(room) {
            let mut socket = SmartSocket::new(
                device.to_string(),
                room.to_string(),
                DeviceStatus::Unknown,
                0.0,
            );
            let mut thermometer = SmartThermometer::new(device.to_string(), room.to_string(), 0.0);
            let mut switch =
                SmartSwitch::new(device.to_string(), room.to_string(), DeviceStatus::Unknown);
            match device {
                SOCKET_1 | SOCKET_2 => {
                    if device == SOCKET_1 {
                        socket.status = DeviceStatus::On;
                        socket.power = rand::thread_rng().gen_range(10.0..3000.0);
                    } else {
                        socket.status = DeviceStatus::Off;
                    }
                    sockets.push(socket);
                }
                THERMOMETER_1 | THERMOMETER_2 => {
                    thermometer.temp = rand::thread_rng().gen_range(20.0..30.0);
                    thermometers.push(thermometer);
                }
                SWITCH_1 | SWITCH_2 => {
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

    let thermometers = thermometers.iter().collect();
    let switches = switches.iter().collect();

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { sockets };
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        thermometers,
        switches,
    };
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("--------------------");
    println!("Report #2: {report2}");
}
