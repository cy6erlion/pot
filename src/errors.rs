//! error management
use std::fmt;

#[derive(Debug)]
/// error management
pub enum PotError {
    /// Error occured while deserializing network type
    NetworkDeserializeError,
}

impl std::error::Error for PotError {}

impl fmt::Display for PotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NetworkDeserializeError => {
                write!(f, "Error occured while deserializing network type")
            }
        }
    }
}
