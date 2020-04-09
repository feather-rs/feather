use crate::load::ident;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, HashMap};
use std::ops::RangeInclusive;
use syn::export::ToTokens;

mod load;

type PropertyIdentifier = String;

#[derive(Debug)]
struct Blocks(Vec<Block>);

static KNOWN_PROPERTY_TYPES: Lazy<Vec<(fn(&str, &[&str]) -> bool, &'static str)>> =
    Lazy::new(|| {
        vec![
            (
                |name, values| {
                    name == "facing" && values.contains(&"north") && !values.contains(&"down")
                },
                "FacingCardinal",
            ),
            (
                |name, values| {
                    name == "facing" && values.contains(&"north") && values.contains(&"down")
                },
                "FacingCubic",
            ),
            (
                |name, values| name == "shape" && values.contains(&"straight"),
                "StairsShape",
            ),
            (
                |name, values| name == "half" && values == ["top", "bottom"],
                "Half",
            ),
        ]
    });

#[derive(Debug)]
pub struct Block {
    /// Lowercase name of this block, minecraft: prefix removed.
    name: Ident,
    /// Name of the block's properties struct.
    property_struct_name: Ident,
    /// This block's properties.
    properties: HashMap<PropertyIdentifier, Property>,
    /// The possible states for this block.
    states: Vec<State>,
    /// The minimum state ID for this block.
    base_id: u16,
}

#[derive(Debug)]
struct Property {
    /// Name of this property, with Rust keywords removed. (e.g. "type" => "kind")
    name: Ident,
    /// CamelCase name of this property if it were a struct or enum.
    ///
    /// Often prefixed with the name of the block to which this property belongs.
    struct_name: Ident,
    /// The possible values of this property.
    possible_values: Vec<PropertyValue>,
    /// The kind of this property.
    kind: PropertyKind,
}

impl Property {
    /// Returns the tokens to create an instance of this property from a `u16`.
    pub fn tokens_for_from_u16(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { .. } => quote! {{ #input as i32 }},
            PropertyKind::Boolean { .. } => quote! { if #input == 0 { false } else { true } },
            PropertyKind::Enum { name, .. } => quote! { #name::try_from(#input)? },
        }
    }

    /// Returns the number of possible values for this property.
    pub fn num_possible_values(&self) -> u16 {
        match &self.kind {
            PropertyKind::Integer { range } => (*range.end() - *range.start() + 1) as u16,
            PropertyKind::Boolean => 2,
            PropertyKind::Enum { variants, .. } => variants.len() as u16,
        }
    }
}

impl ToTokens for Property {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = match &self.kind {
            PropertyKind::Integer { .. } => quote! { i32 },
            PropertyKind::Boolean => quote! { bool },
            PropertyKind::Enum { name, .. } => quote! { #name },
        };

        tokens.extend(x);
    }
}

impl Property {
    /// Returns the tokens necessary to define this property's type,
    /// i.e. if it is an enum.
    pub fn tokens_for_definition(&self) -> Option<TokenStream> {
        match &self.kind {
            PropertyKind::Enum { name, variants } => Some({
                let definition = quote! {
                    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
                    #[repr(u16)]
                    pub enum #name {
                        #(
                            #variants,
                        )*
                    }
                };

                let variant_indices: Vec<_> = (0..variants.len() as u16).collect();
                let try_from_error_msg = format!("invalid value {{}} for {}", name);

                let imp = quote! {
                    impl TryFrom<u16> for #name {
                        type Error = anyhow::Error;

                        fn try_from(value: u16) -> anyhow::Result<Self> {
                            match value {
                                #(
                                    #variant_indices => Ok(#name::#variants),
                                )*
                                x => Err(anyhow::anyhow!(#try_from_error_msg, x)),
                            }
                        }
                    }
                };

                quote! {
                    #definition
                    #imp
                }
            }),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum PropertyValue {
    Integer {
        range: RangeInclusive<i32>,
        value: i32,
    },
    Boolean(bool),
    Enum {
        name: Ident,
        variant: Ident,
    },
}

#[derive(Debug)]
enum PropertyKind {
    Integer { range: RangeInclusive<i32> },
    Boolean,
    Enum { name: Ident, variants: Vec<Ident> },
}

#[derive(Debug)]
struct State {
    /// The values of each property for this state.
    property_values: HashMap<PropertyIdentifier, PropertyValue>,
    /// The Minecraft state ID for this state.
    vanilla_id: u16,
}

#[derive(Debug, Default)]
pub struct Output {
    pub kind: String,
    pub block_fns: String,
    pub block_table: String,
}

/// Generates code for the block report.
pub fn generate() -> anyhow::Result<Output> {
    let blocks = load::load()?;

    let mut output = Output::default();

    output.kind.push_str(&generate_kind(&blocks.0).to_string());
    let (table_src, table_encoded) = generate_table(&blocks.0);
    output.block_table.push_str(&table_src.to_string());

    Ok(output)
}

fn generate_kind(blocks: &[Block]) -> TokenStream {
    let mut body = TokenStream::new();

    for block in blocks {
        let name = &block.property_struct_name;
        body.extend(quote! {
            #name,
        })
    }

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
        #[repr(u16)]
        pub enum BlockKind {
            #body
        }
    }
}

fn generate_table(blocks: &[Block]) -> (TokenStream, serde_cbor::Value) {
    let property_struct_names: BTreeMap<_, _> = blocks
        .iter()
        .flat_map(|block| block.properties.iter())
        .map(|(_, prop)| (prop.name.clone(), property_type_name(prop)))
        .collect();

    let mut fields = vec![];
    for identifier in property_struct_names.keys() {
        fields.push(quote! {
            pub(crate) #identifier: Vec<(u16, u16)>,
        });
    }

    let table = quote! {
        #[derive(Deserialize, Debug)]
        pub struct BlocksTable {
            #(#fields)*
        }
    };

    let mut fs = vec![];

    for (identifier, typ) in &property_struct_names {
        let convert_fn = if &typ.to_string() == "bool" {
            quote! { match x { 1 => Some(true), 0 => Some(false), _ => None } }
        } else {
            quote! { #typ::from_u16(x) }
        };

        let f = quote! {
            pub fn #identifier(&self, index: u16) -> Option<#typ> {
                let (offset_coefficient, stride) = self.#identifier[index as usize];

                if offset_coefficient == 0 {
                    return None;
                }

                let x = crate::n_dimensional_index(index, offset_coefficient, stride);
                #convert_fn
            }
        };
        fs.push(f);
    }

    let table = quote! {
        use num_traits::FromPrimitive;
        #table

        impl BlocksTable {
            #(#fs)*
        }
    };

    (table, serde_cbor::Value::Integer(0))
}

fn property_type_name(property: &Property) -> TokenStream {
    match &property.kind {
        PropertyKind::Enum { name, variants } => {
            let possible_values: Vec<_> = property
                .possible_values
                .iter()
                .map(|value| match value {
                    PropertyValue::Enum { variant, .. } => variant.to_string(),
                    _ => unreachable!(),
                })
                .collect();
            let possible_values: Vec<_> = possible_values.iter().map(String::as_str).collect();

            if let Some(typ) = KNOWN_PROPERTY_TYPES
                .iter()
                .filter_map(|(f, typ)| {
                    if f(&name.to_string(), &possible_values) {
                        Some(*typ)
                    } else {
                        None
                    }
                })
                .next()
            {
                ident(typ).into_token_stream()
            } else {
                property.struct_name.clone().into_token_stream()
            }
        }
        PropertyKind::Integer { .. } => quote! { i32 },
        PropertyKind::Boolean => quote! { bool },
    }
}
