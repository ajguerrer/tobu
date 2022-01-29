use std::path::PathBuf;

use anyhow::{anyhow, bail, Result};

use crate::parse::{
    DescriptorProto, EnumDescriptorProto, FieldDescriptorProto, FieldDescriptorProtoLabel,
    FieldDescriptorProtoType, FileDescriptorProto,
};

#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub dependencies: Vec<Vec<String>>,
    pub messages: Vec<Message>,
}

#[derive(Debug)]
pub struct Message {
    pub name: String,
    pub nested: Vec<Message>,
    pub enums: Vec<Enum>,
    pub fields: Vec<Field>,
}
#[derive(Debug)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
}

#[derive(Debug)]
pub struct EnumValue {
    pub name: String,
    pub number: i32,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub cardinality: Cardinality,
    pub ty: FieldType,
    pub default_value: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum FieldType {
    Double,
    Float,
    Int64,
    UInt64,
    Int32,
    Fixed64,
    Fixed32,
    Bool,
    String,
    Group(String),
    Message(String),
    Bytes,
    UInt32,
    Enum(String),
    SFixed32,
    SFixed64,
    SInt32,
    SInt64,
}

#[derive(Debug, PartialEq)]
pub enum Cardinality {
    Optional,
    Required,
    Repeated,
}

pub fn process_files(files: &[FileDescriptorProto]) -> Result<Vec<File>> {
    files.iter().map(process_file).collect()
}

fn process_file(file: &FileDescriptorProto) -> Result<File> {
    let name = file
        .name
        .as_ref()
        .ok_or_else(|| anyhow!("File name expected: {:?}", file))?;

    let package = file
        .package
        .as_ref()
        .ok_or_else(|| anyhow!("File package expected: {:?}", file))?;
    Ok(File {
        path: process_path(name, package),
        dependencies: process_dependencies(&file.dependency),
        messages: process_messages(&file.message_type)?,
    })
}

fn process_path(name: &str, package: &str) -> PathBuf {
    let mut p = PathBuf::new();
    p.push("src");
    for dir in package.split('.') {
        p.push(dir);
    }
    // split/rsplit will always return an iterator with something in it
    p.push(
        name.rsplit('/')
            .next()
            .unwrap_or_default()
            .split('.')
            .next()
            .unwrap_or_default(),
    );
    p.set_extension("rs");
    p
}

fn process_dependencies(deps: &[String]) -> Vec<Vec<String>> {
    deps.iter()
        .map(|dep| {
            dep.trim_end_matches(".proto")
                .split('/')
                .map(|s| s.to_owned())
                .collect()
        })
        .collect()
}

fn process_messages(msgs: &[DescriptorProto]) -> Result<Vec<Message>> {
    msgs.iter().map(|msg| process_message(msg, "")).collect()
}

fn process_message(msg: &DescriptorProto, base_name: &str) -> Result<Message> {
    let name = base_name.to_string()
        + msg
            .name
            .as_ref()
            .ok_or_else(|| anyhow!("message name required {:#?}", msg))?;
    let nested = msg
        .nested_type
        .iter()
        .map(|n| process_message(n, &name))
        .collect::<Result<Vec<_>>>()?;
    let enums = msg
        .enum_type
        .iter()
        .map(|num| process_enum(num, &name))
        .collect::<Result<Vec<_>>>()?;
    let fields = msg
        .field
        .iter()
        .map(|f| process_field(f, &name, &nested, &enums))
        .collect::<Result<Vec<_>>>()?;
    Ok(Message {
        name,
        nested,
        enums,
        fields,
    })
}

fn process_enum(num: &EnumDescriptorProto, base_name: &str) -> Result<Enum> {
    let name = num
        .name
        .as_ref()
        .ok_or_else(|| anyhow!("enum name required {:#?}", num))?;
    let values = num
        .value
        .iter()
        .map(|val| {
            use heck::CamelCase;
            let value_name = val
                .name
                .as_ref()
                .ok_or_else(|| anyhow!("enum value name required {:#?}", num))?
                .to_camel_case();
            let name = value_name.trim_start_matches(name).to_string();
            let number = val
                .number
                .ok_or_else(|| anyhow!("enum value number required {:#?}", num))?;
            Ok(EnumValue { name, number })
        })
        .collect::<Result<Vec<_>>>()?;
    let name = base_name.to_string() + name;
    Ok(Enum { name, values })
}

fn process_field(
    field: &FieldDescriptorProto,
    base_name: &str,
    nested: &[Message],
    enums: &[Enum],
) -> Result<Field> {
    let mut name = field
        .name
        .as_ref()
        .ok_or_else(|| anyhow!("field name required {:#?}", field))?
        .clone();
    if name == "type" {
        name = "r#type".to_string()
    }
    let cardinality = match field
        .label
        .as_ref()
        .ok_or_else(|| anyhow!("field label required {:#?}", field))?
    {
        FieldDescriptorProtoLabel::Optional => Cardinality::Optional,
        FieldDescriptorProtoLabel::Required => Cardinality::Required,
        FieldDescriptorProtoLabel::Repeated => Cardinality::Repeated,
    };
    let type_name = field
        .type_name
        .as_ref()
        .map(|name| base_name.to_string() + name.rsplit('.').next().unwrap_or_default());

    let nested_names = nested.iter().map(|m| &m.name);
    let enum_names = enums.iter().map(|e| &e.name);
    let ty = if nested_names
        .chain(enum_names)
        .any(|name| type_name == Some(name.clone()))
    {
        process_field_type(field, base_name)?
    } else {
        process_field_type(field, "")?
    };

    let default_value = field
        .default_value
        .as_ref()
        .map(|v| process_default_value(v, &ty))
        .transpose()?;

    Ok(Field {
        name,
        cardinality,
        ty,
        default_value,
    })
}

fn process_default_value(default: &str, ty: &FieldType) -> Result<String> {
    match (default, ty) {
        (default, FieldType::Enum(name)) => {
            use heck::CamelCase;
            Ok(format!(
                "{}::{}",
                default,
                default.to_camel_case().trim_start_matches(name)
            ))
        }
        (default, FieldType::Group(group)) => bail!(
            "Default value({}) for FieldType::Group({}) not supported.",
            default,
            group
        ),
        (default, FieldType::Message(message)) => bail!(
            "Default value({}) for FieldType::Message({}) not supported.",
            default,
            message
        ),
        (default, _) => Ok(default.to_string()),
    }
}

fn process_field_type(field: &FieldDescriptorProto, base_name: &str) -> Result<FieldType> {
    let ty = match field
        .r#type
        .as_ref()
        .ok_or_else(|| anyhow!("field type required {:#?}", field))?
    {
        FieldDescriptorProtoType::Group => {
            FieldType::Group(process_field_type_name(field, base_name)?)
        }
        FieldDescriptorProtoType::Message => {
            FieldType::Message(process_field_type_name(field, base_name)?)
        }
        FieldDescriptorProtoType::Enum => {
            FieldType::Enum(process_field_type_name(field, base_name)?)
        }
        FieldDescriptorProtoType::Double => FieldType::Double,
        FieldDescriptorProtoType::Float => FieldType::Float,
        FieldDescriptorProtoType::Int64 => FieldType::Int64,
        FieldDescriptorProtoType::UInt64 => FieldType::UInt64,
        FieldDescriptorProtoType::Int32 => FieldType::Int32,
        FieldDescriptorProtoType::Fixed64 => FieldType::Fixed64,
        FieldDescriptorProtoType::Fixed32 => FieldType::Fixed32,
        FieldDescriptorProtoType::Bool => FieldType::Bool,
        FieldDescriptorProtoType::String => FieldType::String,
        FieldDescriptorProtoType::Bytes => FieldType::Bytes,
        FieldDescriptorProtoType::UInt32 => FieldType::UInt32,
        FieldDescriptorProtoType::SFixed32 => FieldType::SFixed32,
        FieldDescriptorProtoType::SFixed64 => FieldType::SFixed64,
        FieldDescriptorProtoType::SInt32 => FieldType::SInt32,
        FieldDescriptorProtoType::SInt64 => FieldType::SInt64,
    };
    Ok(ty)
}

fn process_field_type_name(field: &FieldDescriptorProto, base_name: &str) -> Result<String> {
    let type_name = field
        .type_name
        .as_ref()
        .ok_or_else(|| anyhow!("field type name required {:#?}", field))?;
    Ok(base_name.to_string() + type_name.rsplit('.').next().unwrap_or_default())
}
