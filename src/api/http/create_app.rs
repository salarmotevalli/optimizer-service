use std::sync::Arc;

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{
    App, Error,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::Logger,
    web,
};

use super::*;

pub fn create_app(
    container: Arc<Container>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    App::new()
        .app_data(web::Data::from(container.clone()))
        .app_data(TempFileConfig::default().directory("./tmp"))
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .service(
            web::scope("/opt")
                .route("/sign-url", web::post().to(sign_url))
                .route("/upload", web::post().to(upload_image)),
        )
}
