use async_trait::async_trait;

use crate::domain::{error::DomainResult, param::image_service_param::*};

#[async_trait]
pub trait ImageService: Send + Sync {
    async fn opt_img(&self, param: OptImgParam) -> DomainResult<OptImgResult>;
    async fn store_img_info(
        &self,
        param: StoreImageInfoParam,
    ) -> DomainResult<StoreImageInfoResult>;
}
