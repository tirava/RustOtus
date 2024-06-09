use crate::smart_house::SmartHouseError;
use async_trait::async_trait;
use atomic_enum::atomic_enum;
use std::fmt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub mod prelude {
    pub use crate::smart_device::DeviceStatus;
    pub use crate::smart_device::SmartDevice;
    pub use crate::smart_socket::SmartSocket;
    pub use crate::smart_switch::SmartSwitch;
    pub use crate::smart_thermometer::SmartThermometer;
}

#[atomic_enum]
pub enum DeviceStatus {
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

#[async_trait]
pub trait SmartDevice {
    async fn listen(&'static self, addr: &str) -> Result<(), SmartHouseError> {
        let listener = TcpListener::bind(addr).await?;
        println!(
            "SMART_DEVICE: {}: TCP listening on {}...",
            self.name(),
            addr
        );

        loop {
            let (stream, peer_addr) = match listener.accept().await {
                Ok((stream, peer_addr)) => (stream, peer_addr),
                Err(err) => {
                    eprintln!("SMART_DEVICE: stream error: {err}");
                    continue;
                }
            };
            println!("SMART_DEVICE: connected client: {peer_addr}");

            tokio::spawn(async move {
                self.handle_connection(stream).await;
                println!("SMART_DEVICE: disconnected client: {peer_addr}");
            });
        }
    }

    async fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let command = match buf_reader.lines().next_line().await {
            Ok(command) => match command {
                Some(command) => command,
                None => {
                    eprintln!("SMART_DEVICE: no command received");
                    return;
                }
            },
            Err(err) => {
                eprintln!("SMART_DEVICE: read command error: {err}");
                return;
            }
        };

        let result = format!("SMART_DEVICE: received command: {command}");
        println!("{result}");
        let result = self.exec_command(&command);
        println!("'{}'", result);

        let write_result = stream.write_all(result.as_bytes()).await;
        if write_result.is_err() {
            eprintln!("SMART_DEVICE: write error: {}", write_result.unwrap_err());
        }
    }

    async fn send_command(addr: &str, command: &str) -> Result<String, SmartHouseError> {
        println!(
            "SMART_DEVICE: connecting to address '{}' with command '{}'...",
            addr, command
        );

        match TcpStream::connect(addr).await {
            Ok(mut stream) => {
                let command = format!("{}\n", command);
                stream.write_all(command.as_bytes()).await?;

                let mut data = String::new();
                match stream.read_to_string(&mut data).await {
                    Ok(_) => Ok(data),
                    Err(err) => Err(SmartHouseError::from(err)),
                }
            }
            Err(err) => Err(SmartHouseError::from(err)),
        }
    }

    fn name(&self) -> &str;

    fn exec_command(&self, _command: &str) -> String {
        String::from("OK")
    }
}
