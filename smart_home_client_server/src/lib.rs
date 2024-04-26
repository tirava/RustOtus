mod device_info_provider;
mod smart_device;
mod smart_house;
mod smart_socket;
mod smart_switch;
mod smart_thermometer;

pub mod prelude {
    pub use crate::device_info_provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
    pub use crate::smart_device::prelude::*;
    pub use crate::smart_house::{SmartHouse, SmartHouseError};
}
