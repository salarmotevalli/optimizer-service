use std::sync::Arc;

use serviceorented::{
    api::http::{Config, Container},
    service::{
        image_service::ImageServiceImpl, sign_url_service::SignUrlServiceImpl,
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
    let sign_url_service = Arc::new(SignUrlServiceImpl {
        token_service: token_service.clone(),
    });
    let image_service = Arc::new(ImageServiceImpl {});

    serve(Arc::new(Container {
        sign_url_service,
        token_service,
        image_service,
        config: cnf,
    }))
    .await
}
