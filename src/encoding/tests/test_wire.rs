use std::convert::TryFrom;

use bytes::{BufMut, Bytes, BytesMut};

use crate::encoding::{error::Error, field::FieldNumber, wire::*};

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

    let mut parser = Parser::new(buf.freeze());
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num1, FieldValue::Varint(val1))
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num2, FieldValue::Fixed32(val2))
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num3, FieldValue::Fixed64(val3))
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num4, FieldValue::Bytes(val4))
    );
}

#[test]
fn group() {
    let mut buf = BytesMut::new();

    let group_num = FieldNumber::try_from(5000).unwrap();
    put_tag(&mut buf, group_num, WireType::StartGroup);

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Varint);
    let val = 0x123456789;
    put_varint(&mut buf, val);

    put_tag(&mut buf, group_num, WireType::EndGroup);

    let mut parser = Parser::new(buf.freeze());
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(group_num, FieldValue::StartGroup)
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num, FieldValue::Varint(val))
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(group_num, FieldValue::EndGroup)
    );
}

#[test]
fn group_nested() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::StartGroup);

    let nested_num = FieldNumber::try_from(2).unwrap();
    put_tag(&mut buf, nested_num, WireType::StartGroup);

    put_tag(&mut buf, nested_num, WireType::EndGroup);

    let mut parser = Parser::new(buf.freeze());
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num, FieldValue::StartGroup)
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(nested_num, FieldValue::StartGroup)
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(nested_num, FieldValue::EndGroup)
    );
}

#[test]
fn group_empty() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::StartGroup);

    put_tag(&mut buf, num, WireType::EndGroup);

    let mut parser = Parser::new(buf.freeze());
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num, FieldValue::StartGroup)
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num, FieldValue::EndGroup)
    );
}

#[test]
fn group_denormalized() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(5).unwrap();
    put_tag(&mut buf, num, WireType::Fixed32);

    let val = 0xf0e1d2c3;
    put_fixed32(&mut buf, val);

    // manually end group
    buf.put_slice(b"\xac\x80\x80\x00");

    let mut parser = Parser::new(buf.freeze());
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num, FieldValue::Fixed32(val))
    );
    assert_eq!(
        parser.next().unwrap().unwrap(),
        WireField(num, FieldValue::EndGroup)
    );
}

#[test]
fn varint_eof() {
    let values = vec![
        Bytes::from_static(b"\x80"),
        Bytes::from_static(b"\x80\x80"),
        Bytes::from_static(b"\x80\x80\x80"),
        Bytes::from_static(b"\x80\x80\x80\x80"),
        Bytes::from_static(b"\x80\x80\x80\x80\x80"),
        Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80"),
        Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80"),
        Bytes::from_static(b"\x80\x80\x80\x80\x80\x80\x80\x80\x80"),
    ];

    for val in values {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::Varint);

        buf.put_slice(&val);

        assert!(matches!(
            Parser::new(buf.freeze()).next().unwrap(),
            Err(Error::Eof)
        ));
    }
}

#[test]
fn varint_overflow_significant_bits() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Varint);

    // Too many bytes with significant bits
    buf.put_slice(&Bytes::from_static(
        b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x80",
    ));

    assert!(matches!(
        Parser::new(buf.freeze()).next().unwrap(),
        Err(Error::Overflow)
    ));
}

#[test]
fn varint_overflow_u64_max_plus_one() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Varint);

    // Exceeds u64::MAX
    buf.put_slice(&Bytes::from_static(
        b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x02",
    ));

    assert!(matches!(
        Parser::new(buf.freeze()).next().unwrap(),
        Err(Error::Overflow)
    ));
}

#[test]
fn varint_boundaries() {
    let values = vec![
        0x00,
        0x01,
        0x7f,
        0x80,
        0x3f_ff,
        0x40_00,
        0x1f_ff_ff,
        0x20_00_00,
        0x0f_ff_ff_ff,
        0x10_00_00_00,
        0x07_ff_ff_ff_ff,
        0x08_00_00_00_00,
        0x03_ff_ff_ff_ff_ff,
        0x04_00_00_00_00_00,
        0x01_ff_ff_ff_ff_ff_ff,
        0x02_00_00_00_00_00_00,
        0xff_ff_ff_ff_ff_ff_ff,
        0x01_00_00_00_00_00_00_00,
        0x7f_ff_ff_ff_ff_ff_ff_ff,
        0x80_00_00_00_00_00_00_00,
    ];

    for val in values {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::Varint);

        put_varint(&mut buf, val);

        assert_eq!(
            Parser::new(buf.freeze()).next().unwrap().unwrap(),
            WireField(num, FieldValue::Varint(val))
        );
    }
}

#[test]
fn varint_max() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Varint);

    put_varint(&mut buf, u64::MAX);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Varint(u64::MAX))
    );
}

