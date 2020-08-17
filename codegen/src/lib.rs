extern crate proc_macro;

#[macro_use]
extern crate strum_macros;

mod entity_metadata;

use heck::SnakeCase;
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::export::Span;
use syn::Data;
use syn::Ident;
use syn::Type;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AsAny)]
pub fn derive_as_any(_item: TokenStream) -> TokenStream {
    let parsed: DeriveInput = parse_macro_input!(_item as DeriveInput);

    let name = &parsed.ident;

    let result = quote! {
        impl AsAny for #name {
            fn as_any(&self) -> &Any {
                self
            }
        }
    };

    result.into()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum PacketParameterType {
    Varint,
    Varlong,
    String,
    U64,
    U32,
    U16,
    U8,
    I64,
    I32,
    I16,
    I8,
    Position,
    Boolean,
    F32,
    F64,
    Uuid,
    Nbt,
    Slot,
    EntityMetadata,
}

lazy_static! {
    static ref PARAMETER_MAPPINGS: HashMap<&'static str, PacketParameterType> = {
        let mut m = HashMap::new();

        m.insert("VarInt", PacketParameterType::Varint);
        m.insert("VarLong", PacketParameterType::Varlong);
        m.insert("String", PacketParameterType::String);
        m.insert("u64", PacketParameterType::U64);
        m.insert("u32", PacketParameterType::U32);
        m.insert("u16", PacketParameterType::U16);
        m.insert("u8", PacketParameterType::U8);
        m.insert("i64", PacketParameterType::I64);
        m.insert("i32", PacketParameterType::I32);
        m.insert("i16", PacketParameterType::I16);
        m.insert("i8", PacketParameterType::I8);
        m.insert("BlockPosition", PacketParameterType::Position);
        m.insert("bool", PacketParameterType::Boolean);
        m.insert("f32", PacketParameterType::F32);
        m.insert("f64", PacketParameterType::F64);
        m.insert("Uuid", PacketParameterType::Uuid);
        m.insert("Blob", PacketParameterType::Nbt);
        m.insert("Slot", PacketParameterType::Slot);
        m.insert("EntityMetadata", PacketParameterType::EntityMetadata);

        m
    };

    static ref FUNCTION_MAPPINGS: HashMap<PacketParameterType, &'static str> = {
        let mut m = HashMap::new();

        m.insert(PacketParameterType::Varint, "var_int");
        m.insert(PacketParameterType::Varlong, "var_long");
        m.insert(PacketParameterType::String, "string");
        m.insert(PacketParameterType::U64, "u64");
        m.insert(PacketParameterType::U32, "u32");
        m.insert(PacketParameterType::U16, "u16");
        m.insert(PacketParameterType::U8, "u8");
        m.insert(PacketParameterType::I64, "i64");
        m.insert(PacketParameterType::I32, "i32");
        m.insert(PacketParameterType::I16, "i16");
        m.insert(PacketParameterType::I8, "i8");
        m.insert(PacketParameterType::Position, "position");
        m.insert(PacketParameterType::Boolean, "bool");
        m.insert(PacketParameterType::F32, "f32");
        m.insert(PacketParameterType::F64, "f64");
        m.insert(PacketParameterType::Uuid, "uuid");
        m.insert(PacketParameterType::Nbt, "nbt");
        m.insert(PacketParameterType::Slot, "slot");
        m.insert(PacketParameterType::EntityMetadata, "metadata");

        m
    };
}

#[proc_macro_derive(Packet)]
pub fn derive_packet(_item: TokenStream) -> TokenStream {
    let item: DeriveInput = parse_macro_input!(_item as DeriveInput);

    let ident = item.ident.clone();

    let fields = match &item.data {
        Data::Struct(st) => &st.fields,
        _ => panic!("Not a struct"),
    };

    let mut write_code = vec![];
    let mut read_code = vec![];

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let ty = match &field.ty {
            Type::Path(path) => &path.path.segments,
            _ => panic!("Not a path field"),
        };

        if ty.len() != 1 {
            panic!("Must not use paths");
        }

        let ty_ident = &ty.first().unwrap().ident;

        let parameter_type = PARAMETER_MAPPINGS
            .get(ty_ident.to_string().as_str())
            .unwrap_or_else(|| {
                panic!(
                    "Couldn't find packet parameter type corresponding to {}",
                    ty_ident
                )
            });
        let function_ident = Ident::new(
            FUNCTION_MAPPINGS.get(parameter_type).unwrap(),
            Span::call_site(),
        );

        let write_fn_ident = Ident::new(&format!("push_{}", function_ident), Span::call_site());
        let read_fn_ident = Ident::new(&format!("try_get_{}", function_ident), Span::call_site());

        let use_ref = {
            vec![
                PacketParameterType::Position,
                PacketParameterType::String,
                PacketParameterType::Uuid,
                PacketParameterType::Nbt,
                PacketParameterType::EntityMetadata,
            ]
            .contains(parameter_type)
        };

        let write;

        if use_ref {
            write = quote! {
                buf.#write_fn_ident(&self.#field_name);
            };
        } else {
            write = quote! {
                buf.#write_fn_ident(self.#field_name);
            }
        }

        let read;

        read = quote! {
            self.#field_name = buf.#read_fn_ident()?;
        };

        write_code.push(write);
        read_code.push(read);
    }

    let r = quote! {
        impl Packet for #ident {
            fn read_from(&mut self, mut buf: &mut Cursor<&[u8]>) -> anyhow::Result<()> {
                #(#read_code)*
                Ok(())
            }

            fn write_to(&self, buf: &mut BytesMut) {
                #(#write_code)*
            }

            fn ty(&self) -> PacketType {
                PacketType::#ident
            }

            fn ty_sized() -> PacketType where Self: Sized {
                PacketType::#ident
            }

            fn box_clone(&self) -> Box<dyn Packet> {
                Box::new((*self).clone())
            }
        }
    };

    r.into()
}

#[proc_macro_derive(FromSnakeCase)]
pub fn derive_from_snake_case(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let mut match_arms = vec![];

    match &input.data {
        syn::Data::Enum(en) => {
            for variant in &en.variants {
                let snake_case = variant.ident.to_string().to_snake_case();
                let ident = &variant.ident;
                match_arms.push(quote! {
                    #snake_case => Some(#name::#ident)
                });
            }
        }
        _ => panic!("Can only derive `FromSnakeCase` on enums"),
    }

    let result = quote! {
        impl FromSnakeCase for #name {
            fn from_snake_case(val: &str) -> Option<Self> {
                match val {
                    #(#match_arms ,)*
                    _ => None,
                }
            }
        }
    };

    result.into()
}

#[proc_macro_derive(ToSnakeCase)]
pub fn derive_to_snake_case(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &input.ident;

    let mut match_arms = vec![];

    match &input.data {
        syn::Data::Enum(en) => {
            for variant in &en.variants {
                let snake_case = variant.ident.to_string().to_snake_case();
                let ident = &variant.ident;
                match_arms.push(quote! {
                    #name::#ident => #snake_case.to_string()
                });
            }
        }
        _ => panic!("Can only derive `ToSnakeCase` on enums"),
    }

    let result = quote! {
        impl ToSnakeCase for #name {
            fn to_snake_case(&self) -> String {
                match self {
                    #(#match_arms),*
                }
            }
        }
    };

    result.into()
}

#[proc_macro]
pub fn entity_metadata(input: TokenStream) -> TokenStream {
    entity_metadata::entity_metadata(input)
}
