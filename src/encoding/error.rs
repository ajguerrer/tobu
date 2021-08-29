#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("field {0} contains invalid UTF-8")]
    InvalidUTF8(String),

    #[error("required field {0} not set")]
    Required(String),

    #[error("invalid field number {0}")]
    InvalidFieldNumber(i32),

    #[error("invalid wire type {0}")]
    InvalidWireType(i8),

    #[error("invalid value {0} for enum {1}")]
    InvalidEnum(i32, String),

    #[error("invalid proto declaration {0}")]
    InvalidName(String),

    #[error("unexpected EOF")]
    Eof,

    #[error("variable length integer overflow")]
    Overflow,

    #[error("mismatching end group marker")]
    EndGroup,
}
