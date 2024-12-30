mod error_mapper;

use std::sync::Arc;

use crate::domain::{
    entity::{self, image::Image},
    error::{DomainErr, ErrKind},
    param::{
        image_service_param::{OptImgParam, OptImgResult},
        sign_url_service_param::{GenerateSignUrlParam, GenerateSignUrlResult},
        token_service_param::VerifyTokenParam,
    },
    service::{ImageService, SignUrlService, TokenService},
};

use actix_multipart::form::{
    MultipartForm,
    json::Json,
    tempfile::{TempFile, TempFileConfig},
};
use actix_web::{
    App, Error, HttpServer,
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    middleware::Logger,
    web,
};

use serde::Deserialize;

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub file_path: String,
    pub image_white_list: Vec<String>,
}

#[derive(Clone)]
pub struct Container {
    pub sign_url_service: Arc<dyn SignUrlService>,
    pub token_service: Arc<dyn TokenService>,
    pub image_service: Arc<dyn ImageService>,
    pub config: Config,
}

// TODO: move to a separate file
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
        .app_data(TempFileConfig::default().directory("./tmp"))
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .service(
            web::scope("/opt")
                .route("/sign-url", web::post().to(sign_url))
                .route("/upload", web::post().to(upload_image)),
        )
}

pub async fn serve(container: Arc<Container>) -> std::io::Result<()> {
    HttpServer::new(move || create_app(container.clone()))
        // TODO: make it dynamic
        .workers(1)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "5MB")]
    file: TempFile,
    params: Json<UploadImageParam>,
}

#[derive(Deserialize, Debug)]
struct UploadImageParam {
    specification: entity::image_specification::ImageSpecification,
}

async fn upload_image(
    image_service: web::Data<dyn ImageService>,
    token_service: web::Data<dyn TokenService>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<web::Json<OptImgResult>, DomainErr> {
    let content_type = form.file.content_type;

    // TODO: also sign url service need to check file format.
    // TODO: check whit list - content_type.subtype() -
    if content_type.is_none() || content_type.unwrap().type_() != "image" {
        return Err(DomainErr::new(
            "invalid file type".to_string(),
            ErrKind::UnprocessableErr,
        ));
    }

    let mut opt_img_param = OptImgParam {
        image: Image::default(),
        specification: form.params.into_inner().specification,
    };

    let file_name = form.file.file_name.unwrap();
    let path = format!("./tmp/{}", file_name);
    opt_img_param.image.full_name = file_name.to_string();
    opt_img_param.image.size = form.file.size;

    let auth = token_service
        .into_inner()
        .verify_token(VerifyTokenParam {
            token: "test".to_string(),
            image: opt_img_param.image.clone(),
        })
        .await;

    if auth.is_err() {
        return Err(DomainErr::new(
            "invalid token".to_string(),
            ErrKind::UnAuthorizedErr,
        ));
    }

    form.file
        .file
        .persist(&path)
        .map_err(|e| DomainErr::new(e.to_string(), ErrKind::UnprocessableErr))?;

    let result = image_service.opt_img(opt_img_param).await?;

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
