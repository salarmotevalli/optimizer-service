use std::sync::Arc;

use async_trait::async_trait;
use lapin::{
    BasicProperties, ExchangeKind,
    options::{BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
};
use serde::{Deserialize, Serialize};

use crate::domain::{
    error::{DomainErr, DomainResult, ErrKind},
    param::image_service_param::OptImgParam,
    queue::ImageQueue,
};

use super::RabbitMqImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageQueueConfig {
    pub queue_name: String,
    pub routing_key: String,
    pub exchange_name: String,
    pub consumer_taq: String,
}

pub struct ImageQueueRabbitMQImpl {
    conn: Arc<lapin::Connection>,
    config: ImageQueueConfig,
}

impl ImageQueueRabbitMQImpl {
    pub fn new(conn: Arc<lapin::Connection>, config: ImageQueueConfig) -> Self {
        Self { conn, config }
    }
}

#[async_trait]
impl ImageQueue for ImageQueueRabbitMQImpl {
    async fn push_image(&self, param: OptImgParam) -> DomainResult<()> {
        let channel = self.conn.create_channel().await?;
        let _exchange = self
            .declare_exchange(&channel, &self.config.exchange_name)
            .await?;
        let queue = self
            .declare_queue(&channel, &self.config.queue_name)
            .await?;

        // Bind the queue to the exchange
        channel
            .queue_bind(
                queue.name().as_str(),
                &self.config.exchange_name,
                &self.config.routing_key,
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let payload = serde_json::ser::to_vec(&param).map_err(|e| DomainErr {
            message: e.to_string(),
            kind: ErrKind::UnExpectedErr,
        })?;

        channel
            .basic_publish(
                &self.config.exchange_name,
                &self.config.routing_key,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;

        println!(
            "Message published to exchange '{}'",
            self.config.exchange_name
        );

        Ok(())
    }
}

#[async_trait]
impl RabbitMqImpl for ImageQueueRabbitMQImpl {}
