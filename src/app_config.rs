// pub struct AppConfig {}

use crate::{
    api::http::HttpServerConfig,
    service::{authorization_service, jwt_token_service},
};
use figment::providers::{Env, Format, Yaml};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    // root config
    pub app_name: String,
    pub file_temp_dir: String,

    pub http_server_config: HttpServerConfig,
    pub token_service_config: jwt_token_service::JwtTokenConfig,
    pub authorization_service_config: authorization_service::AuthorizationConfig,
}

pub fn load(app_name: String) -> Config {
    let prefix = format!("{}_", app_name.to_uppercase());

    let yaml = Yaml::file("./config.yml");
    let env = Env::prefixed(&prefix).split("__");

    let config: Config = figment::Figment::new()
        // default values
        .join(("app_name", app_name))
        .join(("file_temp_dir", "./tmp".to_string()))
        .merge(yaml)
        .merge(env)
        .extract()
        .unwrap();

    config
}
