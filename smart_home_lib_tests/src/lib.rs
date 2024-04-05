mod device_info_provider;
mod smart_house;

pub mod prelude {
    pub use crate::device_info_provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
    pub use crate::smart_house::{
        DeviceStatus, SmartHouse, SmartSocket, SmartSwitch, SmartThermometer,
    };
}
