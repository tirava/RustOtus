use iced::{window, Application, Settings, Size};
use smart_home_dyn_lib::prelude::*;

fn main() -> Result<(), SmartHouseError> {
    // for listening TCP SmartSocket commands start server example before run gui
    SmartSocketGUI::run(Settings {
        window: window::Settings {
            size: Size::new(500.0, 600.0),
            position: window::Position::Centered,
            resizable: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })?;

    Ok(())
}
