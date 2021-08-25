use serde::Serialize;
use tobu::{
    encoding::field::FieldNumber,
    info::{Cardinality, FieldInfo, MessageInfo, Syntax, Type},
};

#[derive(Serialize)]
struct Inner {
    i1: String,
}

#[derive(Serialize)]
struct Outer {
    s1: i32,
    s2: Inner,
}

const OUTER_INFO: MessageInfo = MessageInfo {
    syntax: Syntax::Proto3,
    name: "Outer",
    fields: &[
        FieldInfo {
            name: "s1",
            json_name: "s1",
            type_name: "s1",
            number: unsafe { FieldNumber::new_unchecked(1) },
            cardinality: Cardinality::Optional,
            ty: Type::Int32,
            message_info: None,
            enum_info: None,
            oneof_index: None,
            packed: true,
        },
        FieldInfo {
            name: "s2",
            json_name: "s2",
            type_name: "s2",
            number: unsafe { FieldNumber::new_unchecked(2) },
            cardinality: Cardinality::Optional,
            ty: Type::Message,
            message_info: Some(&INNER_INFO),
            enum_info: None,
            oneof_index: None,
            packed: true,
        },
    ],
    is_map: false,
};

const INNER_INFO: MessageInfo = MessageInfo {
    syntax: Syntax::Proto3,
    name: "Inner",
    fields: &[FieldInfo {
        name: "i2",
        json_name: "i2",
        type_name: "i2",
        number: unsafe { FieldNumber::new_unchecked(1) },
        cardinality: Cardinality::Optional,
        ty: Type::String,
        message_info: None,
        enum_info: None,
        oneof_index: None,
        packed: true,
    }],
    is_map: false,
};

#[test]
fn main() {
    let s = Outer {
        s1: 1,
        s2: Inner {
            i1: "hello".to_string(),
        },
    };
    let size = tobu::serialized_size(&s, &OUTER_INFO).unwrap();
    assert_eq!(size, 10);
}
