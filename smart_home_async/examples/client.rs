use smart_home_async::prelude::*;

const SOCKET_ADDR: &str = "127.0.0.1:54321";
// const THERMOMETER_ADDR: &str = "127.0.0.1:12345";

#[tokio::main]
async fn main() -> Result<(), SmartHouseError> {
    let result = SmartSocket::send_command(SOCKET_ADDR, "info").await?;
    println!("CLIENT: SmartSocket command 'info' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "on").await?;
    println!("CLIENT: SmartSocket command 'on' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "power").await?;
    println!("CLIENT: SmartSocket command 'power' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "off").await?;
    println!("CLIENT: SmartSocket command 'off' - '{}'\n", result);
    let result = SmartSocket::send_command(SOCKET_ADDR, "power").await?;
    println!("CLIENT: SmartSocket command 'power' - '{}'\n", result);

    let result = SmartSocket::send_command(SOCKET_ADDR, "qqq").await?;
    println!("CLIENT: SmartSocket command 'qqq' - '{}'\n", result);

    // for _ in 0..10 {
    //     sleep(time::Duration::from_secs(1));
    //     let result = SmartThermometer::send_command(THERMOMETER_ADDR, "info")?;
    //     println!("CLIENT: SmartThermometer command 'info' - '{}'\n", result);
    // }

    Ok(())
}
