use std::collections::HashMap;
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
        let token = self
            .token_service
            .generate_token(GenerateTokenParam {
                expire_time: 60 * 60 * 24,
                metadata: HashMap::from([
                    ("image_name".to_string(), param.image_name),
                    ("image_ext".to_string(), param.image_ext),
                    ("image_size".to_string(), param.image_size.to_string()),
                ]),
            })
            .await?;

        let url = format!("{}/upload?token={}", "self.config.base_url", token.token);
        DomainResult::Ok(GenerateSignUrlResult { url })
    }
}
