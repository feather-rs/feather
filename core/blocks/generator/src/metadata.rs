//! Handles loading and output of block metadata,
//! taking from PrismarineJS/minecraft-data.
//!
//! This includes useful information such as drops,
//! light emission, hardness, bounding boxes, ...

use heck::CamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct BlockData<'a>(#[serde(borrow)] Vec<DataEntry<'a>>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DataEntry<'a> {
    id: i32,
    display_name: &'a str,
    name: &'a str,
    hardness: Option<f64>,
    min_state_id: i32,
    max_state_id: u32,
    drops: Vec<usize>,
    diggable: bool,
    transparent: bool,
    filter_light: u8,
    emit_light: u8,
    bounding_box: BoundingBox,
    stack_size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum BoundingBox {
    Empty,
    Block,
}

/// Generates block metadata file.
pub fn generate() -> anyhow::Result<String> {
    let data = parse_data()?;

    let result = generate_tokens(&data);
    Ok(result.to_string())
}

fn parse_data() -> anyhow::Result<BlockData<'static>> {
    const DATA: &[u8] = feather_data::minecraft_data::BLOCKS;

    serde_json::from_slice(DATA).map_err(anyhow::Error::from)
}

fn generate_tokens(data: &BlockData) -> TokenStream {
    let hardness = generate_hardness(data);
    let diggable = generate_diggable(data);
    let transparent = generate_transparent(data);

    quote! {
        impl crate::BlockId {
            #hardness
            #diggable
            #transparent
        }
    }
}

fn generate_hardness(data: &BlockData) -> TokenStream {
    let body = match_arms(data, |entry| {
        let hardness = entry.hardness.unwrap_or_default();
        quote! { #hardness }
    });

    quote! {
        #[doc = "Returns the hardness value of this block."]
        pub fn hardness(self) -> f64 {
            #body
        }
    }
}

fn generate_diggable(data: &BlockData) -> TokenStream {
    let body = match_arms(data, |entry| {
        let diggable = entry.diggable;
        quote! { #diggable }
    });

    quote! {
        #[doc = "Returns whether this block is diggable."]
        pub fn is_diggable(self) -> bool {
            #body
        }
    }
}

fn generate_transparent(data: &BlockData) -> TokenStream {
    let body = match_arms(data, |entry| {
        let opaque = !entry.transparent;
        quote! { #opaque }
    });

    quote! {
        #[doc = "Returns whether this block is opaque."]
        pub fn is_opaque(self) -> bool {
            #body
        }
    }
}

fn match_arms(data: &BlockData, mut f: impl FnMut(&DataEntry) -> TokenStream) -> TokenStream {
    let arms: Vec<_> = data
        .0
        .iter()
        .map(|entry| {
            let ident = kind_ident(entry.name);
            let block = f(entry);

            quote! {
                crate::BlockKind::#ident => { #block }
            }
        })
        .collect();

    quote! {
        match self.kind() {
            #(#arms,)*
        }
    }
}

fn kind_ident(name: &str) -> Ident {
    Ident::new(&name.to_camel_case(), Span::call_site())
}
