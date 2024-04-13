mod device_info_provider;
mod smart_device;
mod smart_house;

pub mod prelude {
    pub use crate::device_info_provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
    pub use crate::smart_device::{DeviceStatus, SmartSocket, SmartSwitch, SmartThermometer};
    pub use crate::smart_house::{SmartHouse, SmartHouseError};
}
