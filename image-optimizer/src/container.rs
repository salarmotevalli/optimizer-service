use std::sync::Arc;

use crate::{
    app_config::Config,
    domain::service::{AuthorizationService, ImageService, TokenService},
};

#[derive(Clone)]
pub struct Container {
    pub authorization_service: Arc<dyn AuthorizationService>,
    pub token_service: Arc<dyn TokenService>,
    pub image_service: Arc<dyn ImageService>,
    pub config: Config,
}

impl Container {
    pub fn new(
        cnf: Config,
        authorization_service: Arc<dyn AuthorizationService>,
        token_service: Arc<dyn TokenService>,
        image_service: Arc<dyn ImageService>,
    ) -> Self {
        Self {
            authorization_service,
            token_service,
            image_service,
            config: cnf,
        }
    }
}
