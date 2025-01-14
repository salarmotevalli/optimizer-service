use std::sync::Arc;

use async_nats::Client;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
    error::DomainResult, param::image_service_param::*, queue::ImageQueue,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageQueueNatsConfig {
    pub income_namespace: String,
    pub result_namespace: String,
}

pub struct ImageQueueNatsImpl {
    client: Arc<Client>,
    config: ImageQueueNatsConfig,
}

impl ImageQueueNatsImpl {
    pub fn new(client: Arc<Client>, config: ImageQueueNatsConfig) -> Self {
        Self { client, config }
    }
}

#[async_trait]
impl ImageQueue for ImageQueueNatsImpl {
    async fn push_image(&self, param: OptimizeImageParam) -> DomainResult<()> {
        let serde_param = serde_json::to_string(&param)?;

        self.client
            .publish(self.config.income_namespace.clone(), serde_param.into())
            .await?;

        Ok(())
    }

    async fn push_process_result(&self, param: ProcessResultParam) -> DomainResult<()> {
        let serde_param = serde_json::to_string(&param)?;

        self.client
            .publish(self.config.result_namespace.clone(), serde_param.into())
            .await?;

        Ok(())
    }
}
