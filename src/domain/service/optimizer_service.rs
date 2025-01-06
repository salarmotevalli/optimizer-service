use async_trait::async_trait;

use crate::domain::error::DomainResult;

#[async_trait]
pub trait OptimizerService: Send + Sync {
    fn process(&self) -> DomainResult<()>;
}
