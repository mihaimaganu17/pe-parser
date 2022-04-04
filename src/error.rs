use std::fmt;

pub type Result<T> = std::result::Result<T, PeError>;

#[derive(Debug)]
pub enum PeError {
    UnknownPath(String),
    FileReadFailed(std::io::Error),
    SliceConversionFailed(std::array::TryFromSliceError),
    IntConversionFailed(std::num::TryFromIntError),
    BufferTooSmall,
    InvalidMachineType(u16),
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

impl From<std::array::TryFromSliceError> for PeError {
    fn from(err: std::array::TryFromSliceError) -> Self {
        PeError::SliceConversionFailed(err)
    }
}

impl From<std::num::TryFromIntError> for PeError {
    fn from(err: std::num::TryFromIntError) -> Self {
        PeError::IntConversionFailed(err)
    }
}
