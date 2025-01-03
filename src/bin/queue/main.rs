use std::sync::Arc;

use tokio;
use serviceorented::{api::queue::QueueConsumer, app_config::Config, infra::queue::rabbitmq::{conn, image_queue::ImageQueueRabbitMQImpl}, service::image_service::{self, ImageServiceImpl}};

fn main() {

    let rt = tokio::runtime::Builder::new_multi_thread()
    // make it dynamic
    .worker_threads(4)
    .enable_all()
    .thread_name("my-custom-name")
    .thread_stack_size(3 * 1024 * 1024)
    .build()
    .unwrap();

    rt.block_on(async {
        let cnf = Config::load("siagoosh".to_string());
    
        let rabbit_conn = Arc::new(conn(cnf.rabbit_mq_config.clone()).await);
        
        let image_queue = ImageQueueRabbitMQImpl::new(
            rabbit_conn.clone(),
            cnf.image_queue_config.clone(),
        );

        let image_service = ImageServiceImpl {image_queue: Arc::new(image_queue)};
    
        QueueConsumer::new(cnf.image_queue_config, rabbit_conn.clone(), Arc::new(image_service));
    });
}
