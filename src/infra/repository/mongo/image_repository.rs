use async_trait::async_trait;

use crate::domain::{entity::image::Image, error::DomainResult, repository::ImageRepository};

pub struct ImageRepositoryImpl {
    db_url: String,
}

impl Default for ImageRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageRepositoryImpl {
    pub fn new() -> Self {
        Self {
            db_url: "test".to_string(),
        }
    }
}

#[async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn get_img(&self, id: u64) -> DomainResult<Image> {
        todo!()
    }
}
