use std::{str::from_utf8, sync::Arc};

use futures_lite::StreamExt;

use crate::{
    domain::{param::image_service_param::*, service::ImageService},
    infra::queue::nats::image_queue::ImageQueueNatsConfig,
};

pub struct QueueConsumer {
    config: ImageQueueNatsConfig,
    client: Arc<async_nats::Client>,
    // TODO: replace with container
    image_service: Arc<dyn ImageService>,
}

impl QueueConsumer {
    pub fn new(
        config: ImageQueueNatsConfig,
        client: Arc<async_nats::Client>,
        image_service: Arc<dyn ImageService>,
    ) -> Self {
        Self {
            config,
            client,
            image_service,
        }
    }

    pub async fn subscribe(&self) -> Result<(), async_nats::Error> {
        let mut subscription = self
            .client
            .subscribe(self.config.namespace.clone())
            .await?
            .take(1);

        while let Some(message) = subscription.next().await {
            let pl = from_utf8(&message.payload)?;
            let image_service_param = serde_json::from_str::<OptimizeImageParam>(&pl).unwrap();

            self.image_service
                .optimize_image(image_service_param)
                .await
                .unwrap();
        }

        Ok(())
    }
}
