extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::fmt;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::ItemFn;
use syn::Result;
use syn::Token;

#[macro_use]
extern crate lazy_static;

#[derive(Clone, Copy, Debug)]
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
}

lazy_static! {
    static ref PARAMETER_MAPPINGS: im::HashMap<&'static str, PacketParameterType> = {
        let mut m = im::HashMap::new();

        m.insert("varint", PacketParameterType::Varint);
        m.insert("varlong", PacketParameterType::Varlong);
        m.insert("string", PacketParameterType::String);
        m.insert("u64", PacketParameterType::U64);
        m.insert("u32", PacketParameterType::U32);
        m.insert("u16", PacketParameterType::U16);
        m.insert("u8", PacketParameterType::U8);
        m.insert("i64", PacketParameterType::I64);
        m.insert("i32", PacketParameterType::I32);
        m.insert("i16", PacketParameterType::I16);
        m.insert("i8", PacketParameterType::I8);
        m.insert("position", PacketParameterType::Position);
        m.insert("boolean", PacketParameterType::Boolean);
        m.insert("f32", PacketParameterType::F32);
        m.insert("f64", PacketParameterType::F64);

        m
    };
}

impl fmt::Display for PacketParameterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct PacketParameter {
    name: syn::Ident,
    ty: PacketParameterType,
}

impl Parse for PacketParameter {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;

        let _: Token![:] = input.parse()?;

        let type_name: syn::Ident = input.parse()?;
        let ty = PARAMETER_MAPPINGS
            .get(type_name.to_string().as_str())
            .unwrap();

        Ok(PacketParameter {
            name,
            ty: ty.clone(),
        })
    }
}

struct Packet {
    name: syn::Ident,
    params: Punctuated<PacketParameter, Token![,]>,
}

impl Parse for Packet {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;

        let _: Token![,] = input.parse()?;

        let params = Punctuated::parse_separated_nonempty(input)?;

        Ok(Packet { name, params })
    }
}

#[proc_macro]
pub fn gen_packet(_item: TokenStream) -> TokenStream {
    let item: Packet = parse_macro_input!(_item as Packet);

    let name = &item.name;

    unimplemented!()
}
