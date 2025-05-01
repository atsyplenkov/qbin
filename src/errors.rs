use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum QuadbinError {
    InvalidDirection(u8),
    InvalidCell(Option<u64>),
    InvalidResolution(u8),
    InvalidOffset(f64),
}

impl fmt::Display for QuadbinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuadbinError::InvalidDirection(e) => write!(f, "invalid direction: {}", e),
            QuadbinError::InvalidCell(e) => write!(f, "invalid cell index: {:?}", e),
            QuadbinError::InvalidResolution(e) => write!(
                f,
                "Invalid resolution specified: {}. Accepted values are between 0 and 26, inclusive",
                e
            ),
            QuadbinError::InvalidOffset(msg) => write!(f, "invalid offset: {}", msg),
        }
    }
}

impl Error for QuadbinError {}
