use std::{convert::TryFrom, result::Result};

use bytes::{Buf, BufMut, Bytes};

use super::{error::DecodeError, field::FieldNumber};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireType {
    Varint = 0,
    Fixed32 = 5,
    Fixed64 = 1,
    Bytes = 2,
    StartGroup = 3,
    EndGroup = 4,
}

impl WireType {
    pub fn new(num: i8) -> Option<Self> {
        match num {
            0 => Some(WireType::Varint),
            5 => Some(WireType::Fixed32),
            1 => Some(WireType::Fixed64),
            2 => Some(WireType::Bytes),
            3 => Some(WireType::StartGroup),
            4 => Some(WireType::EndGroup),
            _ => None,
        }
    }

    pub const fn get(self) -> i8 {
        self as i8
    }
}

impl TryFrom<i8> for WireType {
    type Error = DecodeError;

    fn try_from(num: i8) -> Result<Self, Self::Error> {
        WireType::new(num).ok_or(DecodeError::InvalidWireType(num))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WireField {
    pub num: FieldNumber,
    pub val: FieldValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldValue {
    Varint(u64),
    Fixed32(u32),
    Fixed64(u64),
    Bytes(Bytes),
    StartGroup,
    EndGroup,
}

pub struct Parser {
    buf: Bytes,
}

impl Parser {
    pub fn new(buf: Bytes) -> Self {
        Parser { buf }
    }
}

impl Iterator for Parser {
    type Item = Result<WireField, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            Some(parse_next(&mut self.buf))
        }
    }
}

fn parse_next(buf: &mut Bytes) -> Result<WireField, DecodeError> {
    let (num, typ) = parse_tag(buf)?;
    let val = parse_wire_value(buf, typ)?;
    Ok(WireField { num, val })
}

fn parse_wire_value(buf: &mut Bytes, typ: WireType) -> Result<FieldValue, DecodeError> {
    match typ {
        WireType::Varint => Ok(FieldValue::Varint(parse_varint(buf)?)),
        WireType::Fixed32 => Ok(FieldValue::Fixed32(parse_fixed32(buf)?)),
        WireType::Fixed64 => Ok(FieldValue::Fixed64(parse_fixed64(buf)?)),
        WireType::Bytes => Ok(FieldValue::Bytes(parse_bytes(buf)?)),
        WireType::StartGroup => Ok(FieldValue::StartGroup),
        WireType::EndGroup => Ok(FieldValue::EndGroup),
    }
}

pub fn put_tag(buf: &mut impl BufMut, num: FieldNumber, typ: WireType) {
    put_varint(buf, encode_tag(num, typ));
}

fn parse_tag(buf: &mut Bytes) -> Result<(FieldNumber, WireType), DecodeError> {
    decode_tag(parse_varint(buf)?)
}

pub fn size_tag(num: FieldNumber) -> usize {
    size_varint(encode_tag(num, WireType::Varint))
}

// Varint is a variable length encoding for a u64.
// To encode, a u64 is split every 7 bits and formed into a [u8] where the most
// significant bit of each u8 is '1' unless its the most significant non-zero u8.
pub fn put_varint(buf: &mut impl BufMut, mut val: u64) {
    while val >= 0x80 {
        buf.put_u8(val as u8 | 0x80);
        val >>= 7;
    }
    buf.put_u8(val as u8);
}

pub fn parse_varint(buf: &mut Bytes) -> Result<u64, DecodeError> {
    let mut varint: u64 = 0;

    for index in 0..=9 {
        if buf.is_empty() {
            return Err(DecodeError::Eof);
        }

        let val = buf.get_u8();

        // u64::MAX check
        if index == 9 && val > 1 {
            break;
        }

        varint += (val as u64 & !0x80) << (7 * index);
        if val < 0x80 {
            return Ok(varint);
        }
    }

    Err(DecodeError::Overflow)
}

pub fn size_varint(num: u64) -> usize {
    // 1 + (bits_needed_to_represent(val) - 1)/ 7
    // 9/64 is a good enough approximation of 1/7 and easy to divide
    1 + (64u32 - num.leading_zeros()) as usize * 9 / 64
}

pub fn put_fixed32(buf: &mut impl BufMut, val: u32) {
    buf.put_u32_le(val);
}

fn parse_fixed32(buf: &mut Bytes) -> Result<u32, DecodeError> {
    if buf.len() < 4 {
        return Err(DecodeError::Eof);
    }

    Ok(buf.get_u32_le())
}

pub fn size_fixed32() -> usize {
    4
}

pub fn put_fixed64(buf: &mut impl BufMut, val: u64) {
    buf.put_u64_le(val);
}

fn parse_fixed64(buf: &mut Bytes) -> Result<u64, DecodeError> {
    if buf.len() < 8 {
        return Err(DecodeError::Eof);
    }

    Ok(buf.get_u64_le())
}

pub fn size_fixed64() -> usize {
    8
}

pub fn put_bytes(buf: &mut impl BufMut, val: &[u8]) {
    put_varint(buf, val.len() as u64);
    buf.put_slice(val);
}

fn parse_bytes(buf: &mut Bytes) -> Result<Bytes, DecodeError> {
    let len = parse_varint(buf)? as usize;
    if len > buf.len() {
        Err(DecodeError::Eof)
    } else {
        Ok(buf.split_to(len as usize))
    }
}

pub fn size_bytes(num: usize) -> usize {
    size_varint(num as u64) + num
}

pub fn size_group(num: FieldNumber, len: usize) -> usize {
    size_tag(num) + len
}

fn decode_tag(varint: u64) -> Result<(FieldNumber, WireType), DecodeError> {
    Ok((
        FieldNumber::try_from((varint >> 3) as i32)?,
        WireType::try_from((varint & 7) as i8)?,
    ))
}

pub fn encode_tag(num: FieldNumber, typ: WireType) -> u64 {
    ((num.get() as u64) << 3) | (typ as u64 & 7)
}

pub fn decode_zig_zag(varint: u64) -> i64 {
    (varint >> 1) as i64 ^ (varint as i64) << 63 >> 63
}

pub fn encode_zig_zag(varint: i64) -> u64 {
    (varint << 1) as u64 ^ (varint >> 63) as u64
}
