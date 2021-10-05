use core::convert::TryFrom;

use anyhow::{anyhow, bail, Result};
use bytes::Bytes;
use tobu_format::wire::{parse_varint, FieldValue, Parser};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Version {
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileDescriptorProto {
    pub name: Option<String>,
    pub package: Option<String>,
    pub dependency: Vec<String>,
    pub message_type: Vec<DescriptorProto>,
    pub options: Option<FileOptions>,
    pub source_code_info: Option<SourceCodeInfo>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileOptions {
    pub java_package: Option<String>,
    pub java_outer_classname: Option<String>,
    pub optimize_for: Option<FileDescriptorOptimizeMode>,
    pub go_package: Option<String>,
    pub cc_enable_arenas: Option<bool>,
    pub objc_class_prefix: Option<String>,
    pub csharp_namespace: Option<String>,
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

#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FileDescriptorOptimizeMode {
    Speed = 1,
    CodeSize = 2,
    LiteRuntime = 3,
}

impl Default for FileDescriptorOptimizeMode {
    fn default() -> Self {
        FileDescriptorOptimizeMode::Speed
    }
}

impl TryFrom<i32> for FileDescriptorOptimizeMode {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FileDescriptorOptimizeMode::Speed),
            2 => Ok(FileDescriptorOptimizeMode::CodeSize),
            3 => Ok(FileDescriptorOptimizeMode::LiteRuntime),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: Vec<String>,
    pub parameter: Option<String>,
    pub proto_file: Vec<FileDescriptorProto>,
    pub compiler_version: Option<Version>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescriptorProto {
    pub name: Option<String>,
    pub field: Vec<FieldDescriptorProto>,
    pub nested_type: Vec<DescriptorProto>,
    pub enum_type: Vec<EnumDescriptorProto>,
    pub extension_range: Vec<DescriptorProtoExtensionRange>,
    pub reserved_range: Vec<DescriptorProtoReservedRange>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescriptorProtoReservedRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumDescriptorProto {
    pub name: Option<String>,
    pub value: Vec<EnumValueDescriptorProto>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnumValueDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DescriptorProtoExtensionRange {
    pub start: Option<i32>,
    pub end: Option<i32>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldDescriptorProto {
    pub name: Option<String>,
    pub number: Option<i32>,
    pub label: Option<FieldDescriptorProtoLabel>,
    pub r#type: Option<FieldDescriptorProtoType>,
    pub type_name: Option<String>,
    pub default_value: Option<String>,
    pub oneof_index: Option<i32>,
    pub json_name: Option<String>,
    pub options: Option<FieldOptions>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct FieldOptions {
    pub deprecated: Option<bool>,
    pub packed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FieldDescriptorProtoLabel {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}

impl Default for FieldDescriptorProtoLabel {
    fn default() -> Self {
        FieldDescriptorProtoLabel::Optional
    }
}

impl TryFrom<i32> for FieldDescriptorProtoLabel {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FieldDescriptorProtoLabel::Optional),
            2 => Ok(FieldDescriptorProtoLabel::Required),
            3 => Ok(FieldDescriptorProtoLabel::Repeated),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum FieldDescriptorProtoType {
    Double = 1,
    Float = 2,
    Int64 = 3,
    UInt64 = 4,
    Int32 = 5,
    Fixed64 = 6,
    Fixed32 = 7,
    Bool = 8,
    String = 9,
    Group = 10,
    Message = 11,
    Bytes = 12,
    UInt32 = 13,
    Enum = 14,
    SFixed32 = 15,
    SFixed64 = 16,
    SInt32 = 17,
    SInt64 = 18,
}

impl Default for FieldDescriptorProtoType {
    fn default() -> Self {
        FieldDescriptorProtoType::Double
    }
}

impl TryFrom<i32> for FieldDescriptorProtoType {
    type Error = i32;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(FieldDescriptorProtoType::Double),
            2 => Ok(FieldDescriptorProtoType::Float),
            3 => Ok(FieldDescriptorProtoType::Int64),
            4 => Ok(FieldDescriptorProtoType::UInt64),
            5 => Ok(FieldDescriptorProtoType::Int32),
            6 => Ok(FieldDescriptorProtoType::Fixed64),
            7 => Ok(FieldDescriptorProtoType::Fixed32),
            8 => Ok(FieldDescriptorProtoType::Bool),
            9 => Ok(FieldDescriptorProtoType::String),
            10 => Ok(FieldDescriptorProtoType::Group),
            11 => Ok(FieldDescriptorProtoType::Message),
            12 => Ok(FieldDescriptorProtoType::Bytes),
            13 => Ok(FieldDescriptorProtoType::UInt32),
            14 => Ok(FieldDescriptorProtoType::Enum),
            15 => Ok(FieldDescriptorProtoType::SFixed32),
            16 => Ok(FieldDescriptorProtoType::SFixed64),
            17 => Ok(FieldDescriptorProtoType::SInt32),
            18 => Ok(FieldDescriptorProtoType::SInt64),
            _ => Err(value),
        }
    }
}

pub fn parse_request(b: Bytes) -> Result<CodeGeneratorRequest> {
    let mut req = CodeGeneratorRequest::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => req.file_to_generate.push(String::from_utf8(b.to_vec())?),
            (3, FieldValue::Bytes(b)) => req.compiler_version = Some(parse_version(b)?),
            (15, FieldValue::Bytes(b)) => req.proto_file.push(parse_file(b)?),
            (num, val) => bail!("parse_req {:?}, {:?}", num, val),
        }
    }

    Ok(req)
}

fn parse_version(b: Bytes) -> Result<Version> {
    let mut ver = Version::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Varint(i)) => ver.major = Some(i32::try_from(i)?),
            (2, FieldValue::Varint(i)) => ver.minor = Some(i32::try_from(i)?),
            (3, FieldValue::Varint(i)) => ver.patch = Some(i32::try_from(i)?),
            (4, FieldValue::Bytes(b)) => ver.suffix = Some(String::from_utf8(b.to_vec())?),
            (num, val) => bail!("parse_version {:?}, {:?}", num, val),
        };
    }
    Ok(ver)
}

