use async_trait::async_trait;
use lapin::{
    Channel, Connection, ConnectionProperties, Consumer, ExchangeKind, Queue,
    options::{BasicConsumeOptions, ExchangeDeclareOptions, QueueDeclareOptions},
    types::FieldTable,
};
use serde::{Deserialize, Serialize};

pub mod image_queue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RabbitMQConfig {
    connection_url: String,
}

pub async fn conn(c: RabbitMQConfig) -> Connection {
    Connection::connect(&c.connection_url, ConnectionProperties::default())
        .await
        .unwrap()
}

#[async_trait]
pub trait RabbitMqImpl {
    async fn declare_queue(
        &self,
        channel: &Channel,
        queue_name: &str,
    ) -> Result<Queue, lapin::Error> {
        channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
    }

    async fn declare_consumer(
        &self,
        channel: &Channel,
        queue_name: &str,
        consumer_tag: &str,
    ) -> Result<Consumer, lapin::Error> {
        channel
            .basic_consume(
                queue_name,
                consumer_tag,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
    }

    async fn declare_exchange(
        &self,
        channel: &Channel,
        exchange_name: &str,
    ) -> Result<(), lapin::Error> {
        channel
            .exchange_declare(
                exchange_name,
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
    }
}
