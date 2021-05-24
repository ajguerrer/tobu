use serde::{
    ser::{self, Impossible, SerializeMap, SerializeSeq, SerializeStruct},
    Serializer,
};

use crate::{
    encoding::wire::{encode_zig_zag, size_bytes, size_fixed32, size_fixed64, size_varint},
    error::Error,
    info::{FieldInfo, MessageInfo, Syntax, Type},
};

struct SizeHint {
    message_info: &'static MessageInfo,
    field_index: usize,
}

impl SizeHint {
    fn field_info(&self) -> Result<&'static FieldInfo, Error> {
        self.message_info
            .fields
            .get(self.field_index)
            .ok_or_else(|| ser::Error::custom("field descriptor not found"))
    }
}

impl<'a> Serializer for &'a mut SizeHint {
    type Ok = usize;
    type Error = Error;

    type SerializeSeq = RepeatedSizeHint<'a>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = MapSizeHint<'a>;
    type SerializeStruct = MessageSizeHint<'a>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("i8 not supported"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("i16 not supported"))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("u8 not supported"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("u16 not supported"))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let field_info = self.field_info()?;
        if self.message_info.syntax == Syntax::Proto3 && v == 0 && field_info.oneof_index.is_none()
        {
            return Ok(0);
        }

        match field_info.ty {
            Type::Int32 | Type::Uint32 | Type::Int64 | Type::Uint64 | Type::Bool | Type::Enum => {
                Ok(size_varint(v))
            }
            Type::Fixed32 | Type::SFixed32 | Type::Float => Ok(size_fixed32()),
            Type::Fixed64 | Type::SFixed64 | Type::Double => Ok(size_fixed64()),
            Type::SInt32 | Type::SInt64 => Ok(size_varint(encode_zig_zag(v as i64))),
            _ => Err(ser::Error::custom("field descriptor does not match value")),
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.to_bits() as u64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.to_bits())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("char not supported"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        if self.message_info.syntax == Syntax::Proto3
            && v.is_empty()
            && self.field_info()?.oneof_index.is_none()
        {
            Ok(0)
        } else {
            Ok(size_bytes(v.len()))
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(0)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit not supported"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit struct not supported"))
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit variant not supported"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ser::Error::custom("newtype struct not supported"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ser::Error::custom("newtype variant not supported"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let field_info = self.field_info()?;
        let packed = if field_info.packed {
            Some(size_varint(field_info.number as u64))
        } else {
            None
        };

        Ok(RepeatedSizeHint {
            total: 0,
            ser: self,
            packed,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("char not supported"))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("char not supported"))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("char not supported"))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("char not supported"))
    }
}

struct RepeatedSizeHint<'a> {
    total: usize,
    ser: &'a mut SizeHint,
    packed: Option<usize>,
}

impl<'a> SerializeSeq for RepeatedSizeHint<'a> {
    type Ok = usize;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        match self.packed {
            Some(size_field_number) => {
                self.total += size_field_number;
                self.total += value.serialize(&mut *self.ser)?;
            }
            None => self.total += value.serialize(&mut *self.ser)?,
        };
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.total)
    }
}

struct MapSizeHint<'a> {
    total: usize,
    ser: &'a mut SizeHint,
}

impl<'a> SerializeMap for MapSizeHint<'a> {
    type Ok = usize;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        // field number for key is always 1 (size 1)
        self.total = 1 + key.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        // field number for value is always 2 (size 1)
        self.total = 1 + value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.total)
    }
}

struct MessageSizeHint<'a> {
    total: usize,
    ser: &'a mut SizeHint,
    field_index: usize,
}

impl<'a> SerializeStruct for MessageSizeHint<'a> {
    type Ok = usize;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.total += self.ser.serialize_i32(self.field_number()?)?;
        self.total += value.serialize(&mut *self.ser)?;
        self.ser.field_index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.total)
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        self.ser.field_index += 1;
        Ok(())
    }
}