fn parse_file(b: Bytes) -> Result<FileDescriptorProto> {
    let mut file = FileDescriptorProto::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => file.name = Some(String::from_utf8(b.to_vec())?),
            (2, FieldValue::Bytes(b)) => file.package = Some(String::from_utf8(b.to_vec())?),
            (3, FieldValue::Bytes(b)) => file.dependency.push(String::from_utf8(b.to_vec())?),
            (4, FieldValue::Bytes(b)) => file.message_type.push(parse_message(b)?),
            (8, FieldValue::Bytes(b)) => file.options = Some(parse_file_options(b)?),
            (9, FieldValue::Bytes(b)) => file.source_code_info = Some(parse_source_code_info(b)?),
            (num, val) => bail!("parse_file {:?}, {:?}", num, val),
        };
    }
    Ok(file)
}

fn parse_message(b: Bytes) -> Result<DescriptorProto> {
    let mut message = DescriptorProto::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => message.name = Some(String::from_utf8(b.to_vec())?),
            (2, FieldValue::Bytes(b)) => message.field.push(parse_field(b)?),
            (3, FieldValue::Bytes(b)) => message.nested_type.push(parse_message(b)?),
            (4, FieldValue::Bytes(b)) => message.enum_type.push(parse_enum(b)?),
            (5, FieldValue::Bytes(b)) => message.extension_range.push(parse_extension_range(b)?),
            (9, FieldValue::Bytes(b)) => message.reserved_range.push(parse_reserved_range(b)?),
            (num, val) => bail!("parse_message {:?}, {:?}", num, val),
        };
    }
    Ok(message)
}

fn parse_field(b: Bytes) -> Result<FieldDescriptorProto> {
    let mut field = FieldDescriptorProto::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => field.name = Some(String::from_utf8(b.to_vec())?),
            (3, FieldValue::Varint(i)) => field.number = Some(i32::try_from(i)?),
            (4, FieldValue::Varint(i)) => {
                field.label = Some(
                    FieldDescriptorProtoLabel::try_from(i32::try_from(i)?)
                        .map_err(|i| anyhow!("Invalid FieldDescriptorProtoLabel {}", i))?,
                )
            }
            (5, FieldValue::Varint(i)) => {
                field.r#type = Some(
                    FieldDescriptorProtoType::try_from(i32::try_from(i)?)
                        .map_err(|i| anyhow!("Invalid FieldDescriptorProtoType {}", i))?,
                )
            }
            (6, FieldValue::Bytes(b)) => field.type_name = Some(String::from_utf8(b.to_vec())?),
            (7, FieldValue::Bytes(b)) => field.default_value = Some(String::from_utf8(b.to_vec())?),
            (8, FieldValue::Bytes(b)) => field.options = Some(parse_field_options(b)?),
            (10, FieldValue::Bytes(b)) => field.json_name = Some(String::from_utf8(b.to_vec())?),
            (num, val) => bail!("parse_field {:?}, {:?}", num, val),
        };
    }
    Ok(field)
}

