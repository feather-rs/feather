extern crate proc_macro;

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
        m.insert("NbtTag", PacketParameterType::Nbt);

        m
    };

    static ref FUNCTION_MAPPINGS: HashMap<PacketParameterType, &'static str> = {
        let mut m = HashMap::new();

        m.insert("var_int", PacketParameterType::Varint);
        m.insert("var_long", PacketParameterType::Varlong);
        m.insert("string", PacketParameterType::String);
        m.insert("u64_be", PacketParameterType::U64);
        m.insert("u32_be", PacketParameterType::U32);
        m.insert("u16_be", PacketParameterType::U16);
        m.insert("u8", PacketParameterType::U8);
        m.insert("i64_be", PacketParameterType::I64);
        m.insert("i32_be", PacketParameterType::I32);
        m.insert("i16_be", PacketParameterType::I16);
        m.insert("i8", PacketParameterType::I8);
        m.insert("position", PacketParameterType::Position);
        m.insert("bool", PacketParameterType::Boolean);
        m.insert("f32_be", PacketParameterType::F32);
        m.insert("f64_be", PacketParameterType::F64);
        m.insert("uuid", PacketParameterType::Uuid);
        m.insert("nbt", PacketParameterType::Nbt);

        // I wrote them in the wrong order, so I'm just going to reverse
        // the map.
        let mut reversed = HashMap::new();

        for (key, value) in m {
            reversed.insert(value, key);
        }

        reversed
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

        let ty_ident = &ty.first().unwrap().value().ident;

        let parameter_type = PARAMETER_MAPPINGS
            .get(ty_ident.to_string().as_str())
            .expect(&format!(
                "Couldn't find packet parameter type corresponding to {}",
                ty_ident
            ));
        let function_ident = Ident::new(
            FUNCTION_MAPPINGS.get(parameter_type).unwrap(),
            Span::call_site(),
        );

        let write_fn_ident = Ident::new(&format!("write_{}", function_ident), Span::call_site());
        let read_fn_ident = Ident::new(&format!("read_{}", function_ident), Span::call_site());

        let use_ref = {
            vec![
                PacketParameterType::Position,
                PacketParameterType::String,
                PacketParameterType::Uuid,
                PacketParameterType::Nbt,
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
            fn read_from(&mut self, mut buf: &mut PacketBuf) -> Result<(), ()> {
                #(#read_code)*
                Ok(())
            }

            fn write_to(&self, buf: &mut ByteBuf) {
                #(#write_code)*
            }

            fn ty(&self) -> PacketType {
                PacketType::#ident
            }
        }
    };

    r.into()
}
