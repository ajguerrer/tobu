use crate::encoding::{field::FieldNumber, wire::WireType};

#[derive(Debug, Default, Clone, Copy)]
pub struct MessageInfo {
    pub name: &'static str,
    pub fields: &'static [FieldInfo],
    pub syntax: Syntax,
    pub is_map: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Default for Syntax {
    fn default() -> Self {
        Syntax::Proto2
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct FieldInfo {
    pub name: &'static str,
    pub number: FieldNumber,
    pub cardinality: Cardinality,
    pub ty: Type,
    pub type_name: &'static str,
    pub json_name: &'static str,
    pub packed: bool,
    pub oneof_index: Option<i32>,
    pub message_info: Option<&'static MessageInfo>,
    pub enum_info: Option<&'static EnumInfo>,
}

#[derive(Debug, Clone, Copy)]
pub enum Cardinality {
    Optional,
    Required,
    Repeated,
}

impl Default for Cardinality {
    fn default() -> Self {
        Cardinality::Optional
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Double,
    Float,
    Int64,
    Uint64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String,
    Group,
    Message,
    Bytes,
    Uint32,
    Enum,
    SFixed32,
    SFixed64,
    SInt32,
    SInt64,
}

impl Type {
    pub(crate) fn wire_type(&self) -> WireType {
        match self {
            Type::Double => WireType::Fixed64,
            Type::Float => WireType::Fixed32,
            Type::Int64 => WireType::Varint,
            Type::Uint64 => WireType::Varint,
            Type::Int32 => WireType::Varint,
            Type::Fixed64 => WireType::Fixed64,
            Type::Fixed32 => WireType::Fixed32,
            Type::Bool => WireType::Varint,
            Type::String => WireType::Bytes,
            Type::Group => WireType::StartGroup,
            Type::Message => WireType::Bytes,
            Type::Bytes => WireType::Bytes,
            Type::Uint32 => WireType::Varint,
            Type::Enum => WireType::Varint,
            Type::SFixed32 => WireType::Fixed32,
            Type::SFixed64 => WireType::Fixed64,
            Type::SInt32 => WireType::Varint,
            Type::SInt64 => WireType::Varint,
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Double
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnumInfo {
    pub name: &'static str,
    pub value: &'static [EnumValue],
}

#[derive(Debug, Clone, Copy)]
pub struct EnumValue {
    pub name: &'static str,
    pub number: i32,
}
