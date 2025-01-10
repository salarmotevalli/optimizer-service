use std::sync::Arc;

use serviceorented::{
    api::queue::nats::QueueConsumer,
    app_config::Config,
    infra::queue::nats::{NatsQueue, image_queue::ImageQueueNatsImpl},
    service::{
        file_storage_service::minio::FileStorageMinioImpl,
        image_service::ImageServiceImpl,
        optimizer_service::{self, OptimizerServiceRImageImpl},
    },
};
use tokio;

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        // make it dynamic
        .worker_threads(4)
        .enable_all()
        .thread_name("queue-worker")
        .thread_stack_size(4 * 1024 * 1024)
        .build()
        .unwrap();

    rt.block_on(async {
        let cnf = Config::load("siagoosh".to_string());

        let nats_queue = NatsQueue::new(cnf.nats_config.clone()).await;
        let client = Arc::new(nats_queue.client());

        let image_queue =
            ImageQueueNatsImpl::new(client.clone(), cnf.image_queue_nats_config.clone());

        let file_storage_service = FileStorageMinioImpl {
            config: cnf.minio_config.clone(),
        };
        let optimizer_service = OptimizerServiceRImageImpl {};

        let image_service = ImageServiceImpl {
        file_storage_service: Arc::new(file_storage_service),
            optimizer_service: Arc::new(optimizer_service),
            image_queue: Arc::new(image_queue),
        };

        let _ = QueueConsumer::new(
            cnf.image_queue_nats_config,
            client.clone(),
            Arc::new(image_service),
        )
        .subscribe()
        .await;
    });
}
