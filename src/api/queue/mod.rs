use std::sync::Arc;

use futures_lite::StreamExt;
use lapin::{
    Channel, Connection, ConnectionProperties, Consumer, Queue, options::*, types::FieldTable,
};

use crate::domain::{param::image_service_param::OptImgParam, service::ImageService};

pub struct QueueListener {
    pub conn_url: String,
    pub queue_name: String,
    pub consumer_tag: String,
    pub image_service: Arc<dyn ImageService>,
}

impl QueueListener {
    pub async fn listen(&self) -> Result<(), lapin::Error> {
        let conn = Connection::connect(&self.conn_url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        let _queue = self.declare_queue(&channel).await?;
        let mut consumer = self.declare_consumer(&channel).await?;

        println!("Connected to RabbitMQ");

        while let Some(delivery) = consumer.next().await {
            if let Ok(delivery) = delivery {
                let message = String::from_utf8_lossy(&delivery.data);
                let image_service_param = serde_json::from_str::<OptImgParam>(&message);

                match image_service_param {
                    Ok(param) => {
                        let result = self.image_service.opt_img(param).await;
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
                    Err(e) => {
                        let _ = delivery.reject(BasicRejectOptions { requeue: false });
                        println!("Error: {}", e);
                    }
                }
            }
        }

        Ok(())
    }

    async fn declare_queue(&self, channel: &Channel) -> Result<Queue, lapin::Error> {
        let queue = channel
            .queue_declare(
                &self.queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(queue)
    }

    async fn declare_consumer(&self, channel: &Channel) -> Result<Consumer, lapin::Error> {
        let consumer = channel
            .basic_consume(
                &self.queue_name,
                &self.consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(consumer)
    }
}
