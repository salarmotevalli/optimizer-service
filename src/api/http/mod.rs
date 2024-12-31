mod create_app;
mod error_mapper;

pub use create_app::create_app;

use std::sync::Arc;

use crate::{
    domain::{
        entity,
        error::{DomainErr, ErrKind},
        param::{authorization_service_param::*, image_service_param::*},
        service::*,
    },
    validation::upload_image_validation::UploadImageValidation,
};

use actix_multipart::form::{MultipartForm, json::Json, tempfile::TempFile};
use actix_web::{HttpServer, web};

#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub file_path: String,
    pub image_white_list: Vec<String>,
}

#[derive(Clone)]
pub struct Container {
    pub authorization_service: Arc<dyn AuthorizationService>,
    pub token_service: Arc<dyn TokenService>,
    pub image_service: Arc<dyn ImageService>,
    pub config: Config,
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
pub struct UploadForm {
    #[multipart(limit = "5MB")]
    pub file: TempFile,
    pub params: Json<entity::image_specification::ImageSpecification>,
}

async fn upload_image(
    container: web::Data<Container>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<web::Json<StoreImageInfoResult>, DomainErr> {
    let _validation = UploadImageValidation::new(&form, container.config.clone()).validate()?;
    let file_name = form.file.file_name.unwrap();
    let path = format!("./tmp/{}", file_name);

    let image = entity::image::Image {
        full_name: file_name,
        size: form.file.size,
        ..Default::default()
    };

    let store_image_param = StoreImageInfoParam {
        image: image.clone(),
        specification: form.params.into_inner(),
    };

    // TODO: replace real token with test
    let auth_param = AuthorizeImageUploadParam {
        token: "test".to_string(),
        image,
    };

    container
        .authorization_service
        .authorize_image_upload(auth_param)?;

    form.file
        .file
        .persist(&path)
        .map_err(|e| DomainErr::new(e.to_string(), ErrKind::UnprocessableErr))?;

    let result = container
        .image_service
        .store_img_info(store_image_param)
        .await?;

    Ok(web::Json(result))
}

async fn sign_url(
    authorization_service: web::Data<dyn AuthorizationService>,
    param: web::Json<GenerateSignUrlTokenParam>,
) -> Result<web::Json<GenerateSignUrlTokenResult>, DomainErr> {
    let result = authorization_service
        .generate_sign_url_token(param.into_inner())?;
    Ok(web::Json(result))
}
