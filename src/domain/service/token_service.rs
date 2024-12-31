use async_trait::async_trait;

use crate::domain::{error::DomainResult, param::token_service_param::*};

pub trait TokenService: Send + Sync {
    fn generate_token(&self, param: GenerateTokenParam) -> DomainResult<GenerateTokenResult>;
    fn verify_token(&self, param: VerifyTokenParam) -> DomainResult<VerifyTokenResult>;
}
