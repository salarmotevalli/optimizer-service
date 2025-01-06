use async_trait::async_trait;

use crate::domain::{error::DomainResult, param::image_service_param::OptImgParam};

#[async_trait]
pub trait OptimizerService: Send + Sync {
    fn process(&self,  param: OptImgParam) -> DomainResult<()>;
}
