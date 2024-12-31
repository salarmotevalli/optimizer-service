use crate::domain::{error::DomainResult, param::authorization_service_param::*};
use async_trait::async_trait;

#[async_trait]
pub trait AuthorizationService: Send + Sync {
    async fn generate_sign_url_token(
        &self,
        param: GenerateSignUrlTokenParam,
    ) -> DomainResult<GenerateSignUrlTokenResult>;

    async fn authorize_image_upload(
        &self,
        param: AuthorizeImageUploadParam,
    ) -> DomainResult<AuthorizeImageUploadResult>;
}
