use std::fmt::{Debug, Display};

use std::error::Error;

pub type DomainResult<T> = Result<T, DomainErr>;

#[derive(Debug)]
pub struct DomainErr {
    pub message: String,
    pub kind: ErrKind,
}

impl DomainErr {
    pub fn new(message: String, kind: ErrKind) -> Self {
        Self { message, kind}
    }

    // TODO: pub with_meta()
}

impl Display for DomainErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<T: Error> From<T> for DomainErr {
    fn from(value: T) -> Self {
        Self {
            message: value.to_string(),
            kind: ErrKind::UnExpectedErr,
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
