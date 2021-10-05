#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileDescriptorSet {
    pub file: Vec<FileDescriptorProto>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileDescriptorProto {
    pub name: Option<String>,
    pub package: Option<String>,
    pub dependency: Vec<String>,
    pub public_dependency: Vec<i32>,
    pub weak_dependency: Vec<i32>,
    pub message_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub service: Vec<ServiceDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub options: Option<FileOptions>,
    pub source_code_info: Option<SourceCodeInfo>,
    pub syntax: Option<String>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescriptorProto {
    pub name: Option<String>,
    pub field: Vec<FieldDescriptorProto>,
    pub extension: Vec<FieldDescriptorProto>,
    pub nested_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub extension_range: Vec<DescriptorProtoExtensionRange>,
    pub oneof_decl: Vec<OneofDescriptorProto>,
    pub options: Option<MessageOptions>,
    pub reserved_range: Vec<DescriptorProtoReservedRange>,
    pub reserved_name: Vec<String>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescriptorProtoExtensionRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
    pub options: Option<ExtensionRangeOptions>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescriptorProtoReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ExtensionRangeOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub label: Option<FieldDescriptorProtoLabel>,
    pub r#type: Option<FieldDescriptorProtoType>,
    pub type_name: Option<String>,
    pub extendee: Option<String>,
    pub default_value: Option<String>,
    pub oneof_index: Option<i32>,
    pub json_name: Option<String>,
    pub options: Option<FieldOptions>,
    pub proto3_optional: Option<bool>,
}
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FieldDescriptorProtoType {
    Double = 1i32,
    Float = 2i32,
    Int64 = 3i32,
    Uint64 = 4i32,
    Int32 = 5i32,
    Fixed64 = 6i32,
    Fixed32 = 7i32,
    Bool = 8i32,
    String = 9i32,
    Group = 10i32,
    Message = 11i32,
    Bytes = 12i32,
    Uint32 = 13i32,
    Enum = 14i32,
    Sfixed32 = 15i32,
    Sfixed64 = 16i32,
    Sint32 = 17i32,
    Sint64 = 18i32,
}
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FieldDescriptorProtoLabel {
    Optional = 1i32,
    Required = 2i32,
    Repeated = 3i32,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneofDescriptorProto {
    pub name: Option<String>,
    pub options: Option<OneofOptions>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumDescriptorProto {
    pub name: Option<String>,
    pub value: Vec<EnumValueDescriptorProto>,
    pub options: Option<EnumOptions>,
    pub reserved_range: Vec<EnumDescriptorProtoEnumReservedRange>,
    pub reserved_name: Vec<String>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumDescriptorProtoEnumReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumValueDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub options: Option<EnumValueOptions>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ServiceDescriptorProto {
    pub name: Option<String>,
    pub method: Vec<MethodDescriptorProto>,
    pub options: Option<ServiceOptions>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MethodDescriptorProto {
    pub name: Option<String>,
    pub input_type: Option<String>,
    pub output_type: Option<String>,
    pub options: Option<MethodOptions>,
    pub client_streaming: Option<bool>,
    pub server_streaming: Option<bool>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileOptions {
    pub java_package: Option<String>,
    pub java_outer_classname: Option<String>,
    pub java_multiple_files: Option<bool>,
    pub java_generate_equals_and_hash: Option<bool>,
    pub java_string_check_utf8: Option<bool>,
    pub optimize_for: Option<FileOptionsOptimizeMode>,
    pub go_package: Option<String>,
    pub cc_generic_services: Option<bool>,
    pub java_generic_services: Option<bool>,
    pub py_generic_services: Option<bool>,
    pub php_generic_services: Option<bool>,
    pub deprecated: Option<bool>,
    pub cc_enable_arenas: Option<bool>,
    pub objc_class_prefix: Option<String>,
    pub csharp_namespace: Option<String>,
    pub swift_prefix: Option<String>,
    pub php_class_prefix: Option<String>,
    pub php_namespace: Option<String>,
    pub php_metadata_namespace: Option<String>,
    pub ruby_package: Option<String>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FileOptionsOptimizeMode {
    Speed = 1i32,
    CodeSize = 2i32,
    LiteRuntime = 3i32,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MessageOptions {
    pub message_set_wire_format: Option<bool>,
    pub no_standard_descriptor_accessor: Option<bool>,
    pub deprecated: Option<bool>,
    pub map_entry: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldOptions {
    pub ctype: Option<FieldOptionsCType>,
    pub packed: Option<bool>,
    pub jstype: Option<FieldOptionsJSType>,
    pub lazy: Option<bool>,
    pub deprecated: Option<bool>,
    pub weak: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FieldOptionsCType {
    String = 0i32,
    Cord = 1i32,
    StringPiece = 2i32,
}
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FieldOptionsJSType {
    JsNormal = 0i32,
    JsString = 1i32,
    JsNumber = 2i32,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OneofOptions {
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumOptions {
    pub allow_alias: Option<bool>,
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumValueOptions {
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ServiceOptions {
    pub deprecated: Option<bool>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MethodOptions {
    pub deprecated: Option<bool>,
    pub idempotency_level: Option<MethodOptionsIdempotencyLevel>,
    pub uninterpreted_option: Vec<UninterpretedOption>,
}
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum MethodOptionsIdempotencyLevel {
    IdempotencyUnknown = 0i32,
    NoSideEffects = 1i32,
    Idempotent = 2i32,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UninterpretedOption {
    pub name: Vec<UninterpretedOptionNamePart>,
    pub identifier_value: Option<String>,
    pub positive_int_value: Option<u64>,
    pub negative_int_value: Option<i64>,
    pub double_value: Option<f64>,
    pub string_value: Vec<u8>,
    pub aggregate_value: Option<String>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct UninterpretedOptionNamePart {
    pub name_part: String,
    pub is_extension: bool,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SourceCodeInfo {
    pub location: Vec<SourceCodeInfoLocation>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SourceCodeInfoLocation {
    pub path: Vec<i32>,
    pub span: Vec<i32>,
    pub leading_comments: Option<String>,
    pub trailing_comments: Option<String>,
    pub leading_detached_comments: Vec<String>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeneratedCodeInfo {
    pub annotation: Vec<GeneratedCodeInfoAnnotation>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeneratedCodeInfoAnnotation {
    pub path: Vec<i32>,
    pub source_file: Option<String>,
    pub begin: Option<i32>,
    pub end: Option<i32>,
}
