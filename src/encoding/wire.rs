use std::{convert::TryFrom, io, result::Result};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WireValue {
    Varint(u64),
    Fixed32(u32),
    Fixed64(u64),
    Bytes(Vec<u8>),
    Group(Vec<(FieldNumber, WireValue)>),
}

pub fn consume_field(b: &mut impl io::Read) -> Result<(FieldNumber, WireValue), Error> {
    let (num, typ) = consume_tag(b)?;
    Ok((num, consume_field_value(num, typ, b)?))
}

pub fn append_field(b: &mut impl io::Write, num: FieldNumber, v: WireValue) -> Result<(), Error> {
    match v {
        WireValue::Varint(v) => {
            append_tag(b, num, WireType::Varint)?;
            append_varint(b, v)?;
        }
        WireValue::Fixed32(v) => {
            append_tag(b, num, WireType::Fixed32)?;
            append_fixed32(b, v)?;
        }
        WireValue::Fixed64(v) => {
            append_tag(b, num, WireType::Fixed64)?;
            append_fixed64(b, v)?;
        }
        WireValue::Bytes(v) => {
            append_tag(b, num, WireType::Bytes)?;
            append_bytes(b, &v)?;
        }
        WireValue::Group(v) => {
            append_tag(b, num, WireType::StartGroup)?;
            append_group(b, num, v)?;
        }
    }
    Ok(())
}

