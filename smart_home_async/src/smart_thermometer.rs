use crate::prelude::SmartHouseError;
use crate::smart_device::SmartDevice;
use async_trait::async_trait;
use atomic_float::AtomicF32;
use std::fmt;
use std::sync::atomic::Ordering::SeqCst;
use tokio::net::UdpSocket;

pub struct SmartThermometer {
    pub(crate) name: String,
    pub(crate) room: String,
    pub temp: AtomicF32,
}

impl SmartThermometer {
    pub fn new(name: String, room: String, temp: f32) -> &'static Self {
        Box::leak(Box::new(Self {
            name,
            room,
            temp: AtomicF32::new(temp),
        }))
    }
}

impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "температура - {:.2} °С", self.temp.load(SeqCst))
    }
}

#[async_trait]
impl SmartDevice for SmartThermometer {
    async fn listen(&'static self, addr: &str) -> Result<(), SmartHouseError> {
        let socket = UdpSocket::bind(addr).await?;
        println!("SMART_THERMOMETER: UDP listening on {}...", addr);

        let mut buf = [0; 128];
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, src)) => {
                    println!("SMART_THERMOMETER: received a datagram from client: {src}");

                    let command = String::from_utf8_lossy(&buf[0..len]);
                    let result = self.exec_command(&command);
                    println!("'{}'", result);

                    match socket.send_to(result.as_bytes(), src).await {
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

    async fn send_command(addr: &str, command: &str) -> Result<String, SmartHouseError> {
        println!(
            "SMART_THERMOMETER: connecting to address '{}' with command '{}'...",
            addr, command
        );
        let socket = UdpSocket::bind("0.0.0.0:0").await?;

        match socket.send_to(command.as_bytes(), addr).await {
            Ok(_) => (),
            Err(err) => return Err(SmartHouseError::from(err)),
        }

        let mut buf = [0; 128];
        match socket.recv_from(&mut buf).await {
            Ok((len, _)) => Ok(String::from_utf8_lossy(&buf[0..len]).to_string()),
            Err(err) => Err(SmartHouseError::from(err)),
        }
    }

    fn exec_command(&self, command: &str) -> String {
        print!("SMART_THERMOMETER: command '{command}' -> ");

        match command {
            "info" => {
                format!(
                    "name: {}, room: {}, temperature: {:.2} °С",
                    self.name,
                    self.room,
                    self.temp.load(SeqCst)
                )
            }
            _ => match command.parse::<f32>() {
                Ok(value) => {
                    self.temp.store(value, SeqCst);
                    format!("{:.2}", self.temp.load(SeqCst))
                }
                Err(_) => "unknown command".to_string(),
            },
        }
    }
}
