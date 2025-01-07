use async_trait::async_trait;

use crate::domain::{error::DomainResult, param::file_storage_service_param::*};

#[async_trait]
pub trait FileStorageService: Send + Sync {
    async fn store(&self, param: StoreParam) -> DomainResult<StoreResult>;
}
