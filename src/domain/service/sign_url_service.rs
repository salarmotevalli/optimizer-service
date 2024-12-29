use crate::domain::{error::DomainResult, param::sign_url_service_param::*};
use async_trait::async_trait;

#[async_trait]
pub trait SignUrlService: Send + Sync {
    async fn generate_sign_url(
        &self,
        param: GenerateSignUrlParam,
    ) -> DomainResult<GenerateSignUrlResult>;
}
