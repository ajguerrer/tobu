use bytes::Bytes;
use serde::Deserialize;
use serde_derive::Deserialize;
use tobu::de::Deserializer;

pub struct BasicScalarTypes {
    pub optional_int32: Option<i32>,
    pub optional_int64: Option<i64>,
    pub optional_uint32: Option<u32>,
    pub optional_uint64: Option<u64>,
    pub optional_sint32: Option<i32>,
    pub optional_sint64: Option<i64>,
    pub optional_fixed32: Option<u32>,
    pub optional_fixed64: Option<u64>,
    pub optional_sfixed32: Option<i32>,
    pub optional_sfixed64: Option<i64>,
    pub optional_float: Option<f32>,
    pub optional_double: Option<f64>,
    pub optional_bool: Option<bool>,
    pub optional_string: Option<String>,
    pub optional_bytes: Option<Vec<u8>>,
    pub optional_nested_enum: Option<BasicScalarTypesNestedEnum>,
}

#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[allow(unused_macros)]
    macro_rules! my_try {
($__expr:expr) => {
  match$__expr {
    _serde::__private::Ok(__val) => __val, _serde::__private::Err(__err) => {
      return _serde::__private::Err(__err);

    }
  }
}
}
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for BasicScalarTypes {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
                __field6,
                __field7,
                __field8,
                __field9,
                __field10,
                __field11,
                __field12,
                __field13,
                __field14,
                __field15,
                __ignore,
            }
            struct __FieldVisitor;

            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        4u64 => _serde::__private::Ok(__Field::__field4),
                        5u64 => _serde::__private::Ok(__Field::__field5),
                        6u64 => _serde::__private::Ok(__Field::__field6),
                        7u64 => _serde::__private::Ok(__Field::__field7),
                        8u64 => _serde::__private::Ok(__Field::__field8),
                        9u64 => _serde::__private::Ok(__Field::__field9),
                        10u64 => _serde::__private::Ok(__Field::__field10),
                        11u64 => _serde::__private::Ok(__Field::__field11),
                        12u64 => _serde::__private::Ok(__Field::__field12),
                        13u64 => _serde::__private::Ok(__Field::__field13),
                        14u64 => _serde::__private::Ok(__Field::__field14),
                        15u64 => _serde::__private::Ok(__Field::__field15),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "optional_int32" => _serde::__private::Ok(__Field::__field0),
                        "optional_int64" => _serde::__private::Ok(__Field::__field1),
                        "optional_uint32" => _serde::__private::Ok(__Field::__field2),
                        "optional_uint64" => _serde::__private::Ok(__Field::__field3),
                        "optional_sint32" => _serde::__private::Ok(__Field::__field4),
                        "optional_sint64" => _serde::__private::Ok(__Field::__field5),
                        "optional_fixed32" => _serde::__private::Ok(__Field::__field6),
                        "optional_fixed64" => _serde::__private::Ok(__Field::__field7),
                        "optional_sfixed32" => _serde::__private::Ok(__Field::__field8),
                        "optional_sfixed64" => _serde::__private::Ok(__Field::__field9),
                        "optional_float" => _serde::__private::Ok(__Field::__field10),
                        "optional_double" => _serde::__private::Ok(__Field::__field11),
                        "optional_bool" => _serde::__private::Ok(__Field::__field12),
                        "optional_string" => _serde::__private::Ok(__Field::__field13),
                        "optional_bytes" => _serde::__private::Ok(__Field::__field14),
                        "optional_nested_enum" => _serde::__private::Ok(__Field::__field15),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"optional_int32" => _serde::__private::Ok(__Field::__field0),
                        b"optional_int64" => _serde::__private::Ok(__Field::__field1),
                        b"optional_uint32" => _serde::__private::Ok(__Field::__field2),
                        b"optional_uint64" => _serde::__private::Ok(__Field::__field3),
                        b"optional_sint32" => _serde::__private::Ok(__Field::__field4),
                        b"optional_sint64" => _serde::__private::Ok(__Field::__field5),
                        b"optional_fixed32" => _serde::__private::Ok(__Field::__field6),
                        b"optional_fixed64" => _serde::__private::Ok(__Field::__field7),
                        b"optional_sfixed32" => _serde::__private::Ok(__Field::__field8),
                        b"optional_sfixed64" => _serde::__private::Ok(__Field::__field9),
                        b"optional_float" => _serde::__private::Ok(__Field::__field10),
                        b"optional_double" => _serde::__private::Ok(__Field::__field11),
                        b"optional_bool" => _serde::__private::Ok(__Field::__field12),
                        b"optional_string" => _serde::__private::Ok(__Field::__field13),
                        b"optional_bytes" => _serde::__private::Ok(__Field::__field14),
                        b"optional_nested_enum" => _serde::__private::Ok(__Field::__field15),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<BasicScalarTypes>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = BasicScalarTypes;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct BasicScalarTypes")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<i32>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field1 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<i64>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field2 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<u32>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field3 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<u64>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field4 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<i32>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field5 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<i64>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                5usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field6 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<u32>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                6usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field7 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<u64>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                7usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field8 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<i32>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                8usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field9 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<i64>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                9usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field10 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<f32>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                10usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field11 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<f64>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                11usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field12 = match my_try!(
                        _serde::de::SeqAccess::next_element::<Option<bool>>(&mut __seq)
                    ) {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                12usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field13 = match my_try!(_serde::de::SeqAccess::next_element::<
                        Option<String>,
                    >(&mut __seq))
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                13usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field14 = match my_try!(_serde::de::SeqAccess::next_element::<
                        Option<Vec<u8>>,
                    >(&mut __seq))
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                14usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    let __field15 = match my_try!(_serde::de::SeqAccess::next_element::<
                        Option<BasicScalarTypesNestedEnum>,
                    >(&mut __seq))
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                15usize,
                                &"struct BasicScalarTypes with 16 elements",
                            ));
                        }
                    };
                    _serde::__private::Ok(BasicScalarTypes {
                        optional_int32: __field0,
                        optional_int64: __field1,
                        optional_uint32: __field2,
                        optional_uint64: __field3,
                        optional_sint32: __field4,
                        optional_sint64: __field5,
                        optional_fixed32: __field6,
                        optional_fixed64: __field7,
                        optional_sfixed32: __field8,
                        optional_sfixed64: __field9,
                        optional_float: __field10,
                        optional_double: __field11,
                        optional_bool: __field12,
                        optional_string: __field13,
                        optional_bytes: __field14,
                        optional_nested_enum: __field15,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<Option<i32>> =
                        _serde::__private::None;
                    let mut __field1: _serde::__private::Option<Option<i64>> =
                        _serde::__private::None;
                    let mut __field2: _serde::__private::Option<Option<u32>> =
                        _serde::__private::None;
                    let mut __field3: _serde::__private::Option<Option<u64>> =
                        _serde::__private::None;
                    let mut __field4: _serde::__private::Option<Option<i32>> =
                        _serde::__private::None;
                    let mut __field5: _serde::__private::Option<Option<i64>> =
                        _serde::__private::None;
                    let mut __field6: _serde::__private::Option<Option<u32>> =
                        _serde::__private::None;
                    let mut __field7: _serde::__private::Option<Option<u64>> =
                        _serde::__private::None;
                    let mut __field8: _serde::__private::Option<Option<i32>> =
                        _serde::__private::None;
                    let mut __field9: _serde::__private::Option<Option<i64>> =
                        _serde::__private::None;
                    let mut __field10: _serde::__private::Option<Option<f32>> =
                        _serde::__private::None;
                    let mut __field11: _serde::__private::Option<Option<f64>> =
                        _serde::__private::None;
                    let mut __field12: _serde::__private::Option<Option<bool>> =
                        _serde::__private::None;
                    let mut __field13: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    let mut __field14: _serde::__private::Option<Option<Vec<u8>>> =
                        _serde::__private::None;
                    let mut __field15: _serde::__private::Option<
                        Option<BasicScalarTypesNestedEnum>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        my_try!(_serde::de::MapAccess::next_key::<__Field>(&mut __map))
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_int32",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<i32>>(&mut __map)
                                ));
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_int64",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<i64>>(&mut __map)
                                ));
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_uint32",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<u32>>(&mut __map)
                                ));
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_uint64",
                                        ),
                                    );
                                }
                                __field3 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<u64>>(&mut __map)
                                ));
                            }
                            __Field::__field4 => {
                                if _serde::__private::Option::is_some(&__field4) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_sint32",
                                        ),
                                    );
                                }
                                __field4 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<i32>>(&mut __map)
                                ));
                            }
                            __Field::__field5 => {
                                if _serde::__private::Option::is_some(&__field5) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_sint64",
                                        ),
                                    );
                                }
                                __field5 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<i64>>(&mut __map)
                                ));
                            }
                            __Field::__field6 => {
                                if _serde::__private::Option::is_some(&__field6) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_fixed32",
                                        ),
                                    );
                                }
                                __field6 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<u32>>(&mut __map)
                                ));
                            }
                            __Field::__field7 => {
                                if _serde::__private::Option::is_some(&__field7) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_fixed64",
                                        ),
                                    );
                                }
                                __field7 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<u64>>(&mut __map)
                                ));
                            }
                            __Field::__field8 => {
                                if _serde::__private::Option::is_some(&__field8) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_sfixed32",
                                        ),
                                    );
                                }
                                __field8 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<i32>>(&mut __map)
                                ));
                            }
                            __Field::__field9 => {
                                if _serde::__private::Option::is_some(&__field9) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_sfixed64",
                                        ),
                                    );
                                }
                                __field9 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<i64>>(&mut __map)
                                ));
                            }
                            __Field::__field10 => {
                                if _serde::__private::Option::is_some(&__field10) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_float",
                                        ),
                                    );
                                }
                                __field10 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<f32>>(&mut __map)
                                ));
                            }
                            __Field::__field11 => {
                                if _serde::__private::Option::is_some(&__field11) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_double",
                                        ),
                                    );
                                }
                                __field11 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<f64>>(&mut __map)
                                ));
                            }
                            __Field::__field12 => {
                                if _serde::__private::Option::is_some(&__field12) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_bool",
                                        ),
                                    );
                                }
                                __field12 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<bool>>(&mut __map)
                                ));
                            }
                            __Field::__field13 => {
                                if _serde::__private::Option::is_some(&__field13) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_string",
                                        ),
                                    );
                                }
                                __field13 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<String>>(&mut __map)
                                ));
                            }
                            __Field::__field14 => {
                                if _serde::__private::Option::is_some(&__field14) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_bytes",
                                        ),
                                    );
                                }
                                __field14 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<Option<Vec<u8>>>(
                                        &mut __map
                                    )
                                ));
                            }
                            __Field::__field15 => {
                                if _serde::__private::Option::is_some(&__field15) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_nested_enum",
                                        ),
                                    );
                                }
                                __field15 = _serde::__private::Some(my_try!(
                                    _serde::de::MapAccess::next_value::<
                                        Option<BasicScalarTypesNestedEnum>,
                                    >(&mut __map)
                                ));
                            }
                            _ => {
                                let _ = my_try!(_serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map));
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_int32"))
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_int64"))
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_uint32"))
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_uint64"))
                        }
                    };
                    let __field4 = match __field4 {
                        _serde::__private::Some(__field4) => __field4,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_sint32"))
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::__private::Some(__field5) => __field5,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_sint64"))
                        }
                    };
                    let __field6 = match __field6 {
                        _serde::__private::Some(__field6) => __field6,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_fixed32"))
                        }
                    };
                    let __field7 = match __field7 {
                        _serde::__private::Some(__field7) => __field7,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_fixed64"))
                        }
                    };
                    let __field8 = match __field8 {
                        _serde::__private::Some(__field8) => __field8,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_sfixed32"))
                        }
                    };
                    let __field9 = match __field9 {
                        _serde::__private::Some(__field9) => __field9,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_sfixed64"))
                        }
                    };
                    let __field10 = match __field10 {
                        _serde::__private::Some(__field10) => __field10,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_float"))
                        }
                    };
                    let __field11 = match __field11 {
                        _serde::__private::Some(__field11) => __field11,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_double"))
                        }
                    };
                    let __field12 = match __field12 {
                        _serde::__private::Some(__field12) => __field12,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_bool"))
                        }
                    };
                    let __field13 = match __field13 {
                        _serde::__private::Some(__field13) => __field13,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_string"))
                        }
                    };
                    let __field14 = match __field14 {
                        _serde::__private::Some(__field14) => __field14,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_bytes"))
                        }
                    };
                    let __field15 = match __field15 {
                        _serde::__private::Some(__field15) => __field15,
                        _serde::__private::None => {
                            my_try!(_serde::__private::de::missing_field("optional_nested_enum"))
                        }
                    };
                    _serde::__private::Ok(BasicScalarTypes {
                        optional_int32: __field0,
                        optional_int64: __field1,
                        optional_uint32: __field2,
                        optional_uint64: __field3,
                        optional_sint32: __field4,
                        optional_sint64: __field5,
                        optional_fixed32: __field6,
                        optional_fixed64: __field7,
                        optional_sfixed32: __field8,
                        optional_sfixed64: __field9,
                        optional_float: __field10,
                        optional_double: __field11,
                        optional_bool: __field12,
                        optional_string: __field13,
                        optional_bytes: __field14,
                        optional_nested_enum: __field15,
                    })
                }
            }
            const FIELDS: &[&str] = &[
                "optional_int32",
                "optional_int64",
                "optional_uint32",
                "optional_uint64",
                "optional_sint32",
                "optional_sint64",
                "optional_fixed32",
                "optional_fixed64",
                "optional_sfixed32",
                "optional_sfixed64",
                "optional_float",
                "optional_double",
                "optional_bool",
                "optional_string",
                "optional_bytes",
                "optional_nested_enum",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "BasicScalarTypes",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<BasicScalarTypes>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};

#[derive(Deserialize)]
pub enum BasicScalarTypesNestedEnum {
    Foo = 0,
    Bar = 1,
    Baz = 2,
    Neg = -1, // Intentionally negative.
}

fn main() {
    let buf = b"\x08\xe9\x07\x10\xea\x07\x18\xeb\x07\x20\xec\x07\x28\xda\x0f\x30\xdc\x0f\x3d\xef\x03\x00\x00\x41\xf0\x03\x00\x00\x00\x00\x00\x00\x4d\xf1\x03\x00\x00\x51\xf2\x03\x00\x00\x00\x00\x00\x00\x5d\x00\xe0\x7c\x44\x61\x00\x00\x00\x00\x00\xa4\x8f\x40\x68\x01\x72\x06\x73\x74\x72\x69\x6e\x67\x7a\x05\x62\x79\x74\x65\x73\xa8\x01\x01";
    let mut d = Deserializer::new(Bytes::from_static(buf)).unwrap();
    let _bst = BasicScalarTypes::deserialize(&mut d).unwrap();
}
