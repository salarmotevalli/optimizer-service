use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::domain::error::{DomainErr, DomainResult, ErrKind};
use crate::domain::param::authorization_service_param::*;
use crate::domain::param::token_service_param::{GenerateTokenParam, VerifyTokenParam};
use crate::domain::service::{AuthorizationService, TokenService};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    pub ext_white_list: Vec<String>,
}

pub struct AuthorizationServiceImpl {
    pub token_service: Arc<dyn TokenService>,
    pub config: AuthorizationConfig,
}

impl AuthorizationService for AuthorizationServiceImpl {
    fn generate_sign_url_token(
        &self,
        param: GenerateSignUrlTokenParam,
    ) -> DomainResult<GenerateSignUrlTokenResult> {
        let token = self.token_service.generate_token(GenerateTokenParam {
            image_name: param.image_name,
            image_ext: param.image_ext,
            image_size: param.image_size,
        })?;

        DomainResult::Ok(GenerateSignUrlTokenResult { token: token.token })
    }

    fn authorize_image_upload(
        &self,
        param: AuthorizeImageUploadParam,
    ) -> DomainResult<AuthorizeImageUploadResult> {
        let _is_token_verified = self.token_service.verify_token(VerifyTokenParam {
            token: param.token,
            image: param.image.clone(),
        })?;

        let _is_image_format_valid = self.authorize_image_format(AuthorizeImageFormatParam {
            ext: param.image.ext(),
        })?;

        DomainResult::Ok(AuthorizeImageUploadResult { authorized: true })
    }

    fn authorize_image_format(
        &self,
        param: AuthorizeImageFormatParam,
    ) -> DomainResult<AuthorizeImageFormatResult> {
        match self.config.ext_white_list.contains(&param.ext) {
            true => DomainResult::Ok(AuthorizeImageFormatResult {}),
            false => DomainResult::Err(DomainErr::new(
                "invalid image format".to_string(),
                ErrKind::Forbidden,
            )),
        }
    }
}
