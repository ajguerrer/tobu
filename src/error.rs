use std::fmt::Display;

use serde::{de, ser};
use tobu_format::{error::DecodeError, field::FieldNumber};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error("{0}")]
    DecodeError(#[from] DecodeError),

    #[error("field number {0} not found")]
    FieldNotFound(FieldNumber),

    #[error("unknown sequence length")]
    UnknownSeqLen,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}
