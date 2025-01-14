use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::error::{DomainErr, DomainResult, ErrKind};
use crate::domain::param::token_service_param::*;
use crate::domain::service::TokenService;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtTokenConfig {
    pub secret: String,
    pub expiration_time: u64,
}

pub struct TokenServiceJWTImpl {
    pub config: JwtTokenConfig,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    size: usize,
    name: String,
    user_id: u32,
    ext: String,
}

impl TokenService for TokenServiceJWTImpl {
    fn generate_token(&self, param: GenerateTokenParam) -> DomainResult<GenerateTokenResult> {
        let exp =
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + self.config.expiration_time;

        let c = Claims {
            size: param.image_size,
            name: param.image_name,
            ext: param.image_format,
            user_id: param.user_id,
            exp: exp as usize,
        };

        let token = encode(
            &Header::default(),
            &c,
            &EncodingKey::from_secret(self.config.secret.as_ref()),
        )?;

        Ok(GenerateTokenResult { token })
    }

    fn verify_token(&self, param: VerifyTokenParam) -> DomainResult<VerifyTokenResult> {
        let token = decode::<Claims>(
            &param.token,
            &DecodingKey::from_secret(self.config.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                DomainErr::new(e.to_string(), ErrKind::UnAuthorizedErr)
            }
            _ => DomainErr::new(e.to_string(), ErrKind::UnExpectedErr),
        })?;

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;

        if token.claims.exp < now {
            return DomainResult::Err(DomainErr::new(
                "Token is expired".to_string(),
                ErrKind::UnAuthorizedErr,
            ));
        }

        if token.claims.name != param.image.full_name
            || token.claims.ext != param.image.ext()
            || token.claims.size != param.image.size
        {
            return DomainResult::Err(DomainErr::new(
                "Token claims do not match the provided image details".to_string(),
                ErrKind::UnAuthorizedErr,
            ));
        }

        DomainResult::Ok(VerifyTokenResult {user_id: token.claims.user_id})
    }
}
