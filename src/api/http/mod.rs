mod create_app;
mod error_mapper;

pub use create_app::create_app;
use serde::{Deserialize, Serialize};

use std::sync::Arc;

use crate::{
    container::Container,
    domain::{
        entity::{self, image_specification::{self, ImageSpecification}},
        error::{DomainErr, ErrKind},
        param::{authorization_service_param::*, image_service_param::*},
    },
};

use actix_multipart::form::{MultipartForm, json::Json, tempfile::TempFile};
use actix_web::{
    HttpServer,
    web::{self},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpServerConfig {
    pub port: u16,
    pub workers: usize,
}

pub async fn serve(container: Arc<Container>) -> std::io::Result<()> {
    let create_app_container = container.clone();

    HttpServer::new(move || create_app(create_app_container.clone()))
        .workers(container.config.http_server_config.workers)
        .bind(("127.0.0.1", container.config.http_server_config.port))?
        .run()
        .await
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct spec {}

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "5MB")]
    pub file: TempFile,
    // pub params: Json<image_specification::ImageSpecification>,
}

#[derive(Debug, Deserialize)]
pub struct UploadFileQuery {
    pub token: String,
}

async fn upload_image(
    query: web::Query<UploadFileQuery>,
    MultipartForm(form): MultipartForm<UploadForm>,
    container: web::Data<Container>,
) -> Result<web::Json<StoreImageInfoResult>, DomainErr> {
    let file_name = form.file.file_name.unwrap();
    let path = format!("{}/{}", container.config.file_temp_dir, file_name);

    let image = entity::image::Image {
        full_name: file_name,
        size: form.file.size,
        ..Default::default()
    };

    let store_image_param = StoreImageInfoParam {
        image: image.clone(),
        // specification: form.params.into_inner(),
        specification: ImageSpecification::default(),
    };

    let auth_param = AuthorizeImageUploadParam {
        token: query.token.clone(),
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

async fn sign_url_token(
    container: web::Data<Container>,
    param: web::Json<GenerateSignUrlTokenParam>,
) -> Result<web::Json<GenerateSignUrlTokenResult>, DomainErr> {
    let result = container
        .authorization_service
        .generate_sign_url_token(param.into_inner())?;
    Ok(web::Json(result))
}
