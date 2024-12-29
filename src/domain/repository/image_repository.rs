use async_trait::async_trait;

use crate::domain::entity::image::Image;
use crate::domain::error::DomainResult;

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn get_img(&self, id: u64) -> DomainResult<Image>;
}
