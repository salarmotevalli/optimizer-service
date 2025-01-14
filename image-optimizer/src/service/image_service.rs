use std::collections::HashMap;
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
        let process_res = self.optimizer_service.process(ProcessParam {
            image: param.image.clone(),
            specification: param.specification
        }).await;

        
        if let Err(e) = process_res {
            let mut err: HashMap<String, String> = HashMap::new();          
            err.insert("message".to_string(), e.message.clone());
            
            let param = ProcessResultParam {
                image: param.image,
                err: Some(err)
            };

            let _push_res_to_queue = self.image_queue.push_process_result(param).await?;

            return Err(e);
        }


        let s_param = StoreParam {
            data: process_res.unwrap().data,
            name: self.image_new_name(param.image.full_name.clone())
        };

        let u_res = self.file_storage_service.store(s_param).await;
        if let Err(e) = u_res {
            let mut err: HashMap<String, String> = HashMap::new();          
            err.insert("message".to_string(), e.message.clone());
            
            let param = ProcessResultParam {
                image: param.image,
                err: Some(err)
            };

            // which error should return?
            // TODO: add log then return u_res err
            let _push_res_to_queue = self.image_queue.push_process_result(param).await?;

            return Err(e);
        }


        let param = ProcessResultParam {
            image: param.image,
            err: None
        };
        
        let _push_res_to_queue = self.image_queue.push_process_result(param).await?;

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