#[test]
fn varint_denormalized() {
    let values = vec![
        Bytes::from_static(b"\x01"),
        Bytes::from_static(b"\x81\x00"),
        Bytes::from_static(b"\x81\x80\x00"),
        Bytes::from_static(b"\x81\x80\x80\x00"),
        Bytes::from_static(b"\x81\x80\x80\x80\x00"),
        Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x00"),
        Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x80\x00"),
        Bytes::from_static(b"\x81\x80\x80\x80\x80\x80\x80\x80\x00"),
    ];

    for val in values {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::Varint);

        buf.put_slice(&val);

        assert_eq!(
            Parser::new(buf.freeze()).next().unwrap().unwrap(),
            WireField(num, FieldValue::Varint(1))
        );
    }
}

#[test]
fn bytes_eof() {
    let values = vec![
        Bytes::from_static(b""),
        Bytes::from_static(b"\x01"),
        Bytes::from_static(b"\x05hell"),
    ];

    for val in values {
        let mut buf = BytesMut::new();

        let num = FieldNumber::try_from(1).unwrap();
        put_tag(&mut buf, num, WireType::Bytes);

        buf.put_slice(&val);

        assert!(matches!(
            Parser::new(buf.freeze()).next().unwrap(),
            Err(Error::Eof)
        ));
    }
}

#[test]
fn bytes_empty() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Bytes);

    put_bytes(&mut buf, b"");

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Bytes(Bytes::from_static(b"")))
    );
}

#[test]
fn bytes_small() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Bytes);

    let val = Bytes::from_static(b"hello");
    put_bytes(&mut buf, &val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Bytes(val))
    );
}

#[test]
fn bytes_large() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Bytes);

    let val = Bytes::from(b"hello".repeat(50));
    put_bytes(&mut buf, &val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Bytes(val))
    );
}

#[test]
fn fixed32_eof() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed32);

    buf.put_slice(b"\x01\x02\x03");

    assert!(matches!(
        Parser::new(buf.freeze()).next().unwrap(),
        Err(Error::Eof)
    ));
}

#[test]
fn fixed32_min() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed32);

    let val = 0;
    put_fixed32(&mut buf, val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Fixed32(val))
    );
}

#[test]
fn fixed32_max() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed32);

    let val = u32::MAX;
    put_fixed32(&mut buf, val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Fixed32(val))
    );
}

#[test]
fn fixed32() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed32);

    let val = 0xf0_e1_d2_c3;
    put_fixed32(&mut buf, val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Fixed32(val))
    );
}

#[test]
fn fixed64_eof() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed64);

    buf.put_slice(b"\x01\x02\x03\x04\x05");

    assert!(matches!(
        Parser::new(buf.freeze()).next().unwrap(),
        Err(Error::Eof)
    ));
}

#[test]
fn fixed64_min() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed64);

    let val = 0;
    put_fixed64(&mut buf, val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Fixed64(val))
    );
}

#[test]
fn fixed64_max() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed64);

    let val = 0xff_ff_ff_ff_ff_ff_ff_ff;
    put_fixed64(&mut buf, val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Fixed64(val))
    );
}

#[test]
fn fixed64() {
    let mut buf = BytesMut::new();

    let num = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, num, WireType::Fixed64);

    let val = 0xf0_e1_d2_c3_b4_a5_96_87;
    put_fixed64(&mut buf, val);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(num, FieldValue::Fixed64(val))
    );
}

#[test]
fn tag_eof() {
    assert!(matches!(
        Parser::new(Bytes::from_static(b"\x80")).next().unwrap(),
        Err(Error::Eof)
    ));
}

#[test]
fn tag_invalid_field_type() {
    // num = 1, typ = 6
    assert!(matches!(
        Parser::new(Bytes::from_static(b"\x0e")).next().unwrap(),
        Err(Error::InvalidWireType(6))
    ));
}

#[test]
fn tag_invalid_field_number() {
    let values = vec![0, 19000, 19999, (1 << 29)];
    // num = 0, typ = 0

    for val in values {
        let mut buf = BytesMut::new();

        let invalid_num = unsafe { FieldNumber::new_unchecked(val) };
        put_tag(&mut buf, invalid_num, WireType::Varint);

        assert!(matches!(
            Parser::new(buf.freeze()).next().unwrap(),
            Err(Error::InvalidFieldNumber(_))
        ));
    }
}

#[test]
fn tag_min() {
    let mut buf = BytesMut::new();

    let min = FieldNumber::try_from(1).unwrap();
    put_tag(&mut buf, min, WireType::Fixed32);

    let val = 1;
    put_fixed32(&mut buf, 1);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(min, FieldValue::Fixed32(val))
    );
}

#[test]
fn tag_max() {
    let mut buf = BytesMut::new();

    let max = FieldNumber::try_from((1 << 29) - 1).unwrap();
    put_tag(&mut buf, max, WireType::Fixed32);

    let val = 1;
    put_fixed32(&mut buf, 1);

    assert_eq!(
        Parser::new(buf.freeze()).next().unwrap().unwrap(),
        WireField(max, FieldValue::Fixed32(val))
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
