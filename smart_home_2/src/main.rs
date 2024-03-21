use smart_home_2::*;

fn main() {
    // Инициализация дома
    let mut home = SmartHome::new("Умный дом".to_string(), "Адрес дома, кв.1".to_string());

    // Инициализация помещений
    let kitchen = Room::new("Кухня".to_string());
    let bedroom1 = Room::new("Гостинная".to_string());
    let bedroom2 = Room::new("Спальня".to_string());
    if let Err(err) = home.set_rooms(vec![kitchen, bedroom1, bedroom2]) {
        println!("{err}")
    }

    for room in home.get_rooms() {
        println!("{room}");
    }

    // // Инициализация устройств
    // let socket1 = SmartSocket {};
    // let socket2 = SmartSocket {};
    // let thermo = SmartThermometer {};

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
}
