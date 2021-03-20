#[derive(Debug, Clone, Copy)]
pub struct FileDescriptor {
    name: &'static str,
    package: &'static str,
    message_type: &'static [&'static MessageDescriptor],
    enum_type: &'static [&'static EnumDescriptor],
    syntax: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct MessageDescriptor {
    name: &'static str,
    fields: &'static [FieldDescriptor],
    nested_type: &'static [&'static MessageDescriptor],
    enum_type: &'static [EnumDescriptor],
    oneof_decl: &'static [OneofDescriptor],
    options: MessageOptions,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldDescriptor {
    name: &'static str,
    number: i32,
    label: FieldDescriptorLabel,
    type_: FieldDescriptorType,
    type_name: &'static str,
    json_name: &'static str,
    default_value: &'static str,
    oneof_index: Option<i32>,
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
    name: &'static str,
    value: &'static [EnumValueDescriptor],
}

#[derive(Debug, Clone, Copy)]
pub struct EnumValueDescriptor {
    name: &'static str,
    number: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct OneofDescriptor {
    name: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct MessageOptions {
    map_entry: bool,
}
