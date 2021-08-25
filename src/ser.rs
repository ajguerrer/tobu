use std::io;

use serde::ser::{self, Impossible, SerializeMap, SerializeSeq, SerializeStruct};

use crate::{
    encoding::{
        field::FieldNumber,
        wire::{
            append_bytes, append_fixed32, append_fixed64, append_tag, append_varint, encode_tag,
            encode_zig_zag, size_bytes, size_fixed32, size_fixed64, size_tag, size_varint,
        },
    },
    error::Error,
    info::{FieldInfo, MessageInfo, Syntax, Type},
};

pub(crate) struct SizeHint {
    message_info: &'static MessageInfo,
    field_index: usize,
    is_nested: bool,
}
impl SizeHint {
    pub fn new(message_info: &'static MessageInfo) -> Self {
        SizeHint {
            message_info,
            field_index: 0,
            is_nested: false,
        }
    }

    fn field_info(&self) -> Result<&'static FieldInfo, Error> {
        self.message_info
            .fields
            .get(self.field_index)
            .ok_or_else(|| ser::Error::custom("field descriptor not found"))
    }

    fn message_info(&self) -> Result<&'static MessageInfo, Error> {
        self.field_info()?
            .message_info
            .ok_or_else(|| ser::Error::custom("message descriptor not found"))
    }
}

impl<'a> serde::Serializer for &'a mut SizeHint {
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

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
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

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit struct not supported"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit variant not supported"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ser::Error::custom("newtype struct not supported"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ser::Error::custom("newtype variant not supported"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let field_info = self.field_info()?;
        let size_tag = size_varint(field_info.number.get() as u64);
        if field_info.packed {
            // tag + len + element_1..element_len
            let len = len.ok_or_else(|| ser::Error::custom("unknown seq len"))?;
            let size_len = size_varint(len as u64);
            Ok(RepeatedSizeHint {
                total: size_tag + size_len,
                ser: self,
                size_tag: 0,
            })
        } else {
            // (tag + element_1)..(tag + element_len)
            Ok(RepeatedSizeHint {
                total: 0,
                ser: self,
                size_tag,
            })
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("tuple not supported"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("tuple struct not supported"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("tuple variant not supported"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSizeHint {
            total: 0,
            ser: self,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if self.is_nested {
            let parent = Some((self.message_info, self.field_index));
            self.message_info = self.message_info()?;
            self.field_index = 0;
            Ok(MessageSizeHint {
                total: 0,
                ser: self,
                parent,
            })
        } else {
            self.is_nested = true;
            Ok(MessageSizeHint {
                total: 0,
                ser: self,
                parent: None,
            })
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("struct variant not supported"))
    }
}

pub(crate) struct RepeatedSizeHint<'a> {
    total: usize,
    ser: &'a mut SizeHint,
    size_tag: usize,
}

impl<'a> SerializeSeq for RepeatedSizeHint<'a> {
    type Ok = usize;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.total += self.size_tag;
        self.total += value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.total)
    }
}

pub(crate) struct MapSizeHint<'a> {
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
        // field number for key is always 1, so tag size is always 1
        self.total = 1 + key.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        // field number for key is always 2, so tag size is always 1
        self.total = 1 + value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.total)
    }
}

pub(crate) struct MessageSizeHint<'a> {
    total: usize,
    ser: &'a mut SizeHint,
    parent: Option<(&'static MessageInfo, usize)>,
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
        let field_info = self.ser.field_info()?;
        self.total += size_tag(field_info.number);
        self.total += value.serialize(&mut *self.ser)?;
        self.ser.field_index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if let Some((info, index)) = self.parent {
            self.ser.message_info = info;
            self.ser.field_index = index;
        }
        Ok(self.total)
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        self.ser.field_index += 1;
        Ok(())
    }
}

pub(crate) struct Serializer<W> {
    writer: W,
    message_info: &'static MessageInfo,
    field_index: usize,
    is_nested: bool,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    pub fn new(writer: W, message_info: &'static MessageInfo) -> Self {
        Serializer {
            writer,
            message_info,
            field_index: 0,
            is_nested: false,
        }
    }

