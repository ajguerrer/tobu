use std::convert::TryFrom;

use super::error::Error;

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct FieldNumber(i32);

const MIN_FIELD_NUMBER: i32 = 1;
const FIRST_RESERVED_NUMBER: i32 = 19000;
const LAST_RESERVED_NUMBER: i32 = 19999;
const MAX_VALID_NUMBER: i32 = (1 << 29) - 1;

impl FieldNumber {
    /// # Safety
    ///
    /// This function must be called with a value in range [1, 1 << 29).
    /// Furthermore, value must not be in range [19000, 20000) reserved for internal use.
    pub const unsafe fn new_unchecked(n: i32) -> Self {
        Self(n)
    }

    pub fn new(n: i32) -> Option<Self> {
        if FieldNumber::valid(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    pub const fn get(self) -> i32 {
        self.0
    }

    fn valid(n: i32) -> bool {
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
    type Error = Error;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if FieldNumber::valid(v) {
            Ok(FieldNumber(v))
        } else {
            Err(Error::InvalidFieldNumber(v))
        }
    }
}

impl From<FieldNumber> for i32 {
    fn from(v: FieldNumber) -> Self {
        v.0
    }
}
