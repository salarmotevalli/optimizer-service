use std::sync::Arc;

use serviceorented::{
    api::http::serve,
    app_config,
    container::Container,
    infra::queue::nats::{image_queue::ImageQueueNatsImpl, NatsQueue},
    service::{
        authorization_service::AuthorizationServiceImpl, file_storage_service::minio::FileStorageMinioImpl, image_service::ImageServiceImpl, optimizer_service::{self, OptimizerServiceRImageImpl}, token_service::TokenServiceJWTImpl
    },
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo read app name from env
    let cnf = app_config::Config::load("siagoosh".to_string());

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    std::fs::create_dir_all(&cnf.file_temp_dir)?;

    let token_service = Arc::new(TokenServiceJWTImpl {
        config: cnf.token_service_config.clone(),
    });

    let authorization_service = Arc::new(AuthorizationServiceImpl {
        token_service: token_service.clone(),
        config: cnf.authorization_service_config.clone(),
    });

    let nats_queue = NatsQueue::new(cnf.nats_config.clone()).await;

    let image_queue = ImageQueueNatsImpl::new(
        Arc::new(nats_queue.client()),
        cnf.image_queue_nats_config.clone(),
    );

    let file_storage_service = FileStorageMinioImpl{config: cnf.minio_config.clone()};
    let optimizer_service = OptimizerServiceRImageImpl{file_storage_service: Arc::new(file_storage_service)};

    let image_service = Arc::new(ImageServiceImpl {
        optimizer_service: Arc::new(optimizer_service),
        image_queue: Arc::new(image_queue),
    });

    serve(Arc::new(Container::new(
        cnf,
        authorization_service,
        token_service,
        image_service,
    )))
    .await
}