    fn field_info(&self) -> Result<&'static FieldInfo, Error> {
        self.message_info
            .fields
            .get(self.field_index)
            .ok_or_else(|| ser::Error::custom("field descriptor not found"))
    }

    fn message_info(&self) -> Result<&'static MessageInfo, Error> {
        self.field_info()?
            .message_info
            .ok_or_else(|| ser::Error::custom("message descriptor not found"))
    }
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = RepeatedSerializer<'a, W>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = MessageSerializer<'a, W>;
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
            return Ok(());
        }

        match field_info.ty {
            Type::Int32 | Type::Uint32 | Type::Int64 | Type::Uint64 | Type::Bool | Type::Enum => {
                append_varint(&mut self.writer, v).map_err(ser::Error::custom)
            }
            Type::Fixed32 | Type::SFixed32 | Type::Float => {
                append_fixed32(&mut self.writer, v as u32).map_err(ser::Error::custom)
            }
            Type::Fixed64 | Type::SFixed64 | Type::Double => {
                append_fixed64(&mut self.writer, v).map_err(ser::Error::custom)
            }
            Type::SInt32 | Type::SInt64 => {
                append_varint(&mut self.writer, encode_zig_zag(v as i64))
                    .map_err(ser::Error::custom)
            }
            _ => Err(ser::Error::custom("field descriptor does not match value")),
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.to_bits() as u64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v.to_bits())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
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
            Ok(())
        } else {
            append_bytes(&mut self.writer, v).map_err(ser::Error::custom)
        }
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
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

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit struct not supported"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(ser::Error::custom("unit variant not supported"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ser::Error::custom("newtype struct not supported"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(ser::Error::custom("newtype variant not supported"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let field_info = self.field_info()?;
        let tag = encode_tag(field_info.number, field_info.ty.wire_type());
        if field_info.packed {
            append_varint(&mut self.writer, tag).map_err(ser::Error::custom)?;
            let len = len.ok_or_else(|| ser::Error::custom("unknown seq len"))?;
            append_varint(&mut self.writer, len as u64).map_err(ser::Error::custom)?;
            Ok(RepeatedSerializer {
                ser: self,
                tag: None,
            })
        } else {
            // (tag + element_1)..(tag + element_len)
            Ok(RepeatedSerializer {
                ser: self,
                tag: Some(tag),
            })
        }
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(ser::Error::custom("tuple not supported"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(ser::Error::custom("tuple struct not supported"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("tuple variant not supported"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let mut map_fields = self.message_info()?.fields.iter();

        // safety: 1 and 2 are valid field numbers
        let key_number = unsafe { FieldNumber::new_unchecked(1) };
        let value_number = unsafe { FieldNumber::new_unchecked(2) };

        let key_field = map_fields
            .find(|f| f.number == key_number)
            .ok_or(ser::Error::custom("key field for map not found"))?;
        let value_field = map_fields
            .find(|f| f.number == value_number)
            .ok_or(ser::Error::custom("value field for map not found"))?;

        Ok(MapSerializer {
            ser: self,
            key_tag: encode_tag(key_number, key_field.ty.wire_type()),
            value_tag: encode_tag(value_number, value_field.ty.wire_type()),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        if self.is_nested {
            let parent = Some((self.message_info, self.field_index));
            self.message_info = self.message_info()?;
            self.field_index = 0;
            Ok(MessageSerializer { ser: self, parent })
        } else {
            self.is_nested = true;
            Ok(MessageSerializer {
                ser: self,
                parent: None,
            })
        }
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("struct variant not supported"))
    }
}

pub(crate) struct RepeatedSerializer<'a, W> {
    ser: &'a mut Serializer<W>,
    tag: Option<u64>,
}

impl<'a, W> SerializeSeq for RepeatedSerializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if let Some(tag) = self.tag {
            append_varint(&mut self.ser.writer, tag).map_err(ser::Error::custom)?;
        }
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub(crate) struct MapSerializer<'a, W> {
    ser: &'a mut Serializer<W>,
    key_tag: u64,
    value_tag: u64,
}

impl<'a, W> SerializeMap for MapSerializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        append_varint(&mut self.ser.writer, self.key_tag).map_err(ser::Error::custom)?;
        key.serialize(&mut *self.ser)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        append_varint(&mut self.ser.writer, self.value_tag).map_err(ser::Error::custom)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub(crate) struct MessageSerializer<'a, W> {
    ser: &'a mut Serializer<W>,
    parent: Option<(&'static MessageInfo, usize)>,
}

impl<'a, W> SerializeStruct for MessageSerializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let field_info = self.ser.field_info()?;
        append_tag(
            &mut self.ser.writer,
            field_info.number,
            field_info.ty.wire_type(),
        )
        .map_err(ser::Error::custom)?;
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if let Some((info, index)) = self.parent {
            self.ser.message_info = info;
            self.ser.field_index = index;
        }

        Ok(())
    }

    fn skip_field(&mut self, _key: &'static str) -> Result<(), Self::Error> {
        self.ser.field_index += 1;

        Ok(())
    }
}
