use crate::domain::error::DomainResult;
use crate::domain::param::token_service_param::*;
use crate::domain::service::TokenService;
use async_trait::async_trait;

#[derive(Clone)]
pub struct TokenServiceImpl {}

#[async_trait]
impl TokenService for TokenServiceImpl {
    async fn generate_token(&self, param: GenerateTokenParam) -> DomainResult<GenerateTokenResult> {
        todo!()
    }

    async fn verify_token(&self, param: VerifyTokenParam) -> DomainResult<VerifyTokenResult> {
        todo!()
    }
}
