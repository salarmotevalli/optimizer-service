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

#[derive(Debug)]
pub enum ErrKind {
    UnExpectedErr,
}