use std::fmt::Display;

use serde::{de, ser};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unable to serialize: {0}")]
    SerializeError(String),

    #[error("unable to deserialize: {0}")]
    DeserializeError(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::SerializeError(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::DeserializeError(msg.to_string())
    }
}
