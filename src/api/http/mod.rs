mod error_mapper;

use crate::domain::{
    error::DomainErr,
    param::{
        image_service_param::{OptImgParam, OptImgResult},
        sign_url_service_param::{GenerateSignUrlParam, GenerateSignUrlResult},
    },
    service::{ImageService, SignUrlService, TokenService},
};
use std::sync::Arc;

use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::Logger,
    web,
};

#[derive(Clone)]
pub struct Config {
    pub port: u16,
}

#[derive(Clone)]
pub struct Container {
    pub sign_url_service: Arc<dyn SignUrlService>,
    pub token_service: Arc<dyn TokenService>,
    pub image_service: Arc<dyn ImageService>,
    pub config: Config,
}

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
    let image_service = container.image_service.clone();
    let sign_url_service = container.sign_url_service.clone();

    App::new()
        .app_data(web::Data::from(image_service.clone()))
        .app_data(web::Data::from(sign_url_service.clone()))
        .wrap(Logger::default())
        .service(
            web::scope("/opt")
                .route("/sign-url", web::post().to(sign_url))
                .route("/upload", web::post().to(upload_image)),
        )
}

pub async fn serve(container: Arc<Container>) -> std::io::Result<()> {
    HttpServer::new(move || create_app(container.clone()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn upload_image(
    image_service: web::Data<dyn ImageService>,
    param: web::Json<OptImgParam>,
) -> Result<web::Json<OptImgResult>, DomainErr> {
    let result = image_service.opt_img(param.into_inner()).await?;
    Ok(web::Json(result))
}

async fn sign_url(
    sign_url_service: web::Data<dyn SignUrlService>,
    param: web::Json<GenerateSignUrlParam>,
) -> Result<web::Json<GenerateSignUrlResult>, DomainErr> {
    let result = sign_url_service
        .generate_sign_url(param.into_inner())
        .await?;
    Ok(web::Json(result))
}
