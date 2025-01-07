use async_trait::async_trait;
use aws_config::{retry::RetryConfig, Region};
use aws_sdk_s3::{config::Credentials, primitives::{ByteStream, SdkBody}, Client};
use serde::{Deserialize, Serialize};
use crate::domain::{error::DomainResult, param::file_storage_service_param::{StoreParam, StoreResult}, service::FileStorageService};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MinioConfig {
    pub endpoint: String,
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
}
pub struct FileStorageMinioImpl{
    pub config: MinioConfig
}

#[async_trait]
impl FileStorageService for FileStorageMinioImpl {
    async fn store(&self, param:StoreParam) -> DomainResult<StoreResult>  {

        let config = aws_config::from_env()
        .region(Region::new(self.config.region.clone()))
        .credentials_provider(Credentials::new(self.config.access_key.clone(), self.config.secret_key.clone(), None, None, "MinIO"))
        .endpoint_url(self.config.endpoint.clone())
        .retry_config(RetryConfig::standard())
        .load()
        .await;

        let client = Client::new(&config);

        let body = SdkBody::from(param.data);

        client.put_object()
        .bucket("optimized")
        .body(ByteStream::from(body))
        .key("image.jpeg")
        .send()
        .await.unwrap()
        ;

        Ok(StoreResult{})
    }
}