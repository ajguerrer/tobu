use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};

use serde::{
    de::Visitor, forward_to_deserialize_any, Deserialize, Deserializer, Serialize, Serializer,
};

use crate::error::Error;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Number {
    n: N,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum N {
    Unsigned(u64),
    Signed(i64),
    Float(f64),
}

impl Number {
    #[inline]
    pub fn is_i64(&self) -> bool {
        match self.n {
            N::Unsigned(v) => v <= i64::MAX as u64,
            N::Signed(_) => true,
            N::Float(_) => false,
        }
    }

    #[inline]
    pub fn as_i64(&self) -> Option<i64> {
        match self.n {
            N::Unsigned(n) => {
                if n <= i64::MAX as u64 {
                    Some(n as i64)
                } else {
                    None
                }
            }
            N::Signed(n) => Some(n),
            N::Float(_) => None,
        }
    }

    #[inline]
    pub fn is_u64(&self) -> bool {
        match self.n {
            N::Unsigned(_) => true,
            N::Signed(n) => n >= 0,
            N::Float(_) => false,
        }
    }

    #[inline]
    pub fn as_u64(&self) -> Option<u64> {
        match self.n {
            N::Unsigned(n) => Some(n),
            N::Signed(n) => {
                if n >= 0 {
                    Some(n as u64)
                } else {
                    None
                }
            }
            N::Float(_) => None,
        }
    }

    #[inline]
    pub fn is_f64(&self) -> bool {
        match self.n {
            N::Float(_) => true,
            N::Unsigned(_) | N::Signed(_) => false,
        }
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self.n {
            N::Unsigned(n) => Some(n as f64),
            N::Signed(n) => Some(n as f64),
            N::Float(n) => Some(n),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.n, f)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.n {
            N::Unsigned(n) => fmt::Display::fmt(&n, f),
            N::Signed(n) => fmt::Display::fmt(&n, f),
            N::Float(n) => fmt::Display::fmt(&n, f),
        }
    }
}

impl Serialize for Number {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.n {
            N::Unsigned(i) => serializer.serialize_u64(i),
            N::Signed(i) => serializer.serialize_i64(i),
            N::Float(f) => serializer.serialize_f64(f),
        }
    }
}

impl<'de> Deserialize<'de> for Number {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Number, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number")
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                Ok(value.into())
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Number, E> {
                Ok(value.into())
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

impl<'de> Deserializer<'de> for Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.n {
            N::Unsigned(i) => visitor.visit_u64(i),
            N::Signed(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

impl<'de, 'a> Deserializer<'de> for &'a Number {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        match self.n {
            N::Unsigned(i) => visitor.visit_u64(i),
            N::Signed(i) => visitor.visit_i64(i),
            N::Float(f) => visitor.visit_f64(f),
        }
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

macro_rules! from_signed {
    ($($signed_ty:ty)*) => {
        $(
            impl From<$signed_ty> for Number {
                #[inline]
                fn from(i: $signed_ty) -> Self {
                    Number {
                        n: N::Signed(i as i64),
                    }
                }
            }
        )*
    };
}

macro_rules! from_unsigned {
    ($($unsigned_ty:ty)*) => {
        $(
            impl From<$unsigned_ty> for Number {
                #[inline]
                fn from(i: $unsigned_ty) -> Self {
                    Number {
                        n: N::Unsigned(i as u64),
                    }
                }
            }
        )*
    };
}

macro_rules! from_float {
    ($($float_ty:ty)*) => {
        $(
            impl From<$float_ty> for Number {
                #[inline]
                fn from(i: $float_ty) -> Self {
                    Number {
                        n: N::Float(i as f64),
                    }
                }
            }
        )*
    };
}

from_signed!(i8 i16 i32 i64 isize);
from_unsigned!(u8 u16 u32 u64 usize);
from_float!(f32 f64);

// floats should not be hashed anyway
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.n {
            N::Signed(n) => n.hash(state),
            N::Unsigned(n) => n.hash(state),
            N::Float(_) => 3.hash(state),
        }
    }
}
