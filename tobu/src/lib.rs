use bytes::{Bytes, BytesMut};
use error::Error;
use info::MessageInfo;
use ser::{Serializer, SizeHint};
use serde::Serialize;

pub mod de;
pub mod error;
pub mod info;
pub mod ser;
pub mod value;

pub fn to_vec(value: &impl Serialize, info: &'static MessageInfo) -> Result<Vec<u8>, Error> {
    let mut vec = Vec::with_capacity(serialized_size(value, info)?);

    let mut serializer = Serializer::new(&mut vec, info);
    value.serialize(&mut serializer)?;

    Ok(vec)
}

pub fn to_bytes(value: &impl Serialize, info: &'static MessageInfo) -> Result<Bytes, Error> {
    let mut bytes = BytesMut::with_capacity(serialized_size(value, info)?);

    let mut serializer = Serializer::new(&mut bytes, info);
    value.serialize(&mut serializer)?;

    Ok(bytes.freeze())
}

pub fn to_bytes_mut(value: &impl Serialize, info: &'static MessageInfo) -> Result<BytesMut, Error> {
    let mut bytes = BytesMut::with_capacity(serialized_size(value, info)?);

    let mut serializer = Serializer::new(&mut bytes, info);
    value.serialize(&mut serializer)?;

    Ok(bytes)
}

pub fn serialized_size(value: &impl Serialize, info: &'static MessageInfo) -> Result<usize, Error> {
    let mut size_hint = SizeHint::new(info);
    value.serialize(&mut size_hint)
}
