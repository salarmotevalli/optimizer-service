use crate::domain::{error::DomainResult, param::authorization_service_param::*};
use async_trait::async_trait;

#[async_trait]
pub trait AuthorizationService: Send + Sync {
    fn generate_sign_url_token(
        &self,
        param: GenerateSignUrlTokenParam,
    ) -> DomainResult<GenerateSignUrlTokenResult>;

    fn authorize_image_upload(
        &self,
        param: AuthorizeImageUploadParam,
    ) -> DomainResult<AuthorizeImageUploadResult>;

    fn authorize_image_format(
        &self,
        param: AuthorizeImageFormatParam,
    ) -> DomainResult<AuthorizeImageFormatResult>;
}
