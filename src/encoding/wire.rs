use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::TryFrom;
use std::result::Result;

use crate::{field::FieldNumber, error::Error};

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i8)]
pub enum WireType {
    Varint = 0,
    Fixed32 = 5,
    Fixed64 = 1,
    Bytes = 2,
    StartGroup = 3,
    EndGroup = 4,
}

impl WireType {
    pub fn new(n: i8) -> Option<Self> {
        match n {
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
    type Error = Error;

    fn try_from(n: i8) -> Result<Self, Self::Error> {
        WireType::new(n).ok_or(Error::InvalidWireType(n))
    }
}

#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum WireValue {
    Varint(u64),
    Fixed32(u32),
    Fixed64(u64),
    Bytes(Bytes),
    Group(Vec<(FieldNumber, WireValue)>),
}

pub fn consume_field(b: &mut Bytes) -> Result<(FieldNumber, WireValue), Error> {
    let (num, typ) = consume_tag(b)?;
    Ok((num, consume_field_value(num, typ, b)?))
}

pub fn append_field(b: &mut BytesMut, num: FieldNumber, v: WireValue) {
    match v {
        WireValue::Varint(v) => {
            append_tag(b, num, WireType::Varint);
            append_varint(b, v);
        }
        WireValue::Fixed32(v) => {
            append_tag(b, num, WireType::Fixed32);
            append_fixed32(b, v);
        }
        WireValue::Fixed64(v) => {
            append_tag(b, num, WireType::Fixed64);
            append_fixed64(b, v);
        }
        WireValue::Bytes(v) => {
            append_tag(b, num, WireType::Bytes);
            append_bytes(b, v);
        }
        WireValue::Group(v) => {
            append_tag(b, num, WireType::StartGroup);
            append_group(b, num, v);
        }
    }
}

fn consume_field_value(
    num: FieldNumber,
    typ: WireType,
    b: &mut Bytes,
) -> Result<WireValue, Error> {
    match typ {
        WireType::Varint => Ok(WireValue::Varint(consume_varint(b)?)),
        WireType::Fixed32 => Ok(WireValue::Fixed32(consume_fixed32(b)?)),
        WireType::Fixed64 => Ok(WireValue::Fixed64(consume_fixed64(b)?)),
        WireType::Bytes => Ok(WireValue::Bytes(consume_bytes(b)?)),
        WireType::StartGroup => Ok(WireValue::Group(consume_group(num, b)?)),
        WireType::EndGroup => Err(Error::EndGroup),
    }
}

fn append_tag(b: &mut BytesMut, num: FieldNumber, typ: WireType) {
    append_varint(b, encode_tag(num, typ))
}

fn consume_tag(b: &mut Bytes) -> Result<(FieldNumber, WireType), Error> {
    Ok(decode_tag(consume_varint(b)?)?)
}

// fn size_tag(num: FieldNumber) -> usize {
//     size_varint(encode_tag(num, WireType::Varint))
// }

// Varints are a variable length encoding for a u64.
// To encode, a u64 is split every 7 bits and formed into a [u8] where the most
// significant bit of each u8 is '1' unless its the most significant non-zero u8.
fn append_varint(b: &mut BytesMut, v: u64) {
    let mut v = v;
    while v >= 0x80 {
        b.put_u8(((v & !0x80) | 0x80) as u8);
        v >>= 7;
    }
    b.put_u8(v as u8);
}

pub fn consume_varint(b: &mut Bytes) -> Result<u64, Error> {
    let mut y: u64 = 0;
    for i in 0..=9 {
        if b.is_empty() {
            return Err(Error::EOF);
        }

        let v = b.get_u8() as u64;
        // u64::MAX check
        if i == 9 && v > 1 {
            return Err(Error::Overflow);
        }

        y += (v & !0x80) << (7 * i);
        if v < 0x80 {
            return Ok(y);
        }
    }

    Err(Error::Overflow)
}

// fn size_varint(v: u64) -> usize {
//     // 1 + (bits_needed_to_represent(v) - 1)/ 7
//     1 + (63u32.saturating_sub(v.leading_zeros()) / 7) as usize
// }

fn append_fixed32(b: &mut BytesMut, v: u32) {
    b.put_u32_le(v);
}

fn consume_fixed32(b: &mut Bytes) -> Result<u32, Error> {
    if b.len() < 4 {
        return Err(Error::EOF);
    }

    Ok(b.get_u32_le())
}

// fn size_fixed32() -> usize {
//     4
// }

fn append_fixed64(b: &mut BytesMut, v: u64) {
    b.put_u64_le(v);
}

fn consume_fixed64(b: &mut Bytes) -> Result<u64, Error> {
    if b.len() < 8 {
        return Err(Error::EOF);
    }

    Ok(b.get_u64_le())
}

// fn size_fixed64() -> usize {
//     8
// }

fn append_bytes(b: &mut BytesMut, v: Bytes) {
    append_varint(b, v.len() as u64);
    b.put(v);
}

fn consume_bytes(b: &mut Bytes) -> Result<Bytes, Error> {
    let m = consume_varint(b)?;
    if m > b.len() as u64 {
        Err(Error::EOF)
    } else {
        Ok(b.split_to(m as usize))
    }
}

// fn size_bytes(n: usize) -> usize {
//     size_varint(n as u64) + n
// }

fn append_group(b: &mut BytesMut, num: FieldNumber, v: Vec<(FieldNumber, WireValue)>) {
    for (vn, vv) in v {
        append_field(b, vn, vv);
    }
    append_tag(b, num, WireType::EndGroup);
}

fn consume_group(num: FieldNumber, b: &mut Bytes) -> Result<Vec<(FieldNumber, WireValue)>, Error> {
    let mut v = Vec::new();
    loop {
        let (num2, typ2) = consume_tag(b)?;
        if typ2 == WireType::EndGroup {
            if num != num2 {
                return Err(Error::EndGroup);
            }
            return Ok(v);
        }
        v.push((num2, consume_field_value(num2, typ2, b)?));
    }
}

// fn size_group(num: FieldNumber, n: usize) -> usize {
//     n + size_tag(num)
// }

fn decode_tag(x: u64) -> Result<(FieldNumber, WireType), Error> {
    Ok((
        FieldNumber::try_from((x >> 3) as i32)?,
        WireType::try_from((x & 7) as i8)?,
    ))
}

fn encode_tag(num: FieldNumber, typ: WireType) -> u64 {
    ((num.get() as u64) << 3) | (typ as u64 & 7)
}

// pub fn decode_zig_zag(x: u64) -> i64 {
//     (x >> 1) as i64 ^ (x as i64) << 63 >> 63
// }

// pub fn encode_zig_zag(x: i64) -> u64 {
//     (x << 1) as u64 ^ (x >> 63) as u64
// }

// fn decode_bool(x: u64) -> bool {
//     x != 0
// }

// fn encode_bool(x: bool) -> u64 {
//     x as u64
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_end_mismatch() {
        let mut b = BytesMut::new();
        let num = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num, WireType::EndGroup);
        assert_eq!(consume_field(&mut b.freeze()), Err(Error::EndGroup));
    }

    #[test]
    fn field() {
        let mut b = BytesMut::new();
        let num1 = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num1, WireType::Varint);
        append_varint(&mut b, 0x123456789);

        let num2 = FieldNumber::try_from(2).unwrap();
        append_tag(&mut b, num2, WireType::Fixed32);
        append_fixed32(&mut b, 0x1234);

        let num3 = FieldNumber::try_from(3).unwrap();
        append_tag(&mut b, num3, WireType::Fixed64);
        append_fixed64(&mut b, 0x123456789);

        let num4 = FieldNumber::try_from(4).unwrap();
        append_tag(&mut b, num4, WireType::Bytes);
        append_bytes(&mut b, Bytes::from_static(b"hello"));

        let mut b = b.freeze();
        assert_eq!(
            consume_field(&mut b),
            Ok((num1, WireValue::Varint(0x123456789)))
        );
        assert_eq!(
            consume_field(&mut b),
            Ok((num2, WireValue::Fixed32(0x1234)))
        );
        assert_eq!(
            consume_field(&mut b),
            Ok((num3, WireValue::Fixed64(0x123456789)))
        );
        assert_eq!(
            consume_field(&mut b),
            Ok((num4, WireValue::Bytes(Bytes::from_static(b"hello"))))
        );
    }

    #[test]
    fn field_with_group() {
        let mut b = BytesMut::new();
        let num = FieldNumber::try_from(1).unwrap();
        let num_group = FieldNumber::try_from(5000).unwrap();
        append_tag(&mut b, num_group, WireType::StartGroup);
        append_group(
            &mut b,
            num_group,
            vec![(num, WireValue::Varint(0x123456789))],
        );

        let mut b = b.freeze();
        assert_eq!(
            consume_field(&mut b),
            Ok((
                num_group,
                WireValue::Group(vec![(num, WireValue::Varint(0x123456789))])
            ))
        );
    }
    #[test]
    fn group_eof() {
        let mut b = BytesMut::new();
        let num = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num, WireType::StartGroup);
        assert_eq!(consume_group(num, &mut b.freeze()), Err(Error::EOF));
    }

    #[test]
    fn group_nested_eof() {
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        let mut b = BytesMut::new();
        append_tag(&mut b, num2, WireType::StartGroup);
        append_tag(&mut b, num2, WireType::EndGroup);
        assert_eq!(consume_group(num1, &mut b.freeze()), Err(Error::EOF));
    }

    #[test]
    fn group_end_mismatch() {
        let mut b = BytesMut::new();
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        append_tag(&mut b, num2, WireType::EndGroup);
        assert_eq!(consume_group(num1, &mut b.freeze()), Err(Error::EndGroup));
    }

    #[test]
    fn group_nested_end_mismatch() {
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        let mut b = BytesMut::new();
        append_tag(&mut b, num2, WireType::StartGroup);
        append_tag(&mut b, num1, WireType::EndGroup);
        assert_eq!(consume_group(num1, &mut b.freeze()), Err(Error::EndGroup));
    }

    #[test]
    fn group() {
        let mut b = BytesMut::new();
        let num = FieldNumber::try_from(5).unwrap();
        append_tag(&mut b, num, WireType::Fixed32);
        append_fixed32(&mut b, 0xf0e1d2c3);
        append_tag(&mut b, num, WireType::EndGroup);
        assert_eq!(
            consume_group(num, &mut b.freeze()),
            Ok(vec![(num, WireValue::Fixed32(0xf0e1d2c3))])
        );
    }

    #[test]
    fn group_nested() {
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        let mut b = BytesMut::new();
        append_tag(&mut b, num2, WireType::StartGroup);
        append_tag(&mut b, num2, WireType::EndGroup);
        append_tag(&mut b, num1, WireType::EndGroup);
        assert_eq!(
            consume_group(num1, &mut b.freeze()),
            Ok(vec![(num2, WireValue::Group(Vec::new()))])
        );
    }

    #[test]
    fn group_empty() {
        let mut b = BytesMut::new();
        let num = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num, WireType::EndGroup);
        assert_eq!(consume_group(num, &mut b.freeze()), Ok(Vec::new()));
    }

    #[test]
    fn group_denormalized() {
        let mut b = BytesMut::new();
        let num = FieldNumber::try_from(5).unwrap();
        append_tag(&mut b, num, WireType::Fixed32);
        append_fixed32(&mut b, 0xf0e1d2c3);
        // manually end group
        b.extend_from_slice(b"\xac\x80\x80\x00");
        assert_eq!(
            consume_group(num, &mut b.freeze()),
            Ok(vec![(num, WireValue::Fixed32(0xf0e1d2c3))])
        );
    }

    #[test]
    fn varint_eof() {
        let mut b = Bytes::from_static(b"\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
        let mut b = Bytes::from_static(b"\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
        let mut b = Bytes::from_static(b"\x80\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
        let mut b = Bytes::from_static(b"\x80\x80\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
        let mut b = Bytes::from_static(b"\x80\x80\x80\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
        let mut b = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
        let mut b = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
        let mut b = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::EOF));
    }

    #[test]
    fn varint_overflow() {
        // Too many MSB's
        let mut b = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x80");
        assert_eq!(consume_varint(&mut b), Err(Error::Overflow));
        // Exceeds u64::MAX
        let mut b = Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x02");
        assert_eq!(consume_varint(&mut b), Err(Error::Overflow));
    }

    #[test]
    fn varint_boundaries() {
        let vals: Vec<(u64, &[u8])> = vec![
            (0x00, b"\x00"),
            (0x01, b"\x01"),
            (0x7f, b"\x7f"),
            (0x80, b"\x80\x01"),
            (0x3f_ff, b"\xff\x7f"),
            (0x40_00, b"\x80\x80\x01"),
            (0x1f_ff_ff, b"\xff\xff\x7f"),
            (0x20_00_00, b"\x80\x80\x80\x01"),
            (0x0f_ff_ff_ff, b"\xff\xff\xff\x7f"),
            (0x10_00_00_00, b"\x80\x80\x80\x80\x01"),
            (0x07_ff_ff_ff_ff, b"\xff\xff\xff\xff\x7f"),
            (0x08_00_00_00_00, b"\x80\x80\x80\x80\x80\x01"),
            (0x03_ff_ff_ff_ff_ff, b"\xff\xff\xff\xff\xff\x7f"),
            (0x04_00_00_00_00_00, b"\x80\x80\x80\x80\x80\x80\x01"),
            (0x01_ff_ff_ff_ff_ff_ff, b"\xff\xff\xff\xff\xff\xff\x7f"),
            (0x02_00_00_00_00_00_00, b"\x80\x80\x80\x80\x80\x80\x80\x01"),
            (0xff_ff_ff_ff_ff_ff_ff, b"\xff\xff\xff\xff\xff\xff\xff\x7f"),
            (
                0x01_00_00_00_00_00_00_00,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\x01",
            ),
            (
                0x7f_ff_ff_ff_ff_ff_ff_ff,
                b"\xff\xff\xff\xff\xff\xff\xff\xff\x7f",
            ),
            (
                0x80_00_00_00_00_00_00_00,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x01",
            ),
        ];

        for (v, raw) in vals {
            let mut b = BytesMut::new();
            append_varint(&mut b, v);
            assert_eq!(b, Bytes::from(raw));
            assert_eq!(consume_varint(&mut b.freeze()), Ok(v));
        }
    }

    #[test]
    fn varint_max() {
        let mut b = BytesMut::new();
        append_varint(&mut b, u64::MAX);
        assert_eq!(
            b,
            Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01")
        );
        assert_eq!(consume_varint(&mut b.freeze()), Ok(u64::MAX));
    }
    #[test]
    fn varint_denormalized() {
        let mut b = Bytes::from_static(b"\x01");
        assert_eq!(consume_varint(&mut b), Ok(1));
        let mut b = Bytes::from_static(b"\x81\x00");
        assert_eq!(consume_varint(&mut b), Ok(1));
        let mut b = Bytes::from_static(b"\x81\x80\x00");
        assert_eq!(consume_varint(&mut b), Ok(1));
        let mut b = Bytes::from_static(b"\x81\x80\x80\x00");
        assert_eq!(consume_varint(&mut b), Ok(1));
        let mut b = Bytes::from_static(b"\x81\x80\x80\x80\x00");
        assert_eq!(consume_varint(&mut b), Ok(1));
        let mut b = Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x00");
        assert_eq!(consume_varint(&mut b), Ok(1));
        let mut b = Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x80\x00");
        assert_eq!(consume_varint(&mut b), Ok(1));
        let mut b = Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x80\x80\x00");
        assert_eq!(consume_varint(&mut b), Ok(1));
    }

    #[test]
    fn bytes_eof() {
        let mut b = Bytes::from_static(b"");
        assert_eq!(consume_bytes(&mut b), Err(Error::EOF));

        let mut b = Bytes::from_static(b"\x01");
        assert_eq!(consume_bytes(&mut b), Err(Error::EOF));

        let mut b = Bytes::from_static(b"\x05hell");
        assert_eq!(consume_bytes(&mut b), Err(Error::EOF));
    }

    #[test]
    fn bytes_empty() {
        let mut b = BytesMut::new();
        append_bytes(&mut b, Bytes::new());
        assert_eq!(b, Bytes::from_static(b"\x00"));
        assert_eq!(consume_bytes(&mut b.freeze()), Ok(Bytes::from_static(b"")));
    }

    #[test]
    fn bytes_small() {
        let mut b = BytesMut::new();
        append_bytes(&mut b, Bytes::from_static(b"hello"));
        assert_eq!(b, Bytes::from_static(b"\x05hello"));
        assert_eq!(consume_bytes(&mut b.freeze()), Ok(Bytes::from_static(b"hello")));
    }

    #[test]
    fn bytes_large() {
        let v = Bytes::from(b"hello".repeat(50));
        let mut b = BytesMut::new();
        append_bytes(&mut b, v.clone());
        assert_eq!(b, Bytes::from([Bytes::from_static(b"\xfa\x01"), v.clone()].concat()));
        assert_eq!(consume_bytes(&mut b.freeze()), Ok(v));
    }

    #[test]
    fn fixed32_eof() {
        assert_eq!(consume_fixed32(&mut Bytes::from("")), Err(Error::EOF));
    }

    #[test]
    fn fixed32_min() {
        let mut b = BytesMut::new();
        let v = 0;
        append_fixed32(&mut b, v);
        assert_eq!(b, Bytes::from_static(b"\x00\x00\x00\x00"));
        assert_eq!(consume_fixed32(&mut b.freeze()), Ok(v));
    }

    #[test]
    fn fixed32_max() {
        let mut b = BytesMut::new();
        let v = 0xff_ff_ff_ff;
        append_fixed32(&mut b, v);
        assert_eq!(b, Bytes::from_static(b"\xff\xff\xff\xff"));
        assert_eq!(consume_fixed32(&mut b.freeze()), Ok(v));
    }

    #[test]
    fn fixed32() {
        let mut b = BytesMut::new();
        let v = 0xf0_e1_d2_c3;
        append_fixed32(&mut b, v);
        assert_eq!(b, Bytes::from_static(b"\xc3\xd2\xe1\xf0"));
        assert_eq!(consume_fixed32(&mut b.freeze()), Ok(v));
    }

    #[test]
    fn fixed64_eof() {
        assert_eq!(consume_fixed32(&mut Bytes::from("")), Err(Error::EOF));
    }

    #[test]
    fn fixed64_min() {
        let mut b = BytesMut::new();
        let v = 0;
        append_fixed64(&mut b, v);
        assert_eq!(b, Bytes::from_static(b"\x00\x00\x00\x00\x00\x00\x00\x00"));
        assert_eq!(consume_fixed64(&mut b.freeze()), Ok(v));
    }
    #[test]
    fn fixed64_max() {
        let mut b = BytesMut::new();
        let v = 0xff_ff_ff_ff_ff_ff_ff_ff;
        append_fixed64(&mut b, v);
        assert_eq!(b, Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\xff"));
        assert_eq!(consume_fixed64(&mut b.freeze()), Ok(v));
    }
    #[test]
    fn fixed64() {
        let mut b = BytesMut::new();
        let v = 0xf0_e1_d2_c3_b4_a5_96_87;
        append_fixed64(&mut b, v);
        assert_eq!(b, Bytes::from_static(b"\x87\x96\xa5\xb4\xc3\xd2\xe1\xf0"));
        assert_eq!(consume_fixed64(&mut b.freeze()), Ok(v));
    }

    #[test]
    fn tag_eof() {
        assert_eq!(consume_tag(&mut Bytes::from("")), Err(Error::EOF));
    }

    #[test]
    fn tag_invalid_field_type() {
        // number = 1, type = 6
        assert_eq!(
            consume_tag(&mut Bytes::from_static(b"\x0e")),
            Err(Error::InvalidWireType(6))
        );
    }

    #[test]
    fn tag_invalid_field_number() {
        // number = 0, type = 0
        assert_eq!(
            consume_tag(&mut Bytes::from_static(b"\x00")),
            Err(Error::InvalidFieldNumber(0))
        );
    }

    #[test]
    fn tag_min() {
        let mut b = BytesMut::new();
        append_tag(
            &mut b,
            FieldNumber::try_from(1).unwrap(),
            WireType::Fixed32,
        );
        assert_eq!(b, Bytes::from_static(b"\x0d"));
        assert_eq!(
            consume_tag(&mut b.freeze()),
            Ok((FieldNumber::try_from(1).unwrap(), WireType::Fixed32))
        );
    }

    #[test]
    fn tag_max() {
        let mut b = BytesMut::new();
        let max = FieldNumber::try_from((1 << 29) - 1).unwrap();
        append_tag(&mut b, max, WireType::Fixed32);
        assert_eq!(b, Bytes::from_static(b"\xfd\xff\xff\xff\x0f"));
        assert_eq!(
            consume_tag(&mut b.freeze()),
            Ok((max, WireType::Fixed32))
        );
    }

    // #[test]
    // fn zig_zag() {
    //     let vals: Vec<(i64, u64)> = vec![
    //         (i64::MIN, u64::MAX),
    //         (i64::MIN + 1, u64::MAX - 2),
    //         (-1, 1),
    //         (0, 0),
    //         (1, 2),
    //         (i64::MAX - 1, u64::MAX - 3),
    //         (i64::MAX, u64::MAX - 1),
    //     ];

    //     for (dec, enc) in vals {
    //         assert_eq!(encode_zig_zag(dec), enc);
    //         assert_eq!(decode_zig_zag(enc), dec);
    //     }
    // }
}
