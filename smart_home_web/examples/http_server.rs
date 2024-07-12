use smart_home_web::prelude::*;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), SmartHouseError> {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    let bind_address = env::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8000".to_string());
    let workers = env::var("WORKERS").unwrap_or(2.to_string()).parse()?;

    let app_data = match env::var("MONGO_URI") {
        Ok(uri) => AppData::new(Box::new(
            SmartHouseStorageMongoDB::new(uri).connect().await?,
        )),
        Err(_) => AppData::new(Box::new(SmartHouseStorageMemory::new())),
    };

    // app_data.storage.init().await?;

    HTTPServer::new(bind_address, log_level, workers, app_data)
        .start()
        .await?;

    Ok(())
}