fn consume_field_value(
    num: FieldNumber,
    typ: WireType,
    b: &mut impl io::Read,
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

pub fn append_tag(b: &mut impl io::Write, num: FieldNumber, typ: WireType) -> Result<(), Error> {
    append_varint(b, encode_tag(num, typ))
}

fn consume_tag(b: &mut impl io::Read) -> Result<(FieldNumber, WireType), Error> {
    decode_tag(consume_varint(b)?)
}

pub fn size_tag(num: FieldNumber) -> usize {
    size_varint(encode_tag(num, WireType::Varint))
}

// Varints are a variable length encoding for a u64.
// To encode, a u64 is split every 7 bits and formed into a [u8] where the most
// significant bit of each u8 is '1' unless its the most significant non-zero u8.
pub fn append_varint(b: &mut impl io::Write, v: u64) -> Result<(), Error> {
    let mut v = v;
    while v >= 0x80 {
        b.write(&[((v & !0x80) | 0x80) as u8])?;
        v >>= 7;
    }
    b.write(&[v as u8])?;
    Ok(())
}

pub fn consume_varint(b: &mut impl io::Read) -> Result<u64, Error> {
    let mut y: u64 = 0;

    for i in 0..=9 {
        let mut v = 0;
        if b.read(std::slice::from_mut(&mut v))? != 1 {
            return Err(Error::Eof);
        }
        // u64::MAX check
        if i == 9 && v > 1 {
            return Err(Error::Overflow);
        }

        y += (v as u64 & !0x80) << (7 * i);
        if v < 0x80 {
            return Ok(y);
        }
    }

    Err(Error::Overflow)
}

pub fn size_varint(n: u64) -> usize {
    // 1 + (bits_needed_to_represent(v) - 1)/ 7
    // 9/64 is a good enough approximation of 1/7 and easy to divide
    1 + (64u32 - n.leading_zeros()) as usize * 9 / 64
}

pub fn append_fixed32(b: &mut impl io::Write, v: u32) -> Result<(), Error> {
    b.write(&v.to_le_bytes())?;
    Ok(())
}

pub fn consume_fixed32(b: &mut impl io::Read) -> Result<u32, Error> {
    let mut v = [0; 4];
    if b.read(&mut v)? != 4 {
        return Err(Error::Eof);
    }

    Ok(u32::from_le_bytes(v))
}

pub fn size_fixed32() -> usize {
    4
}

pub fn append_fixed64(b: &mut impl io::Write, v: u64) -> Result<(), Error> {
    b.write(&v.to_le_bytes())?;
    Ok(())
}

pub fn consume_fixed64(b: &mut impl io::Read) -> Result<u64, Error> {
    let mut v = [0; 8];
    if b.read(&mut v)? != 8 {
        return Err(Error::Eof);
    }

    Ok(u64::from_le_bytes(v))
}

pub fn size_fixed64() -> usize {
    8
}

pub fn append_bytes(b: &mut impl io::Write, v: &[u8]) -> Result<(), Error> {
    append_varint(b, v.len() as u64)?;
    b.write(v)?;
    Ok(())
}

pub fn consume_bytes(b: &mut impl io::Read) -> Result<Vec<u8>, Error> {
    let len = consume_varint(b)? as usize;
    let mut v = vec![0; len];
    if dbg!(b.read(v.as_mut_slice())?) != len {
        Err(Error::Eof)
    } else {
        Ok(v)
    }
}

pub fn size_bytes(n: usize) -> usize {
    size_varint(n as u64) + n
}

pub fn append_group(
    b: &mut impl io::Write,
    num: FieldNumber,
    v: Vec<(FieldNumber, WireValue)>,
) -> Result<(), Error> {
    for (vn, vv) in v {
        append_field(b, vn, vv)?;
    }
    append_tag(b, num, WireType::EndGroup)
}

pub fn consume_group(
    num: FieldNumber,
    b: &mut impl io::Read,
) -> Result<Vec<(FieldNumber, WireValue)>, Error> {
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

pub fn size_group(num: FieldNumber, n: usize) -> usize {
    n + size_tag(num)
}

fn decode_tag(x: u64) -> Result<(FieldNumber, WireType), Error> {
    Ok((
        FieldNumber::try_from((x >> 3) as i32)?,
        WireType::try_from((x & 7) as i8)?,
    ))
}

pub fn encode_tag(num: FieldNumber, typ: WireType) -> u64 {
    ((num.get() as u64) << 3) | (typ as u64 & 7)
}

pub fn decode_zig_zag(x: u64) -> i64 {
    (x >> 1) as i64 ^ (x as i64) << 63 >> 63
}

pub fn encode_zig_zag(x: i64) -> u64 {
    (x << 1) as u64 ^ (x >> 63) as u64
}

pub fn decode_bool(x: u64) -> bool {
    x != 0
}

pub fn encode_bool(x: bool) -> u64 {
    if x {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_end_mismatch() -> Result<(), Error> {
        let mut b = Vec::new();
        let num = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num, WireType::EndGroup)?;
        assert!(matches!(
            consume_field(&mut b.as_slice()),
            Err(Error::EndGroup)
        ));

        Ok(())
    }

    #[test]
    fn field() -> Result<(), Error> {
        let mut b = Vec::new();
        let num1 = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num1, WireType::Varint)?;
        append_varint(&mut b, 0x123456789)?;

        let num2 = FieldNumber::try_from(2).unwrap();
        append_tag(&mut b, num2, WireType::Fixed32)?;
        append_fixed32(&mut b, 0x1234)?;

        let num3 = FieldNumber::try_from(3).unwrap();
        append_tag(&mut b, num3, WireType::Fixed64)?;
        append_fixed64(&mut b, 0x123456789)?;

        let num4 = FieldNumber::try_from(4).unwrap();
        append_tag(&mut b, num4, WireType::Bytes)?;
        append_bytes(&mut b, b"hello")?;

        let mut b = b.as_slice();
        assert_eq!(
            consume_field(&mut b)?,
            (num1, WireValue::Varint(0x123456789))
        );
        assert_eq!(consume_field(&mut b)?, (num2, WireValue::Fixed32(0x1234)));
        assert_eq!(
            consume_field(&mut b)?,
            (num3, WireValue::Fixed64(0x123456789))
        );
        assert_eq!(
            consume_field(&mut b)?,
            (num4, WireValue::Bytes(b"hello".to_vec()))
        );

        Ok(())
    }

    #[test]
    fn field_with_group() -> Result<(), Error> {
        let mut b = Vec::new();
        let num = FieldNumber::try_from(1).unwrap();
        let num_group = FieldNumber::try_from(5000).unwrap();
        append_tag(&mut b, num_group, WireType::StartGroup)?;
        append_group(
            &mut b,
            num_group,
            vec![(num, WireValue::Varint(0x123456789))],
        )?;

        let mut b = b.as_slice();
        assert_eq!(
            consume_field(&mut b)?,
            (
                num_group,
                WireValue::Group(vec![(num, WireValue::Varint(0x123456789))])
            )
        );

        Ok(())
    }
    #[test]
    fn group_eof() -> Result<(), Error> {
        let mut b = Vec::new();
        let num = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num, WireType::StartGroup)?;
        assert!(matches!(
            consume_group(num, &mut b.as_slice()),
            Err(Error::Eof)
        ));

        Ok(())
    }

    #[test]
    fn group_nested_eof() -> Result<(), Error> {
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        let mut b = Vec::new();
        append_tag(&mut b, num2, WireType::StartGroup)?;
        append_tag(&mut b, num2, WireType::EndGroup)?;
        assert!(matches!(
            consume_group(num1, &mut b.as_slice()),
            Err(Error::Eof)
        ));

        Ok(())
    }

    #[test]
    fn group_end_mismatch() -> Result<(), Error> {
        let mut b = Vec::new();
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        append_tag(&mut b, num2, WireType::EndGroup)?;
        assert!(matches!(
            consume_group(num1, &mut b.as_slice()),
            Err(Error::EndGroup)
        ));

        Ok(())
    }

    #[test]
    fn group_nested_end_mismatch() -> Result<(), Error> {
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        let mut b = Vec::new();
        append_tag(&mut b, num2, WireType::StartGroup)?;
        append_tag(&mut b, num1, WireType::EndGroup)?;
        assert!(matches!(
            consume_group(num1, &mut b.as_slice()),
            Err(Error::EndGroup)
        ));

        Ok(())
    }

    #[test]
    fn group() -> Result<(), Error> {
        let mut b = Vec::new();
        let num = FieldNumber::try_from(5).unwrap();
        append_tag(&mut b, num, WireType::Fixed32)?;
        append_fixed32(&mut b, 0xf0e1d2c3)?;
        append_tag(&mut b, num, WireType::EndGroup)?;
        assert_eq!(
            consume_group(num, &mut b.as_slice())?,
            vec![(num, WireValue::Fixed32(0xf0e1d2c3))]
        );

        Ok(())
    }

    #[test]
    fn group_nested() -> Result<(), Error> {
        let num1 = FieldNumber::try_from(1).unwrap();
        let num2 = FieldNumber::try_from(2).unwrap();
        let mut b = Vec::new();
        append_tag(&mut b, num2, WireType::StartGroup)?;
        append_tag(&mut b, num2, WireType::EndGroup)?;
        append_tag(&mut b, num1, WireType::EndGroup)?;
        assert_eq!(
            consume_group(num1, &mut b.as_slice())?,
            vec![(num2, WireValue::Group(Vec::new()))]
        );

        Ok(())
    }

    #[test]
    fn group_empty() -> Result<(), Error> {
        let mut b = Vec::new();
        let num = FieldNumber::try_from(1).unwrap();
        append_tag(&mut b, num, WireType::EndGroup)?;
        assert_eq!(consume_group(num, &mut b.as_slice())?, Vec::new());

        Ok(())
    }

    #[test]
    fn group_denormalized() -> Result<(), Error> {
        let mut b = Vec::new();
        let num = FieldNumber::try_from(5).unwrap();
        append_tag(&mut b, num, WireType::Fixed32)?;
        append_fixed32(&mut b, 0xf0e1d2c3)?;
        // manually end group
        b.extend_from_slice(b"\xac\x80\x80\x00");
        assert_eq!(
            consume_group(num, &mut b.as_slice())?,
            vec![(num, WireValue::Fixed32(0xf0e1d2c3))]
        );

        Ok(())
    }

    #[test]
    fn varint_eof() {
        let mut b = b"\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
        let mut b = b"\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
        let mut b = b"\x80\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
        let mut b = b"\x80\x80\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
        let mut b = b"\x80\x80\x80\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
        let mut b = b"\x80\x80\x80\x80\x80\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
        let mut b = b"\x80\x80\x80\x80\x80\x80\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
        let mut b = b"\x80\x80\x80\x80\x80\x80\x80\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Eof)));
    }

    #[test]
    fn varint_overflow() {
        // Too many MSB's
        let mut b = b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x80".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Overflow)));
        // Exceeds u64::MAX
        let mut b = b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x02".as_ref();
        assert!(matches!(consume_varint(&mut b), Err(Error::Overflow)));
    }

    #[test]
    fn varint_boundaries() -> Result<(), Error> {
        let vals = vec![
            (0x00, b"\x00".as_ref()),
            (0x01, b"\x01".as_ref()),
            (0x7f, b"\x7f".as_ref()),
            (0x80, b"\x80\x01".as_ref()),
            (0x3f_ff, b"\xff\x7f".as_ref()),
            (0x40_00, b"\x80\x80\x01".as_ref()),
            (0x1f_ff_ff, b"\xff\xff\x7f".as_ref()),
            (0x20_00_00, b"\x80\x80\x80\x01".as_ref()),
            (0x0f_ff_ff_ff, b"\xff\xff\xff\x7f".as_ref()),
            (0x10_00_00_00, b"\x80\x80\x80\x80\x01".as_ref()),
            (0x07_ff_ff_ff_ff, b"\xff\xff\xff\xff\x7f".as_ref()),
            (0x08_00_00_00_00, b"\x80\x80\x80\x80\x80\x01".as_ref()),
            (0x03_ff_ff_ff_ff_ff, b"\xff\xff\xff\xff\xff\x7f".as_ref()),
            (
                0x04_00_00_00_00_00,
                b"\x80\x80\x80\x80\x80\x80\x01".as_ref(),
            ),
            (
                0x01_ff_ff_ff_ff_ff_ff,
                b"\xff\xff\xff\xff\xff\xff\x7f".as_ref(),
            ),
            (
                0x02_00_00_00_00_00_00,
                b"\x80\x80\x80\x80\x80\x80\x80\x01".as_ref(),
            ),
            (
                0xff_ff_ff_ff_ff_ff_ff,
                b"\xff\xff\xff\xff\xff\xff\xff\x7f".as_ref(),
            ),
            (
                0x01_00_00_00_00_00_00_00,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\x01".as_ref(),
            ),
            (
                0x7f_ff_ff_ff_ff_ff_ff_ff,
                b"\xff\xff\xff\xff\xff\xff\xff\xff\x7f".as_ref(),
            ),
            (
                0x80_00_00_00_00_00_00_00,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x01".as_ref(),
            ),
        ];

        for (v, raw) in vals {
            let mut b = Vec::new();
            append_varint(&mut b, v)?;
            assert_eq!(b, raw);
            assert_eq!(consume_varint(&mut b.as_slice())?, v);
        }

        Ok(())
    }

    #[test]
    fn varint_max() -> Result<(), Error> {
        let mut b = Vec::new();
        append_varint(&mut b, u64::MAX)?;
        assert_eq!(b, b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01".as_ref());
        assert_eq!(consume_varint(&mut b.as_slice())?, u64::MAX);

        Ok(())
    }

    #[test]
    fn varint_denormalized() -> Result<(), Error> {
        let mut b = b"\x01".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);
        let mut b = b"\x81\x00".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);
        let mut b = b"\x81\x80\x00".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);
        let mut b = b"\x81\x80\x80\x00".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);
        let mut b = b"\x81\x80\x80\x80\x00".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);
        let mut b = b"\x81\x80\x80\x80\x80\x80\x00".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);
        let mut b = b"\x81\x80\x80\x80\x80\x80\x80\x00".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);
        let mut b = b"\x81\x80\x80\x80\x80\x80\x80\x80\x00".as_ref();
        assert_eq!(consume_varint(&mut b)?, 1);

        Ok(())
    }

    #[test]
    fn bytes_eof() {
        let mut b = b"".as_ref();
        assert!(matches!(consume_bytes(&mut b), Err(Error::Eof)));

        let mut b = b"\x01".as_ref();
        assert!(matches!(consume_bytes(&mut b), Err(Error::Eof)));

        let mut b = b"\x05hell".as_ref();
        assert!(matches!(consume_bytes(&mut b), Err(Error::Eof)));
    }

    #[test]
    fn bytes_empty() -> Result<(), Error> {
        let mut b = Vec::new();
        append_bytes(&mut b, b"")?;
        assert_eq!(b, b"\x00".as_ref());
        assert_eq!(consume_bytes(&mut b.as_slice())?, b"".to_vec());

        Ok(())
    }

    #[test]
    fn bytes_small() -> Result<(), Error> {
        let mut b = Vec::new();
        append_bytes(&mut b, b"hello".as_ref())?;
        assert_eq!(b, b"\x05hello".as_ref());
        assert_eq!(consume_bytes(&mut b.as_slice())?, b"hello".to_vec());

        Ok(())
    }

    #[test]
    fn bytes_large() -> Result<(), Error> {
        let v = Vec::from(b"hello".repeat(50));
        let mut b = Vec::new();
        append_bytes(&mut b, v.as_slice())?;
        assert_eq!(b, Vec::from([b"\xfa\x01".as_ref(), v.as_slice()].concat()));
        assert_eq!(consume_bytes(&mut b.as_slice())?, v);

        Ok(())
    }

    #[test]
    fn fixed32_eof() {
        assert!(matches!(
            consume_fixed32(&mut b"".as_ref()),
            Err(Error::Eof)
        ));
    }

    #[test]
    fn fixed32_min() -> Result<(), Error> {
        let mut b = Vec::new();
        let v = 0;
        append_fixed32(&mut b, v)?;
        assert_eq!(b, b"\x00\x00\x00\x00".as_ref());
        assert_eq!(consume_fixed32(&mut b.as_slice())?, v);

        Ok(())
    }

    #[test]
    fn fixed32_max() -> Result<(), Error> {
        let mut b = Vec::new();
        let v = 0xff_ff_ff_ff;
        append_fixed32(&mut b, v)?;
        assert_eq!(b, b"\xff\xff\xff\xff".as_ref());
        assert_eq!(consume_fixed32(&mut b.as_slice())?, v);

        Ok(())
    }

    #[test]
    fn fixed32() -> Result<(), Error> {
        let mut b = Vec::new();
        let v = 0xf0_e1_d2_c3;
        append_fixed32(&mut b, v)?;
        assert_eq!(b, b"\xc3\xd2\xe1\xf0".as_ref());
        assert_eq!(consume_fixed32(&mut b.as_slice())?, v);

        Ok(())
    }

    #[test]
    fn fixed64_eof() {
        assert!(matches!(
            consume_fixed32(&mut b"".as_ref()),
            Err(Error::Eof)
        ));
    }

    #[test]
    fn fixed64_min() -> Result<(), Error> {
        let mut b = Vec::new();
        let v = 0;
        append_fixed64(&mut b, v)?;
        assert_eq!(b, b"\x00\x00\x00\x00\x00\x00\x00\x00".as_ref());
        assert_eq!(consume_fixed64(&mut b.as_slice())?, v);

        Ok(())
    }

    #[test]
    fn fixed64_max() -> Result<(), Error> {
        let mut b = Vec::new();
        let v = 0xff_ff_ff_ff_ff_ff_ff_ff;
        append_fixed64(&mut b, v)?;
        assert_eq!(b, b"\xff\xff\xff\xff\xff\xff\xff\xff".as_ref());
        assert_eq!(consume_fixed64(&mut b.as_slice())?, v);

        Ok(())
    }

    #[test]
    fn fixed64() -> Result<(), Error> {
        let mut b = Vec::new();
        let v = 0xf0_e1_d2_c3_b4_a5_96_87;
        append_fixed64(&mut b, v)?;
        assert_eq!(b, b"\x87\x96\xa5\xb4\xc3\xd2\xe1\xf0".as_ref());
        assert_eq!(consume_fixed64(&mut b.as_slice())?, v);

        Ok(())
    }

    #[test]
    fn tag_eof() {
        assert!(matches!(consume_tag(&mut b"".as_ref()), Err(Error::Eof)));
    }

    #[test]
    fn tag_invalid_field_type() {
        // number = 1, type = 6
        assert!(matches!(
            consume_tag(&mut b"\x0e".as_ref()),
            Err(Error::InvalidWireType(6))
        ));
    }

    #[test]
    fn tag_invalid_field_number() {
        // number = 0, type = 0
        assert!(matches!(
            consume_tag(&mut b"\x00".as_ref()),
            Err(Error::InvalidFieldNumber(0))
        ));
    }

    #[test]
    fn tag_min() -> Result<(), Error> {
        let mut b = Vec::new();
        append_tag(&mut b, FieldNumber::try_from(1).unwrap(), WireType::Fixed32)?;
        assert_eq!(b, b"\x0d".as_ref());
        assert_eq!(
            consume_tag(&mut b.as_slice())?,
            (FieldNumber::try_from(1).unwrap(), WireType::Fixed32)
        );

        Ok(())
    }

    #[test]
    fn tag_max() -> Result<(), Error> {
        let mut b = Vec::new();
        let max = FieldNumber::try_from((1 << 29) - 1).unwrap();
        append_tag(&mut b, max, WireType::Fixed32)?;
        assert_eq!(b, b"\xfd\xff\xff\xff\x0f".as_ref());
        assert_eq!(consume_tag(&mut b.as_slice())?, (max, WireType::Fixed32));

        Ok(())
    }

    #[test]
    fn zig_zag() {
        let vals: Vec<(i64, u64)> = vec![
            (i64::MIN, u64::MAX),
            (i64::MIN + 1, u64::MAX - 2),
            (-1, 1),
            (0, 0),
            (1, 2),
            (i64::MAX - 1, u64::MAX - 3),
            (i64::MAX, u64::MAX - 1),
        ];

        for (dec, enc) in vals {
            assert_eq!(encode_zig_zag(dec), enc);
            assert_eq!(decode_zig_zag(enc), dec);
        }
    }
}
