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

    let token_service = Arc::new(TokenServiceImpl {});
    let sign_url_service = Arc::new(SignUrlServiceImpl {
        token_service: token_service.clone(),
    });
    let image_service = Arc::new(ImageServiceImpl {});

    serve(Arc::new(Container {
        sign_url_service,
        token_service,
        image_service,
        config: Config { port: 8080 },
    }))
    .await
}
