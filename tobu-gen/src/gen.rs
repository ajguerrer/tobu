use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::process::{Cardinality, Enum, Field, FieldType, File, Message};

pub fn gen_file(file: &File) -> TokenStream {
    let use_crates = file.dependencies.iter().map(|mods| {
        let mods = mods.iter().map(|m| format_ident!("{}", m));
        quote! { use crate::#(#mods)::*::*; }
    });
    let messages = file.messages.iter().map(|m| gen_message(m));

    quote! {
        #![allow(dead_code)]
        #![allow(clippy::enum_variant_names)]
        #(#use_crates)*

        #(#messages)*
    }
}

fn gen_message(message: &Message) -> TokenStream {
    let name = format_ident!("{}", message.name);
    let fields = message.fields.iter().map(|field| gen_field(field));
    let nested = message.nested.iter().map(|message| gen_message(message));
    let enums = message.enums.iter().map(|num| gen_enum(num));

    quote! {
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct #name {
            #(pub #fields),*
        }

        #(#nested)*

        #(#enums)*
    }
}

fn gen_enum(num: &Enum) -> TokenStream {
    let name = format_ident!("{}", num.name);
    let values = num.values.iter().map(|v| {
        let name = format_ident!("{}", v.name);
        let number = v.number;
        quote! { #name = #number }
    });

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        #[repr(i32)]
        pub enum #name {
            #(#values),*
        }
    }
}

fn gen_field(field: &Field) -> TokenStream {
    let name = format_ident!("{}", field.name);
    let ty = gen_field_type(&field.cardinality, &field.ty);
    quote! {
        #name: #ty
    }
}

fn gen_field_type(cardinality: &Cardinality, ty: &FieldType) -> TokenStream {
    let mut tokens = TokenStream::new();
    match cardinality {
        Cardinality::Optional => {
            if *ty != FieldType::Bytes {
                tokens.extend(quote! { Option< });
            }
        }
        Cardinality::Required => {}
        Cardinality::Repeated => tokens.extend(quote! { Vec< }),
    };

    match ty {
        FieldType::Group(name) | FieldType::Message(name) | FieldType::Enum(name) => {
            let name = format_ident!("{}", name);
            tokens.extend(quote! { #name });
        }
        FieldType::Double => tokens.extend(quote! { f64 }),
        FieldType::Float => tokens.extend(quote! { f32 }),
        FieldType::Int64 => tokens.extend(quote! { i64 }),
        FieldType::UInt64 => tokens.extend(quote! { u64 }),
        FieldType::Int32 => tokens.extend(quote! { i32 }),
        FieldType::Fixed64 => tokens.extend(quote! { u64 }),
        FieldType::Fixed32 => tokens.extend(quote! { i32 }),
        FieldType::Bool => tokens.extend(quote! { bool}),
        FieldType::String => tokens.extend(quote! { String }),
        FieldType::Bytes => tokens.extend(quote! { Vec<u8> }),
        FieldType::UInt32 => tokens.extend(quote! { u32 }),
        FieldType::SFixed32 => tokens.extend(quote! { i32 }),
        FieldType::SFixed64 => tokens.extend(quote! { i64 }),
        FieldType::SInt32 => tokens.extend(quote! { i32 }),
        FieldType::SInt64 => tokens.extend(quote! { i64 }),
    }

    match cardinality {
        Cardinality::Optional => {
            if *ty != FieldType::Bytes {
                tokens.extend(quote! { > });
            }
        }
        Cardinality::Repeated => tokens.extend(quote! { > }),
        Cardinality::Required => {}
    }

    tokens
}
