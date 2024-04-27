use smart_home_client_server::prelude::*;

fn main() -> Result<(), SmartHouseError> {
    SmartSocket::listen("127.0.0.1:8181")?;
    SmartThermometer::listen("127.0.0.1:8282")?;
    SmartSwitch::listen("127.0.0.1:8383")?;

    Ok(())
}
