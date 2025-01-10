use async_trait::async_trait;

use crate::domain::{
    error::DomainResult,
    param::optimizer_service_param::*,
};

#[async_trait]
pub trait OptimizerService: Send + Sync {
    async fn process(&self, param: ProcessParam) -> DomainResult<ProcessResult>;
}
