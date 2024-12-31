use std::sync::Arc;

use serviceorented::{
    api::http::{Config, Container},
    service::{
        authorization_service::{AuthorizationServiceImpl, SignUrlServiceImpl},
        image_service::ImageServiceImpl,
        token_service::TokenServiceImpl,
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use serviceorented::api::http::serve;
    let cnf = Config {
        port: 8080,
        file_path: "./tmp".to_string(),
        image_white_list: vec!["png".to_string(), "jpg".to_string(), "jpeg".to_string()],
    };

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // TODO: make it dynamic
    std::fs::create_dir_all(&cnf.file_path)?;

    let token_service = Arc::new(TokenServiceImpl {});
    let authorization_service = Arc::new(AuthorizationServiceImpl {
        token_service: token_service.clone(),
    });
    let image_service = Arc::new(ImageServiceImpl {});

    serve(Arc::new(Container {
        authorization_service,
        token_service,
        image_service,
        config: cnf,
    }))
    .await
}
