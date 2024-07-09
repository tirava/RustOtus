use smart_home_web::prelude::*;
use std::collections::HashMap;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), SmartHouseError> {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    let bind_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8000".to_string());
    let workers = env::var("WORKERS").unwrap_or(2.to_string()).parse()?;

    let house = SmartHouse::new(
        "Мой умный дом".to_string(),
        "ул. Умных домов, д.1, кв.2".to_string(),
        HashMap::from([
            ("KITCHEN", &["SOCKET", "THERMOMETER", "SWITCH"][..]),
            ("LIVING_ROOM", &[]),
            ("BEDROOM", &[]),
        ]),
    );

    HTTPServer::new(bind_address, log_level, workers, AppData::new(house))
        .start()
        .await?;

    Ok(())
}
