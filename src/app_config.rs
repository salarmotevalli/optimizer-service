// pub struct AppConfig {}

use figment::providers::{Env, Format, Yaml};
use serde::{Deserialize, Serialize};
use crate::service::{authorization_service, jwt_token_service};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub token_service_config: jwt_token_service::JwtTokenConfig,
    pub authorization_service_config: authorization_service::AuthorizationConfig,
}

pub fn load(app_name: String) -> Config {
    let prefix = format!("{}_", app_name.to_uppercase());
    
    let config: Config= figment::Figment::new()
    .merge(Yaml::file("./config.yml"))
    .merge(Env::prefixed(&prefix).split("__").map(|i| i.to_string().to_lowercase().into()))
    .extract().unwrap();

    config
}
