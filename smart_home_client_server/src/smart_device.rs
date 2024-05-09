use crate::smart_house::SmartHouseError;
use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

pub mod prelude {
    pub use crate::smart_device::DeviceStatus;
    pub use crate::smart_device::SmartDevice;
    pub use crate::smart_socket::SmartSocket;
    pub use crate::smart_switch::SmartSwitch;
    pub use crate::smart_thermometer::SmartThermometer;
}

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

pub trait SmartDevice {
    fn listen(&mut self, addr: &str) -> Result<(), SmartHouseError> {
        let listener = TcpListener::bind(addr)?;
        println!("SMART_DEVICE: listening on {}...", addr);

        for stream in listener.incoming() {
            if stream.is_err() {
                eprintln!("SMART_DEVICE: stream error: {}", stream.unwrap_err());
                continue;
            }

            let stream = stream.unwrap();
            let peer_addr = stream.peer_addr()?;
            println!("SMART_DEVICE: connected client: {peer_addr}");

            self.handle_connection(stream);
            println!("SMART_DEVICE: disconnected client: {peer_addr}");
        }

        Ok(())
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let command = match buf_reader.lines().next() {
            Some(command) => match command {
                Ok(command) => command,
                Err(err) => {
                    eprintln!("SMART_DEVICE: read command error: {err}");
                    return;
                }
            },
            None => {
                eprintln!("SMART_DEVICE: no command received");
                return;
            }
        };

        let result = self.exec_command(&command);
        println!("'{}'", result);

        let write_result = stream.write_all(result.as_bytes());
        if write_result.is_err() {
            eprintln!("SMART_DEVICE: write error: {}", write_result.unwrap_err());
        }
    }

    fn send_command(addr: &str, command: &str) -> Result<String, SmartHouseError> {
        println!(
            "SMART_DEVICE: connecting to address '{}' with command '{}'...",
            addr, command
        );

        match TcpStream::connect(addr) {
            Ok(mut stream) => {
                let command = format!("{}\n", command);
                stream.write_all(command.as_bytes())?;

                let mut data = String::new();
                match stream.read_to_string(&mut data) {
                    Ok(_) => Ok(data),
                    Err(err) => Err(SmartHouseError::from(err)),
                }
            }
            Err(err) => Err(SmartHouseError::from(err)),
        }
    }

    fn exec_command(&mut self, _command: &str) -> String {
        String::from("OK")
    }
}
