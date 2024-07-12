use crate::prelude::SmartHouseError;
use crate::smart_house_storage::prelude::*;
use async_trait::async_trait;

pub mod prelude {
    pub use crate::smart_house_storage::SmartHouseStorage;
    pub use crate::smart_house_storage_memory::SmartHouseStorageMemory;
    pub use crate::smart_house_storage_mock::MockDeviceInfoProvider;
    pub use crate::smart_house_storage_mongodb::SmartHouseStorageMongoDB;
}

#[async_trait]
pub trait SmartHouseStorage {
    async fn rooms(&self) -> Result<Vec<String>, SmartHouseError>;

    async fn add_room(&self, room: &str) -> Result<(), SmartHouseError>;

    async fn remove_room(&self, room: &str) -> Result<(), SmartHouseError>;

    async fn devices(&self, room: &str) -> Result<Vec<String>, SmartHouseError>;

    async fn add_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError>;

    async fn remove_device(&self, room: &str, device: &str) -> Result<(), SmartHouseError>;
}

#[async_trait]
pub trait SmartHouseDeviceStorage: SmartHouseStorage + MockDeviceInfoProvider {}

#[async_trait]
impl SmartHouseDeviceStorage for SmartHouseStorageMemory {}

#[async_trait]
impl SmartHouseDeviceStorage for SmartHouseStorageMongoDB {}
