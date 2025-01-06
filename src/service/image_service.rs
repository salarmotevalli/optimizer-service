use std::fmt::format;
use std::sync::Arc;

use crate::domain::entity::image::Image;
use crate::domain::error::DomainResult;
use crate::domain::param::image_service_param::*;
use crate::domain::queue::image_queue::ImageQueue;
use crate::domain::service::{ImageService, OptimizerService};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ImageServiceImpl {
    pub image_queue: Arc<dyn ImageQueue>,
    // pub optimizer_service: Arc<dyn OptimizerService>,
}

#[async_trait]
impl ImageService for ImageServiceImpl {
    async fn opt_img(&self, param: OptImgParam) -> DomainResult<OptImgResult> {
        // self.optimizer_service.process().await?;

        Ok(OptImgResult {
            image: Image::default(),
        })
    }

    async fn store_img_info(
        &self,
        param: StoreImageInfoParam,
    ) -> DomainResult<StoreImageInfoResult> {
        let opt_image_param = OptImgParam {
            image_path: format!("{}/{}", "temp", param.image.full_name),
            specification: param.specification,
        };

        self.image_queue.push_image(opt_image_param).await?;

        DomainResult::Ok(StoreImageInfoResult {})
    }
}
