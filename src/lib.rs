use error::Error;
use info::MessageInfo;
use ser::SizeHint;
use serde::Serialize;

pub mod encoding;
pub mod error;
pub mod info;
pub mod ser;
pub mod value;

pub fn serialized_size(value: &impl Serialize, info: &'static MessageInfo) -> Result<usize, Error> {
    let mut size_hint = SizeHint::new(info);
    value.serialize(&mut size_hint)
}
