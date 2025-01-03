use async_trait::async_trait;

use crate::domain::{error::DomainResult, param::image_service_param::OptImgParam};

#[async_trait]
pub trait ImageQueue: Send + Sync {
    async fn push_image(&self, param: OptImgParam) -> DomainResult<()>;
}
