use smart_home_gui::prelude::*;

const SOCKET_ADDR: &str = "127.0.0.1:54321";

#[tokio::main]
async fn main() -> Result<(), SmartHouseError> {
    tokio::spawn(async move {
        let _ = SmartSocket::new(
            "Розеточка".to_string(),
            "Комнатка".to_string(),
            DeviceStatus::Off,
            0.0,
        )
        .listen(SOCKET_ADDR)
        .await;
    });

    let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "on").await?;
    println!("CLIENT: SmartSocket command 'on' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);

    Ok(())
}
