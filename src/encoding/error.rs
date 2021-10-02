#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("invalid field number {0}")]
    InvalidFieldNumber(i32),

    #[error("invalid wire type {0}")]
    InvalidWireType(i8),

    #[error("unexpected EOF")]
    Eof,

    #[error("variable length integer overflow")]
    Overflow,
}
