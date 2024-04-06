use smart_home_lib_tests::prelude::SmartHouse;
use std::collections::HashMap;

pub const HOUSE_NAME: &str = "Мой умный дом";
const HOUSE_ADDRESS: &str = "ул. Умных домов, д.1, кв.2";
pub const KITCHEN: &str = "Кухня";
pub const LIVING_ROOM: &str = "Гостинная";
pub const BEDROOM: &str = "Спальня";
const THERMOMETER_1: &str = "Термометр-1";
const THERMOMETER_2: &str = "Термометр-2";
const SOCKET_1: &str = "Розетка-1";
const SOCKET_2: &str = "Розетка-2";
const SWITCH_1: &str = "Выключатель-1";
const SWITCH_2: &str = "Выключатель-2";

pub(crate) fn new_house() -> SmartHouse {
    SmartHouse::new(
        HOUSE_NAME.to_string(),
        HOUSE_ADDRESS.to_string(),
        HashMap::from([
            (KITCHEN, vec![SOCKET_1, SOCKET_2, SWITCH_1]),
            (LIVING_ROOM, vec![THERMOMETER_1, SOCKET_1, SWITCH_2]),
            (BEDROOM, vec![THERMOMETER_2, SWITCH_1, SWITCH_2]),
        ]),
    )
}
