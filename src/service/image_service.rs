use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use crate::domain::error::DomainResult;
use crate::domain::param::file_storage_service_param::StoreParam;
use crate::domain::param::image_service_param::*;
use crate::domain::param::optimizer_service_param::ProcessParam;
use crate::domain::queue::image_queue::ImageQueue;
use crate::domain::service::{FileStorageService, ImageService, OptimizerService};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ImageServiceImpl {
    pub image_queue: Arc<dyn ImageQueue>,
    pub optimizer_service: Arc<dyn OptimizerService>,
    pub file_storage_service: Arc<dyn FileStorageService>
}

impl ImageServiceImpl {
    fn image_new_name(&self, name: String) -> String {
        let t_stamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let ext_buf =PathBuf::from(&name); 
        let ext = ext_buf.extension().and_then(|e| e.to_str()).unwrap_or("");
        
        let new_name = format!("{}.{}", t_stamp, ext);
        
        new_name
    }
}

#[async_trait]
impl ImageService for ImageServiceImpl {
    async fn optimize_image(&self, param: OptimizeImageParam) -> DomainResult<OptimizeImageResult> {
        
        let res = self.optimizer_service.process(ProcessParam {
            image: param.image.clone(),
            specification: param.specification
        }).await?;


        let s_param = StoreParam {
            data: res.data,
            name: self.image_new_name(param.image.full_name)
        };

        self.file_storage_service.store(s_param).await?;
        
        Ok(OptimizeImageResult {})
    }

    async fn store_img_info(
        &self,
        param: StoreImageInfoParam,
    ) -> DomainResult<StoreImageInfoResult> {
        let opt_image_param = OptimizeImageParam {
            image: param.image,
            specification: param.specification,
        };

        self.image_queue.push_image(opt_image_param).await?;

        DomainResult::Ok(StoreImageInfoResult {})
    }
}
