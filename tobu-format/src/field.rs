use std::{convert::TryFrom, fmt::Display};

use super::error::DecodeError;

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct FieldNumber(i32);

const MIN_FIELD_NUMBER: i32 = 1;
const FIRST_RESERVED_NUMBER: i32 = 19000;
const LAST_RESERVED_NUMBER: i32 = 19999;
const MAX_VALID_NUMBER: i32 = (1 << 29) - 1;

impl FieldNumber {
    // TODO: someday we can make this a const fn
    pub fn new(n: i32) -> Self {
        assert!(FieldNumber::valid(n));
        Self(n)
    }

    pub const fn get(self) -> i32 {
        self.0
    }

    const fn valid(n: i32) -> bool {
        MIN_FIELD_NUMBER <= n && n < FIRST_RESERVED_NUMBER
            || LAST_RESERVED_NUMBER < n && n <= MAX_VALID_NUMBER
    }
}

impl Default for FieldNumber {
    fn default() -> Self {
        FieldNumber(1)
    }
}

impl TryFrom<i32> for FieldNumber {
    type Error = DecodeError;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if FieldNumber::valid(v) {
            Ok(FieldNumber(v))
        } else {
            Err(DecodeError::InvalidFieldNumber(v))
        }
    }
}

impl From<FieldNumber> for i32 {
    fn from(v: FieldNumber) -> Self {
        v.0
    }
}

impl From<FieldNumber> for u64 {
    fn from(v: FieldNumber) -> Self {
        v.0 as u64
    }
}

impl Display for FieldNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
