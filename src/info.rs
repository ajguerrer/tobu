pub struct FileDescriptor {
    pub name: &'static str,
    pub package: &'static str,
    pub message_type: &'static [&'static MessageDescriptor],
    pub enum_type: &'static [&'static EnumDescriptor],
    pub syntax: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct MessageDescriptor {
    pub name: &'static str,
    pub fields: &'static [FieldDescriptor],
    pub nested_type: &'static [&'static MessageDescriptor],
    pub enum_type: &'static [EnumDescriptor],
    pub oneof_decl: &'static [OneofDescriptor],
    pub options: MessageOptions,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldDescriptor {
    pub name: &'static str,
    pub number: i32,
    pub label: FieldDescriptorLabel,
    pub type_: FieldDescriptorType,
    pub type_name: &'static str,
    pub json_name: &'static str,
    pub default_value: &'static str,
    pub oneof_index: Option<i32>,
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum FieldDescriptorLabel {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum FieldDescriptorType {
    Double = 1,
    Float = 2,
    Int64 = 3,
    Uint64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    Group = 10,
    Message = 11,
    Bytes = 12,
    Uint32 = 13,
    Enum = 14,
    SFixed32 = 15,
    SFixed64 = 16,
    SInt32 = 17,
    SInt64 = 18,
}

#[derive(Debug, Clone, Copy)]
pub struct EnumDescriptor {
    pub name: &'static str,
    pub value: &'static [EnumValueDescriptor],
}

#[derive(Debug, Clone, Copy)]
pub struct EnumValueDescriptor {
    pub name: &'static str,
    pub number: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct OneofDescriptor {
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct MessageOptions {
    pub map_entry: bool,
}
