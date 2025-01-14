use async_trait::async_trait;

use crate::domain::{error::DomainResult, param::image_service_param::*};

#[async_trait]
pub trait ImageQueue: Send + Sync {
    async fn push_image(&self, param: OptimizeImageParam) -> DomainResult<()>;
    async fn push_process_result(&self, param: ProcessResultParam) -> DomainResult<()>;
}
