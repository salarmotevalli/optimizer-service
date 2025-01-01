use std::time::SystemTimeError;

pub type DomainResult<T> = Result<T, DomainErr>;

#[derive(Debug)]
pub struct DomainErr {
    pub message: String,
    pub kind: ErrKind,
}

impl DomainErr {
    pub fn new(message: String, kind: ErrKind) -> Self {
        Self { message, kind }
    }
}

impl std::fmt::Display for DomainErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<SystemTimeError> for DomainErr {
    fn from(value: SystemTimeError) -> Self {
        Self {
            message: value.to_string(),
            kind: ErrKind::UnExpectedErr,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for DomainErr {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        let kind = match value.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => ErrKind::UnAuthorizedErr,
            _ => ErrKind::UnExpectedErr,
        };
        
        Self {
            message: value.to_string(),
            kind
        }
    }
}

#[derive(Debug)]
pub enum ErrKind {
    UnExpectedErr,
    UnAuthorizedErr,
    Forbidden,
    UnprocessableErr,
}
