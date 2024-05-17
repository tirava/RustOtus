use crate::prelude::SmartHouseError;
use crate::smart_device::SmartDevice;
use std::fmt;
use std::net::UdpSocket;

pub struct SmartThermometer {
    pub(crate) name: String,
    pub(crate) room: String,
    pub temp: f32,
}

impl SmartThermometer {
    pub fn new(name: String, room: String, temp: f32) -> Self {
        Self { name, room, temp }
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "температура - {:.2} °С", self.temp)
    }
}

impl SmartDevice for SmartThermometer {
    fn listen(&mut self, addr: &str) -> Result<(), SmartHouseError> {
        let socket = UdpSocket::bind(addr)?;
        println!("SMART_THERMOMETER: UDP listening on {}...", addr);

        let mut buf = [0; 128];
        loop {
            match socket.recv_from(&mut buf) {
                Ok((len, src)) => {
                    println!("SMART_THERMOMETER: received a datagram from client: {src}");

                    let command = String::from_utf8_lossy(&buf[0..len]);
                    let result = self.exec_command(&command);
                    println!("'{}'", result);

                    match socket.send_to(result.as_bytes(), src) {
                        Ok(_) => (),
                        Err(err) => {
                            eprintln!("SMART_THERMOMETER: couldn't send a datagram: {}", err);
                            continue;
                        }
                    }
                    println!("SMART_DEVICE: sent a datagram to client: {src}");
                }

                Err(err) => {
                    eprintln!("SMART_THERMOMETER: couldn't receive a datagram: {}", err);
                }
            }
        }
    }

    fn send_command(addr: &str, command: &str) -> Result<String, SmartHouseError> {
        println!(
            "SMART_THERMOMETER: connecting to address '{}' with command '{}'...",
            addr, command
        );
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        match socket.send_to(command.as_bytes(), addr) {
            Ok(_) => (),
            Err(err) => return Err(SmartHouseError::from(err)),
        }

        let mut buf = [0; 128];
        match socket.recv_from(&mut buf) {
            Ok((len, _)) => Ok(String::from_utf8_lossy(&buf[0..len]).to_string()),
            Err(err) => Err(SmartHouseError::from(err)),
        }
    }

    fn exec_command(&mut self, command: &str) -> String {
        print!("SMART_THERMOMETER: command '{command}' -> ");

        match command {
            "info" => {
                format!(
                    "name: {}, room: {}, temperature: {:.2} °С",
                    self.name, self.room, self.temp
                )
            }
            _ => match command.parse::<f32>() {
                Ok(value) => {
                    self.temp = value;
                    format!("{:.2}", self.temp)
                }
                Err(_) => "unknown command".to_string(),
            },
        }
    }
}
