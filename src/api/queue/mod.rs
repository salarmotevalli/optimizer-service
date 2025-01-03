use std::sync::Arc;

use futures_lite::StreamExt;
use lapin::{Channel, Consumer, Queue, options::*, types::FieldTable};
use serde::de;

use crate::{
    domain::{param::image_service_param::OptImgParam, service::ImageService},
    infra::queue::rabbitmq::{RabbitMqImpl, image_queue::ImageQueueConfig},
};

pub struct QueueConsumer {
    config: ImageQueueConfig,
    conn: Arc<lapin::Connection>,
    image_service: Arc<dyn ImageService>,
}

impl QueueConsumer {
    pub fn new(
        config: ImageQueueConfig,
        conn: Arc<lapin::Connection>,
        image_service: Arc<dyn ImageService>,
    ) -> Self {
        Self {
            config,
            conn,
            image_service,
        }
    }

    pub async fn consume(&self) -> Result<(), lapin::Error> {
        let channel = self.conn.create_channel().await?;
        let _queue = self
            .declare_queue(&channel, &self.config.queue_name)
            .await?;
        let mut consumer = self
            .declare_consumer(&channel, &self.config.queue_name, &self.config.consumer_taq)
            .await?;

        println!("Connected to RabbitMQ");

        while let Some(d) = consumer.next().await {
            if let Err(e) = d {
                continue;
            }

            let delivery = d.unwrap();

            let message = String::from_utf8_lossy(&delivery.data);
            let image_service_param = serde_json::from_str::<OptImgParam>(&message);

            if let Err(e) = image_service_param {
                let _ = delivery.reject(BasicRejectOptions { requeue: false });
                println!("Error: {}", e);
                continue;
            }


            let result = self.image_service.opt_img(image_service_param.unwrap()).await;
            
            match result {
                Ok(_result) => {
                    let _ = delivery.ack(BasicAckOptions::default()).await;
                }
                Err(e) => {
                    let _ = delivery
                        .nack(BasicNackOptions {
                            requeue: true,
                            ..Default::default()
                        })
                        .await;
                    println!("Error: {}", e);
                }
            }
        }

        Ok(())
    }
}

impl RabbitMqImpl for QueueConsumer {}
