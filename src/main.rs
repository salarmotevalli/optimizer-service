use std::sync::Arc;

use serviceorented::{
    api::http::serve,
    app_config,
    container::Container,
    service::{
        authorization_service::{AuthorizationConfig, AuthorizationServiceImpl},
        image_service::ImageServiceImpl,
        jwt_token_service::{JwtTokenConfig, JwtTokenService},
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo read app name from env
    let cnf = app_config::load("siagoosh".to_string());

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    std::fs::create_dir_all(&cnf.file_temp_dir)?;

    let token_service = Arc::new(JwtTokenService {
        config: cnf.token_service_config.clone(),
    });

    let authorization_service = Arc::new(AuthorizationServiceImpl {
        token_service: token_service.clone(),
        config: cnf.authorization_service_config.clone(),
    });
    
    let image_service = Arc::new(ImageServiceImpl {});

    serve(Arc::new(Container::new(
        cnf,
        authorization_service,
        token_service,
        image_service,
    )))
    .await
}
