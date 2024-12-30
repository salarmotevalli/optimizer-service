use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::domain::error::{DomainErr, DomainResult, ErrKind};
use crate::domain::param::token_service_param::*;
use crate::domain::service::TokenService;
use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

const SECRET: &str = "test";

#[derive(Clone)]

pub struct TokenServiceImpl {}

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    size: usize,
    name: String,
    ext: String,
}

#[async_trait]
impl TokenService for TokenServiceImpl {
    async fn generate_token(&self, param: GenerateTokenParam) -> DomainResult<GenerateTokenResult> {
        let du = Duration::from_secs(12 * 60 * 60).as_secs();

        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + du;

        let c = Claims {
            size: param.image_size,
            name: param.image_name,
            ext: param.image_ext,
            exp: exp as usize,
        };

        // TODO: change the secret
        let token = encode(
            &Header::default(),
            &c,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
        .map_err(|e| DomainErr::new(e.to_string(), ErrKind::UnExpectedErr))?;

        Ok(GenerateTokenResult { token })
    }

    async fn verify_token(&self, param: VerifyTokenParam) -> DomainResult<VerifyTokenResult> {
        let token = decode::<Claims>(
            &param.token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| DomainErr::new(e.to_string(), ErrKind::UnExpectedErr))?;

        // TODO: check the token is expired
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if token.claims.exp < now {
            return DomainResult::Err(DomainErr::new(
                "Token is expired".to_string(),
                ErrKind::UnAuthorizedErr,
            ));
        }

        if token.claims.exp < now
            || token.claims.name != param.image.full_name
            || token.claims.ext != param.image.ext()
            || token.claims.size != param.image.size
        {
            return DomainResult::Err(DomainErr::new(
                "Token is invalid".to_string(),
                ErrKind::UnAuthorizedErr,
            ));
        }

        DomainResult::Ok(VerifyTokenResult {})
    }
}
