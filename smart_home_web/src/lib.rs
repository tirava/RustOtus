mod device_info_provider;
mod http_handler;
mod http_server;
pub mod smart_device;
mod smart_house;
mod smart_socket;
mod smart_switch;
mod smart_thermometer;

pub mod prelude {
    pub use crate::device_info_provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
    pub use crate::http_server::HTTPServer;
    pub use crate::smart_device::prelude::*;
    pub use crate::smart_house::{SmartHouse, SmartHouseError};
}
