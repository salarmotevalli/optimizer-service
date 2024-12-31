use std::sync::Arc;

use crate::domain::error::{DomainErr, DomainResult, ErrKind};
use crate::domain::param::authorization_service_param::*;
use crate::domain::param::token_service_param::{GenerateTokenParam, VerifyTokenParam};
use crate::domain::service::{AuthorizationService, TokenService};
use async_trait::async_trait;

#[derive(Clone)]
pub struct AuthorizationServiceImpl {
    pub token_service: Arc<dyn TokenService>,
}

#[async_trait]
impl AuthorizationService for AuthorizationServiceImpl {
    async fn generate_sign_url_token(
        &self,
        param: GenerateSignUrlTokenParam,
    ) -> DomainResult<GenerateSignUrlTokenResult> {
        let token = self
            .token_service
            .generate_token(GenerateTokenParam {
                expire_time: 60 * 60 * 24,
                image_name: param.image_name,
                image_ext: param.image_ext,
                image_size: param.image_size,
            })
            .await?;

        DomainResult::Ok(GenerateSignUrlTokenResult { token: token.token })
    }

    async fn authorize_image_upload(
        &self,
        param: AuthorizeImageUploadParam,
    ) -> DomainResult<AuthorizeImageUploadResult> {
        return DomainResult::Ok(AuthorizeImageUploadResult { authorized: true });

        self.token_service
            .verify_token(VerifyTokenParam {
                token: "test".to_string(),
                image: param.image,
            })
            .await;

        return Err(DomainErr::new(
            "invalid token".to_string(),
            ErrKind::UnAuthorizedErr,
        ));
    }
}
