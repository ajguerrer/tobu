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
    pub number: i32,
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
