use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::{fmt, io};

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
    fn listen(&self, addr: &str) -> Result<(), io::Error> {
        let listener = TcpListener::bind(addr)?;

        for stream in listener.incoming() {
            let mut stream = stream?;
            let buf_reader = BufReader::new(&mut stream);
            let command = buf_reader
                .lines()
                .next()
                .expect("не удалось получить команду")?;

            let result = self.exec_command(&command)?;
            println!("{}", result);
            stream.write_all(result.as_bytes())?
        }

        Ok(())
    }

    fn exec_command(&self, _command: &str) -> Result<String, io::Error> {
        Ok(String::from("OK"))
    }
}
