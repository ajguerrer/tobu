use crate::encoding::field::FieldNumber;

#[test]
fn field_number() {
    assert_eq!(FieldNumber::default().get(), 1);
    assert!(matches!(FieldNumber::new(-1), None));
    assert!(matches!(FieldNumber::new(0), None));
    assert!(matches!(FieldNumber::new(1), Some(_)));
    assert!(matches!(FieldNumber::new(18999), Some(_)));
    assert!(matches!(FieldNumber::new(19000), None));
    assert!(matches!(FieldNumber::new(19999), None));
    assert!(matches!(FieldNumber::new(20000), Some(_)));
    assert!(matches!(FieldNumber::new((1 << 29) - 1), Some(_)));
    assert!(matches!(FieldNumber::new(1 << 29), None));
}
