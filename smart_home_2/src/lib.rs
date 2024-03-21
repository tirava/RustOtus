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

pub struct SmartHome {
    name: String,
    address: String,
    rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(name: String, address: String) -> Self {
        Self {
            name,
            address,
            rooms: vec![],
        }
    }

    pub fn set_rooms(&mut self, rooms: Vec<Room>) -> Result<(), SmartHomeError> {
        // Проверить имена помещений на уникальность
        if rooms.iter().enumerate().any(|(i, room1)| {
            rooms
                .iter()
                .enumerate()
                .any(|(j, room2)| i < j && room1.name.to_lowercase() == room2.name.to_lowercase())
        }) {
            return Err(SmartHomeError::RoomsMustBeUnique);
        }

        self.rooms = rooms;
        Ok(())
    }

    pub fn get_rooms(&self) -> Vec<&str> {
        self.rooms.iter().map(|room| room.name.as_str()).collect()
    }

    // fn devices(&self, room: &str) -> [&str; 3] {
    //     // Размер возвращаемого массива можно выбрать самостоятельно
    //     todo!("список устройств в комнате `room`")
    // }
    //
    // fn create_report(
    //     &self,
    //     /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    // ) -> String {
    //     todo!("перебор комнат и устройств в них для составления отчёта")
    // }
}

pub struct Room {
    name: String,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

// trait DeviceInfoProvider {
//     // todo: метод, возвращающий состояние устройства по имени комнаты и имени устройства
// }
//
// // ***** Пример использования библиотеки умный дом:
//
// // Пользовательские устройства:
// struct SmartSocket {}
// struct SmartThermometer {}
//
// // Пользовательские поставщики информации об устройствах.
// // Могут как хранить устройства, так и заимствывать.
// struct OwningDeviceInfoProvider {
//     socket: SmartSocket,
// }
// struct BorrowingDeviceInfoProvider<'a, 'b> {
//     socket: &'a SmartSocket,
//     thermo: &'b SmartThermometer,
// }
//
// // todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

pub enum SmartHomeError {
    RoomsMustBeUnique,
}

impl std::fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeError::RoomsMustBeUnique => write!(f, "Помещения должны быть уникальными!"),
        }
    }
}
