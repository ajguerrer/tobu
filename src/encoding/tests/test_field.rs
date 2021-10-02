use std::convert::TryFrom;

use crate::encoding::field::FieldNumber;

#[test]
fn field_number() {
    assert_eq!(FieldNumber::default().get(), 1);
    assert!(matches!(FieldNumber::try_from(-1), Err(_)));
    assert!(matches!(FieldNumber::try_from(0), Err(_)));
    assert!(matches!(FieldNumber::try_from(1), Ok(_)));
    assert!(matches!(FieldNumber::try_from(18999), Ok(_)));
    assert!(matches!(FieldNumber::try_from(19000), Err(_)));
    assert!(matches!(FieldNumber::try_from(19999), Err(_)));
    assert!(matches!(FieldNumber::try_from(20000), Ok(_)));
    assert!(matches!(FieldNumber::try_from((1 << 29) - 1), Ok(_)));
    assert!(matches!(FieldNumber::try_from(1 << 29), Err(_)));
}
