use std::{fmt::format, sync::Arc};

use async_nats::Client;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::{
    error::DomainResult, param::image_service_param::OptImgParam, queue::ImageQueue,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageQueueNatsConfig {
    pub namespace: String,
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
    async fn push_image(&self, param: OptImgParam) -> DomainResult<()> {
        let serde_param = serde_json::to_string(&param)?;

        self.client
            .publish(self.config.namespace.clone(), serde_param.into())
            .await?;

        Ok(())
    }
}
