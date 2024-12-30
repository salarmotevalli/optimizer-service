use std::sync::Arc;

use crate::domain::error::DomainResult;
use crate::domain::param::sign_url_service_param::*;
use crate::domain::param::token_service_param::GenerateTokenParam;
use crate::domain::service::{SignUrlService, TokenService};
use async_trait::async_trait;

#[derive(Clone)]
pub struct SignUrlServiceImpl {
    pub token_service: Arc<dyn TokenService>,
}

#[async_trait]
impl SignUrlService for SignUrlServiceImpl {
    async fn generate_sign_url(
        &self,
        param: GenerateSignUrlParam,
    ) -> DomainResult<GenerateSignUrlResult> {
        println!("request is here in sign url service");

        let token = self
            .token_service
            .generate_token(GenerateTokenParam {
                expire_time: 60 * 60 * 24,
                image_name: param.image_name,
                image_ext: param.image_ext,
                image_size: param.image_size,
            })
            .await?;

        let url = format!(
            "{}/opt/upload?token={}",
            "self.config.base_url", token.token
        );
        DomainResult::Ok(GenerateSignUrlResult { url })
    }
}
