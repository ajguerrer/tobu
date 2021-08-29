use std::{convert::TryFrom, result::Result};

use bytes::{Buf, BufMut, Bytes};

use super::{error::Error, field::FieldNumber};

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
    type Error = Error;

    fn try_from(num: i8) -> Result<Self, Self::Error> {
        WireType::new(num).ok_or(Error::InvalidWireType(num))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WireValue {
    Varint(u64),
    Fixed32(u32),
    Fixed64(u64),
    Bytes(Bytes),
    Group(Vec<(FieldNumber, WireValue)>),
}

pub fn parse(buf: &mut Bytes) -> Result<Vec<(FieldNumber, WireValue)>, Error> {
    let mut fields = Vec::new();
    while !buf.is_empty() {
        fields.push(parse_field(buf)?);
    }

    Ok(fields)
}

pub fn parse_field(buf: &mut Bytes) -> Result<(FieldNumber, WireValue), Error> {
    let (num, typ) = parse_tag(buf)?;
    Ok((num, parse_field_value(num, typ, buf)?))
}

fn parse_field_value(num: FieldNumber, typ: WireType, buf: &mut Bytes) -> Result<WireValue, Error> {
    match typ {
        WireType::Varint => Ok(WireValue::Varint(parse_varint(buf)?)),
        WireType::Fixed32 => Ok(WireValue::Fixed32(parse_fixed32(buf)?)),
        WireType::Fixed64 => Ok(WireValue::Fixed64(parse_fixed64(buf)?)),
        WireType::Bytes => Ok(WireValue::Bytes(parse_bytes(buf)?)),
        WireType::StartGroup => Ok(WireValue::Group(parse_group(num, buf)?)),
        WireType::EndGroup => Err(Error::EndGroup),
    }
}

pub fn put_field(buf: &mut impl BufMut, num: FieldNumber, val: &WireValue) {
    match val {
        WireValue::Varint(val) => {
            put_tag(buf, num, WireType::Varint);
            put_varint(buf, *val);
        }
        WireValue::Fixed32(val) => {
            put_tag(buf, num, WireType::Fixed32);
            put_fixed32(buf, *val);
        }
        WireValue::Fixed64(val) => {
            put_tag(buf, num, WireType::Fixed64);
            put_fixed64(buf, *val);
        }
        WireValue::Bytes(val) => {
            put_tag(buf, num, WireType::Bytes);
            put_bytes(buf, val);
        }
        WireValue::Group(val) => {
            put_tag(buf, num, WireType::StartGroup);
            put_group(buf, num, val);
        }
    }
}

pub fn put_tag(buf: &mut impl BufMut, num: FieldNumber, typ: WireType) {
    put_varint(buf, encode_tag(num, typ));
}

fn parse_tag(buf: &mut Bytes) -> Result<(FieldNumber, WireType), Error> {
    decode_tag(parse_varint(buf)?)
}

pub fn size_tag(num: FieldNumber) -> usize {
    size_varint(encode_tag(num, WireType::Varint))
}

// Varint is a variable length encoding for a u64.
// To encode, a u64 is split every 7 bits and formed into a [u8] where the most
// significant bit of each u8 is '1' unless its the most significant non-zero u8.
pub fn put_varint(buf: &mut impl BufMut, val: u64) {
    let mut val = val;
    while val >= 0x80 {
        buf.put_u8(((val & !0x80) | 0x80) as u8);
        val >>= 7;
    }
    buf.put_u8(val as u8);
}

pub fn parse_varint(buf: &mut Bytes) -> Result<u64, Error> {
    let mut varint: u64 = 0;

    for index in 0..=9 {
        if buf.is_empty() {
            return Err(Error::Eof);
        }

        let val = buf.get_u8();

        // u64::MAX check
        if index == 9 && val > 1 {
            return Err(Error::Overflow);
        }

        varint += (val as u64 & !0x80) << (7 * index);
        if val < 0x80 {
            return Ok(varint);
        }
    }

    Err(Error::Overflow)
}

pub fn size_varint(num: u64) -> usize {
    // 1 + (bits_needed_to_represent(val) - 1)/ 7
    // 9/64 is a good enough approximation of 1/7 and easy to divide
    1 + (64u32 - num.leading_zeros()) as usize * 9 / 64
}

pub fn put_fixed32(buf: &mut impl BufMut, val: u32) {
    buf.put_u32_le(val);
}

pub fn parse_fixed32(buf: &mut Bytes) -> Result<u32, Error> {
    if buf.len() < 4 {
        return Err(Error::Eof);
    }

    Ok(buf.get_u32_le())
}

pub fn size_fixed32() -> usize {
    4
}

pub fn put_fixed64(buf: &mut impl BufMut, val: u64) {
    buf.put_u64_le(val);
}

pub fn parse_fixed64(buf: &mut Bytes) -> Result<u64, Error> {
    if buf.len() < 8 {
        return Err(Error::Eof);
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

pub fn parse_bytes(buf: &mut Bytes) -> Result<Bytes, Error> {
    let len = parse_varint(buf)? as usize;
    if len > buf.len() {
        Err(Error::Eof)
    } else {
        Ok(buf.split_to(len as usize))
    }
}

pub fn size_bytes(num: usize) -> usize {
    size_varint(num as u64) + num
}

pub fn put_group(buf: &mut impl BufMut, num: FieldNumber, fields: &[(FieldNumber, WireValue)]) {
    for (field_num, field_val) in fields {
        put_field(buf, *field_num, field_val);
    }
    put_tag(buf, num, WireType::EndGroup)
}

pub fn parse_group(
    num: FieldNumber,
    buf: &mut Bytes,
) -> Result<Vec<(FieldNumber, WireValue)>, Error> {
    let mut fields = Vec::new();
    loop {
        let (num2, typ2) = parse_tag(buf)?;
        if typ2 == WireType::EndGroup {
            if num != num2 {
                return Err(Error::EndGroup);
            }
            return Ok(fields);
        }
        fields.push((num2, parse_field_value(num2, typ2, buf)?));
    }
}

pub fn size_group(num: FieldNumber, len: usize) -> usize {
    len + size_tag(num)
}

fn decode_tag(varint: u64) -> Result<(FieldNumber, WireType), Error> {
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

pub fn decode_bool(varint: u64) -> bool {
    varint != 0
}

pub fn encode_bool(varint: bool) -> u64 {
    if varint {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    #[test]
    fn field_end_mismatch() {
        let mut buf = BytesMut::new();

        put_tag(
            &mut buf,
            FieldNumber::try_from(1).unwrap(),
            WireType::EndGroup,
        );

        assert!(matches!(
            parse_field(&mut buf.freeze()),
            Err(Error::EndGroup)
        ));
    }

    #[test]
    fn field() {
        let mut buf = BytesMut::new();

        let num1 = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num1, WireType::Varint);
        let val1 = 0x123456789;
        put_varint(&mut buf, val1);

        let num2 = FieldNumber::try_from(2).unwrap();
        put_tag(&mut buf, num2, WireType::Fixed32);
        let val2 = 0x1234;
        put_fixed32(&mut buf, val2);

        let num3 = FieldNumber::try_from(3).unwrap();
        put_tag(&mut buf, num3, WireType::Fixed64);
        let val3 = 0x123456789;
        put_fixed64(&mut buf, val3);

        let num4 = FieldNumber::try_from(4).unwrap();
        put_tag(&mut buf, num4, WireType::Bytes);
        let val4 = Bytes::from_static(b"hello");
        put_bytes(&mut buf, &val4);

        let mut buf = buf.freeze();
        assert_eq!(
            parse_field(&mut buf).unwrap(),
            (num1, WireValue::Varint(val1))
        );
        assert_eq!(
            parse_field(&mut buf).unwrap(),
            (num2, WireValue::Fixed32(val2))
        );
        assert_eq!(
            parse_field(&mut buf).unwrap(),
            (num3, WireValue::Fixed64(val3))
        );
        assert_eq!(
            parse_field(&mut buf).unwrap(),
            (num4, WireValue::Bytes(val4))
        );
    }

    #[test]
    fn field_with_group() {
        let mut buf = BytesMut::new();

        let group_num = FieldNumber::try_from(5000).unwrap();
        put_tag(&mut buf, group_num, WireType::StartGroup);

        let group = vec![(
            FieldNumber::try_from(1).unwrap(),
            WireValue::Varint(0x123456789),
        )];
        put_group(&mut buf, group_num, &group);

        assert_eq!(
            parse_field(&mut buf.freeze()).unwrap(),
            (group_num, WireValue::Group(group))
        );
    }

    #[test]
    fn group_eof() {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::StartGroup);

        assert!(matches!(
            parse_group(num, &mut buf.freeze()),
            Err(Error::Eof)
        ));
    }

    #[test]
    fn group_nested_eof() {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::StartGroup);
        put_tag(&mut buf, num, WireType::EndGroup);

        let wrong_num = FieldNumber::try_from(2).unwrap();
        assert!(matches!(
            parse_group(wrong_num, &mut buf.freeze()),
            Err(Error::Eof)
        ));
    }

    #[test]
    fn group_end_mismatch() {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::EndGroup);

        let wrong_num = FieldNumber::try_from(2).unwrap();
        assert!(matches!(
            parse_group(wrong_num, &mut buf.freeze()),
            Err(Error::EndGroup)
        ));
    }

    #[test]
    fn group_nested_end_mismatch() {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::StartGroup);

        let wrong_num = FieldNumber::try_from(2).unwrap();
        put_tag(&mut buf, wrong_num, WireType::EndGroup);

        assert!(matches!(
            parse_group(wrong_num, &mut buf.freeze()),
            Err(Error::EndGroup)
        ));
    }

    #[test]
    fn group() {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(5).unwrap();
        put_tag(&mut buf, num, WireType::Fixed32);

        let val = 0xf0e1d2c3;
        put_fixed32(&mut buf, val);

        put_tag(&mut buf, num, WireType::EndGroup);

        assert_eq!(
            parse_group(num, &mut buf.freeze()).unwrap(),
            vec![(num, WireValue::Fixed32(val))]
        );
    }

    #[test]
    fn group_nested() {
        let mut buf = BytesMut::new();

        let nested_num = FieldNumber::try_from(2).unwrap();
        put_tag(&mut buf, nested_num, WireType::StartGroup);
        put_tag(&mut buf, nested_num, WireType::EndGroup);

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::EndGroup);

        assert_eq!(
            parse_group(num, &mut buf.freeze()).unwrap(),
            vec![(nested_num, WireValue::Group(Vec::new()))]
        );
    }

    #[test]
    fn group_empty() {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::EndGroup);

        assert_eq!(parse_group(num, &mut buf.freeze()).unwrap(), Vec::new());
    }

    #[test]
    fn group_denormalized() {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(5).unwrap();
        put_tag(&mut buf, num, WireType::Fixed32);

        let val = 0xf0e1d2c3;
        put_fixed32(&mut buf, val);

        // manually end group
        buf.extend_from_slice(b"\xac\x80\x80\x00");

        assert_eq!(
            parse_group(num, &mut buf.freeze()).unwrap(),
            vec![(num, WireValue::Fixed32(val))]
        );
    }

    #[test]
    fn varint_eof() {
        let mut buf = Bytes::from_static(b"\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x80\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x80\x80\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x80\x80\x80\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Eof)));
    }

    #[test]
    fn varint_overflow() {
        // Too many bytes with significant bits
        let mut buf = Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x80");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Overflow)));
        // Exceeds u64::MAX
        let mut buf = Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x02");
        assert!(matches!(parse_varint(&mut buf), Err(Error::Overflow)));
    }

    #[test]
    fn varint_boundaries() {
        let values = vec![
            (0x00, Bytes::from_static(b"\x00")),
            (0x01, Bytes::from_static(b"\x01")),
            (0x7f, Bytes::from_static(b"\x7f")),
            (0x80, Bytes::from_static(b"\x80\x01")),
            (0x3f_ff, Bytes::from_static(b"\xff\x7f")),
            (0x40_00, Bytes::from_static(b"\x80\x80\x01")),
            (0x1f_ff_ff, Bytes::from_static(b"\xff\xff\x7f")),
            (0x20_00_00, Bytes::from_static(b"\x80\x80\x80\x01")),
            (0x0f_ff_ff_ff, Bytes::from_static(b"\xff\xff\xff\x7f")),
            (0x10_00_00_00, Bytes::from_static(b"\x80\x80\x80\x80\x01")),
            (
                0x07_ff_ff_ff_ff,
                Bytes::from_static(b"\xff\xff\xff\xff\x7f"),
            ),
            (
                0x08_00_00_00_00,
                Bytes::from_static(b"\x80\x80\x80\x80\x80\x01"),
            ),
            (
                0x03_ff_ff_ff_ff_ff,
                Bytes::from_static(b"\xff\xff\xff\xff\xff\x7f"),
            ),
            (
                0x04_00_00_00_00_00,
                Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x01"),
            ),
            (
                0x01_ff_ff_ff_ff_ff_ff,
                Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\x7f"),
            ),
            (
                0x02_00_00_00_00_00_00,
                Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x01"),
            ),
            (
                0xff_ff_ff_ff_ff_ff_ff,
                Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\x7f"),
            ),
            (
                0x01_00_00_00_00_00_00_00,
                Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x01"),
            ),
            (
                0x7f_ff_ff_ff_ff_ff_ff_ff,
                Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\xff\x7f"),
            ),
            (
                0x80_00_00_00_00_00_00_00,
                Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x01"),
            ),
        ];

        for (val, raw) in values {
            let mut buf = BytesMut::new();
            put_varint(&mut buf, val);
            assert_eq!(buf, raw);
            assert_eq!(parse_varint(&mut buf.freeze()).unwrap(), val);
        }
    }

    #[test]
    fn varint_max() {
        let mut buf = BytesMut::new();

        put_varint(&mut buf, u64::MAX);
        assert_eq!(
            buf,
            Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01")
        );

        assert_eq!(parse_varint(&mut buf.freeze()).unwrap(), u64::MAX);
    }

    #[test]
    fn varint_denormalized() {
        let mut buf = Bytes::from_static(b"\x01");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);

        let mut buf = Bytes::from_static(b"\x81\x00");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);

        let mut buf = Bytes::from_static(b"\x81\x80\x00");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);

        let mut buf = Bytes::from_static(b"\x81\x80\x80\x00");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);

        let mut buf = Bytes::from_static(b"\x81\x80\x80\x80\x00");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);

        let mut buf = Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x00");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);

        let mut buf = Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x80\x00");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);

        let mut buf = Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x80\x80\x00");
        assert_eq!(parse_varint(&mut buf).unwrap(), 1);
    }

    #[test]
    fn bytes_eof() {
        let mut buf = Bytes::from_static(b"");
        assert!(matches!(parse_bytes(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x01");
        assert!(matches!(parse_bytes(&mut buf), Err(Error::Eof)));

        let mut buf = Bytes::from_static(b"\x05hell");
        assert!(matches!(parse_bytes(&mut buf), Err(Error::Eof)));
    }

    #[test]
    fn bytes_empty() {
        let mut buf = BytesMut::new();

        put_bytes(&mut buf, b"");
        assert_eq!(buf, Bytes::from_static(b"\x00"));

        assert_eq!(
            parse_bytes(&mut buf.freeze()).unwrap(),
            Bytes::from_static(b"")
        );
    }

    #[test]
    fn bytes_small() {
        let mut buf = BytesMut::new();

        put_bytes(&mut buf, &Bytes::from_static(b"hello"));

        assert_eq!(buf, Bytes::from_static(b"\x05hello"));
        assert_eq!(
            parse_bytes(&mut buf.freeze()).unwrap(),
            Bytes::from_static(b"hello")
        );
    }

    #[test]
    fn bytes_large() {
        let mut buf = BytesMut::new();

        let val = Bytes::from(b"hello".repeat(50));
        put_bytes(&mut buf, &val);

        assert_eq!(
            buf,
            Bytes::from([Bytes::from_static(b"\xfa\x01"), val.clone()].concat())
        );
        assert_eq!(parse_bytes(&mut buf.freeze()).unwrap(), &val);
    }

    #[test]
    fn fixed32_eof() {
        assert!(matches!(
            parse_fixed32(&mut Bytes::from_static(b"")),
            Err(Error::Eof)
        ));
    }

    #[test]
    fn fixed32_min() {
        let mut buf = BytesMut::new();

        let val = 0;
        put_fixed32(&mut buf, val);

        assert_eq!(buf, Bytes::from_static(b"\x00\x00\x00\x00"));
        assert_eq!(parse_fixed32(&mut buf.freeze()).unwrap(), val);
    }

    #[test]
    fn fixed32_max() {
        let mut buf = BytesMut::new();

        let val = 0xff_ff_ff_ff;
        put_fixed32(&mut buf, val);

        assert_eq!(buf, Bytes::from_static(b"\xff\xff\xff\xff"));
        assert_eq!(parse_fixed32(&mut buf.freeze()).unwrap(), val);
    }

    #[test]
    fn fixed32() {
        let mut buf = BytesMut::new();

        let val = 0xf0_e1_d2_c3;
        put_fixed32(&mut buf, val);

        assert_eq!(buf, Bytes::from_static(b"\xc3\xd2\xe1\xf0"));
        assert_eq!(parse_fixed32(&mut buf.freeze()).unwrap(), val);
    }

    #[test]
    fn fixed64_eof() {
        assert!(matches!(
            parse_fixed32(&mut Bytes::from_static(b"")),
            Err(Error::Eof)
        ));
    }

    #[test]
    fn fixed64_min() {
        let mut buf = BytesMut::new();

        let val = 0;
        put_fixed64(&mut buf, val);

        assert_eq!(buf, Bytes::from_static(b"\x00\x00\x00\x00\x00\x00\x00\x00"));
        assert_eq!(parse_fixed64(&mut buf.freeze()).unwrap(), val);
    }

    #[test]
    fn fixed64_max() {
        let mut buf = BytesMut::new();

        let val = 0xff_ff_ff_ff_ff_ff_ff_ff;
        put_fixed64(&mut buf, val);

        assert_eq!(buf, Bytes::from_static(b"\xff\xff\xff\xff\xff\xff\xff\xff"));
        assert_eq!(parse_fixed64(&mut buf.freeze()).unwrap(), val);
    }

    #[test]
    fn fixed64() {
        let mut buf = BytesMut::new();

        let val = 0xf0_e1_d2_c3_b4_a5_96_87;
        put_fixed64(&mut buf, val);

        assert_eq!(buf, Bytes::from_static(b"\x87\x96\xa5\xb4\xc3\xd2\xe1\xf0"));
        assert_eq!(parse_fixed64(&mut buf.freeze()).unwrap(), val);
    }

    #[test]
    fn tag_eof() {
        assert!(matches!(
            parse_tag(&mut Bytes::from_static(b"")),
            Err(Error::Eof)
        ));
    }

    #[test]
    fn tag_invalid_field_type() {
        // num = 1, typ = 6
        assert!(matches!(
            parse_tag(&mut Bytes::from_static(b"\x0e")),
            Err(Error::InvalidWireType(6))
        ));
    }

    #[test]
    fn tag_invalid_field_number() {
        // num = 0, typ = 0
        assert!(matches!(
            parse_tag(&mut Bytes::from_static(b"\x00")),
            Err(Error::InvalidFieldNumber(0))
        ));
    }

    #[test]
    fn tag_min() {
        let mut buf = BytesMut::new();

        put_tag(
            &mut buf,
            FieldNumber::try_from(1).unwrap(),
            WireType::Fixed32,
        );

        assert_eq!(buf, Bytes::from_static(b"\x0d"));
        assert_eq!(
            parse_tag(&mut buf.freeze()).unwrap(),
            (FieldNumber::try_from(1).unwrap(), WireType::Fixed32)
        );
    }

    #[test]
    fn tag_max() {
        let mut buf = BytesMut::new();

        let max = FieldNumber::try_from((1 << 29) - 1).unwrap();
        put_tag(&mut buf, max, WireType::Fixed32);

        assert_eq!(buf, Bytes::from_static(b"\xfd\xff\xff\xff\x0f"));
        assert_eq!(
            parse_tag(&mut buf.freeze()).unwrap(),
            (max, WireType::Fixed32)
        );
    }

    #[test]
    fn zig_zag() {
        let values: Vec<(i64, u64)> = vec![
            (i64::MIN, u64::MAX),
            (i64::MIN + 1, u64::MAX - 2),
            (-1, 1),
            (0, 0),
            (1, 2),
            (i64::MAX - 1, u64::MAX - 3),
            (i64::MAX, u64::MAX - 1),
        ];

        for (dec, enc) in values {
            assert_eq!(encode_zig_zag(dec), enc);
            assert_eq!(decode_zig_zag(enc), dec);
        }
    }
}
