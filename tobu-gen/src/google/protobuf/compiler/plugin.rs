#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]
use crate::google::protobuf::descriptor::*;
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Version {
    pub major: Option<i32>,
    pub minor: Option<i32>,
    pub patch: Option<i32>,
    pub suffix: Option<String>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CodeGeneratorRequest {
    pub file_to_generate: Vec<String>,
    pub parameter: Option<String>,
    pub proto_file: Vec<FileDescriptorProto>,
    pub compiler_version: Option<Version>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CodeGeneratorResponse {
    pub error: Option<String>,
    pub supported_features: Option<u64>,
    pub file: Vec<CodeGeneratorResponseFile>,
}
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CodeGeneratorResponseFile {
    pub name: Option<String>,
    pub insertion_point: Option<String>,
    pub content: Option<String>,
    pub generated_code_info: Option<GeneratedCodeInfo>,
}
#[derive(Debug, Clone, PartialEq)]
#[repr(i32)]
pub enum CodeGeneratorResponseFeature {
    None = 0i32,
    Proto3Optional = 1i32,
}