fn parse_extension_range(b: Bytes) -> Result<DescriptorProtoExtensionRange> {
    let mut range = DescriptorProtoExtensionRange::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Varint(i)) => range.start = Some(i32::try_from(i)?),
            (2, FieldValue::Varint(i)) => range.end = Some(i32::try_from(i)?),
            (num, val) => bail!("parse_extension_range {:?}, {:?}", num, val),
        };
    }
    Ok(range)
}

fn parse_enum(b: Bytes) -> Result<EnumDescriptorProto> {
    let mut enumeration = EnumDescriptorProto::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => enumeration.name = Some(String::from_utf8(b.to_vec())?),
            (2, FieldValue::Bytes(b)) => enumeration.value.push(parse_enum_value(b)?),
            (num, val) => bail!("parse_enum {:?}, {:?}", num, val),
        };
    }
    Ok(enumeration)
}

fn parse_enum_value(b: Bytes) -> Result<EnumValueDescriptorProto> {
    let mut value = EnumValueDescriptorProto::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => value.name = Some(String::from_utf8(b.to_vec())?),
            (2, FieldValue::Varint(i)) => value.number = Some(i32::try_from(i)?),
            (num, val) => bail!("parse_enum_value {:?}, {:?}", num, val),
        };
    }
    Ok(value)
}

fn parse_field_options(b: Bytes) -> Result<FieldOptions> {
    let mut options = FieldOptions::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (2, FieldValue::Varint(i)) => options.packed = Some(i != 0),
            (3, FieldValue::Varint(i)) => options.deprecated = Some(i != 0),
            (num, val) => bail!("parse_field_options {:?}, {:?}", num, val),
        };
    }
    Ok(options)
}

fn parse_reserved_range(b: Bytes) -> Result<DescriptorProtoReservedRange> {
    let mut range = DescriptorProtoReservedRange::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Varint(i)) => range.start = Some(i32::try_from(i)?),
            (2, FieldValue::Varint(i)) => range.end = Some(i32::try_from(i)?),
            (num, val) => bail!("parse_reserved_range {:?}, {:?}", num, val),
        };
    }
    Ok(range)
}

fn parse_file_options(b: Bytes) -> Result<FileOptions> {
    let mut options = FileOptions::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => {
                options.java_package = Some(String::from_utf8(b.to_vec())?)
            }
            (8, FieldValue::Bytes(b)) => {
                options.java_outer_classname = Some(String::from_utf8(b.to_vec())?)
            }
            (9, FieldValue::Varint(i)) => {
                options.optimize_for = Some(
                    FileDescriptorOptimizeMode::try_from(i32::try_from(i)?)
                        .map_err(|i| anyhow!("Invalid FileDescriptorOptimizeMode {}", i))?,
                )
            }
            (11, FieldValue::Bytes(b)) => options.go_package = Some(String::from_utf8(b.to_vec())?),
            (31, FieldValue::Varint(i)) => options.cc_enable_arenas = Some(i != 0),
            (36, FieldValue::Bytes(b)) => {
                options.objc_class_prefix = Some(String::from_utf8(b.to_vec())?)
            }
            (37, FieldValue::Bytes(b)) => {
                options.csharp_namespace = Some(String::from_utf8(b.to_vec())?)
            }
            (num, val) => bail!("parse_file_options {:?}, {:?}", num, val),
        };
    }
    Ok(options)
}

fn parse_source_code_info(b: Bytes) -> Result<SourceCodeInfo> {
    let mut info = SourceCodeInfo::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(b)) => info.location.push(parse_location(b)?),
            (num, val) => bail!("parse_source_code_info {:?}, {:?}", num, val),
        };
    }
    Ok(info)
}

fn parse_location(b: Bytes) -> Result<SourceCodeInfoLocation> {
    let mut location = SourceCodeInfoLocation::default();
    for r in Parser::new(b) {
        match r.map(|f| (f.num.get(), f.val))? {
            (1, FieldValue::Bytes(mut b)) => {
                while !b.is_empty() {
                    location.path.push(i32::try_from(parse_varint(&mut b)?)?);
                }
            }
            (2, FieldValue::Bytes(mut b)) => {
                while !b.is_empty() {
                    location.span.push(i32::try_from(parse_varint(&mut b)?)?);
                }
            }
            (3, FieldValue::Bytes(b)) => {
                location.leading_comments = Some(String::from_utf8(b.to_vec())?)
            }
            (4, FieldValue::Bytes(b)) => {
                location.trailing_comments = Some(String::from_utf8(b.to_vec())?)
            }
            (6, FieldValue::Bytes(b)) => location
                .leading_detached_comments
                .push(String::from_utf8(b.to_vec())?),
            (num, val) => bail!("parse_location {:?}, {:?}", num, val),
        };
    }
    Ok(location)
}
