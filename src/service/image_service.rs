use crate::domain::error::DomainResult;
use crate::domain::param::image_service_param::*;
use crate::domain::service::ImageService;
use async_trait::async_trait;

#[derive(Clone)]
pub struct ImageServiceImpl {}

#[async_trait]
impl ImageService for ImageServiceImpl {
    async fn opt_img(&self, param: OptImgParam) -> DomainResult<OptImgResult> {
        DomainResult::Ok(OptImgResult { image: param.image })
    }

    async fn store_img_info(
        &self,
        param: StoreImageInfoParam,
    ) -> DomainResult<StoreImageInfoResult> {
        DomainResult::Ok(StoreImageInfoResult {})
    }
}
