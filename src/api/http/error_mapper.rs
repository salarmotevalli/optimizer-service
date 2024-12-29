use actix_web::{HttpResponse, ResponseError};

use crate::domain::error::{DomainErr, ErrKind};

impl ResponseError for DomainErr {
    fn error_response(&self) -> HttpResponse {
        match self.kind {
            ErrKind::UnExpectedErr => HttpResponse::InternalServerError().json(self.to_string()),
        }
    }
}
