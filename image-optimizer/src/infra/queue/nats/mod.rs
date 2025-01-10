use async_nats::Client;

use serde::{Deserialize, Serialize};

pub mod image_queue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NatsConfig {
    connection_url: String,
}

pub struct NatsQueue {
    client: Client,
}

impl NatsQueue {
    pub async fn new(conf: NatsConfig) -> NatsQueue {
        let client = async_nats::connect(&conf.connection_url).await.unwrap();

        NatsQueue { client }
    }

    pub fn client(&self) -> Client {
        self.client.clone()
    }
}
