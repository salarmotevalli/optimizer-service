use async_trait::async_trait;

use crate::domain::{error::DomainResult, param::token_service_param::*};

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn generate_token(&self, param: GenerateTokenParam) -> DomainResult<GenerateTokenResult>;
    async fn verify_token(&self, param: VerifyTokenParam) -> DomainResult<VerifyTokenResult>;
}
