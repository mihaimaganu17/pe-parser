use std::fmt;

pub type Result<T> = std::result::Result<T, PeError>;

#[derive(Debug)]
pub enum PeError {
    UnknownPath(String),
    FileReadFailed(std::io::Error),
}

impl fmt::Display for PeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<std::io::Error> for PeError {
    fn from(err: std::io::Error) -> Self {
        PeError::FileReadFailed(err)
    }
}

