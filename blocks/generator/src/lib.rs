use crate::load::ident;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::ops::RangeInclusive;
use std::str::FromStr;
use syn::export::ToTokens;

mod load;

struct Blocks {
    property_types: BTreeMap<String, Property>,
    blocks: Vec<Block>,
}

pub struct Block {
    /// Lowercase name of this block, minecraft: prefix removed.
    name: Ident,
    /// `name.to_camel_case()`
    name_camel_case: Ident,
    /// This block's properties.
    properties: Vec<String>,
    /// Default state and its property values.
    default_state: Vec<(String, String)>,
    /// Block states mapped to vanilla state IDs.
    ids: Vec<(Vec<(String, TokenStream)>, u16)>,
    /// Strides and offset coefficients for each property of this block.
    index_parameters: HashMap<String, (u16, u16)>,
}

#[derive(Debug)]
struct Property {
    /// Name of this property, with Rust keywords removed. (e.g. "type" => "kind")
    name: Ident,
    /// CamelCase name of this property if it were a struct or enum.
    ///
    /// Often prefixed with the name of the block to which this property belongs.
    name_camel_case: Ident,
    /// The kind of this property.
    kind: PropertyKind,
}

impl Property {
    /// Returns the tokens to create an instance of this property from a `u16`.
    pub fn tokens_for_from_u16(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { .. } => quote! {{ #input as i32 }},
            PropertyKind::Boolean { .. } => quote! { if #input == 0 { false } else { true } },
            PropertyKind::Enum { name, .. } => {
                quote! { #name::try_from(#input).expect("invalid block state") }
            }
        }
    }

    /// Returns an expression for a value of this property.
    fn expr_for_value(&self, value: &str) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { .. } => {
                let value = i32::from_str(value).unwrap();
                quote! { #value }
            }
            PropertyKind::Boolean => {
                let value = bool::from_str(value).unwrap();
                quote! { #value }
            }
            PropertyKind::Enum { name, .. } => {
                let variant = ident(value);
                quote! { #name::#variant }
            }
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
enum PropertyKind {
    Integer { range: RangeInclusive<i32> },
    Boolean,
    Enum { name: Ident, variants: Vec<Ident> },
}

#[derive(Debug, Default)]
pub struct Output {
    pub kind: String,
    pub block_fns: String,
    pub block_table: String,
    pub block_table_serialized: Vec<u8>,
}

/// Generates code for the block report.
pub fn generate() -> anyhow::Result<Output> {
    let blocks = load::load()?;

    let mut output = Output::default();

    output.kind.push_str(&generate_kind(&blocks).to_string());
    let table_src = generate_table(&blocks);
    output.block_table.push_str(&table_src.to_string());
    let block_fns_src = generate_block_fns(&blocks);
    output.block_fns.push_str(&block_fns_src.to_string());

    output.block_table_serialized = serialize_block_table(&blocks);

    Ok(output)
}

/// Generates the `BlockKind` enum.
fn generate_kind(blocks: &Blocks) -> TokenStream {
    let mut variants = vec![];

    for block in &blocks.blocks {
        let name = &block.name_camel_case;
        variants.push(quote! { #name });
    }

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ToPrimitive, FromPrimitive)]
        #[repr(u16)]
        pub enum BlockKind {
            #(#variants,)*
        }
    }
}

/// Generates the `BlockTable` struct and its implementation.
fn generate_table(blocks: &Blocks) -> TokenStream {
    let mut fields = vec![];
    let mut fns = vec![];
    let mut types = vec![];

    for property in blocks.property_types.values() {
        let name = &property.name;

        types.push(property.tokens_for_definition());

        fields.push(quote! {
            #name: Vec<(u16, u16)>
        });

        let from_u16 = property.tokens_for_from_u16(quote! { x });

        let doc = format!(
            "Retrieves the `{}` value for the given block kind with the given state value.
        Returns the value of the property, or `None` if it does not exist.",
            name
        );
        fns.push(quote! {
            #[doc = #doc]
            pub fn #name(&self, kind: BlockKind, state: u16) -> Option<#property> {
                let (offset_coefficient, stride) = self.#name[kind as u16 as usize];

                if offset_coefficient == 0 {
                    return None;
                }

                let x = crate::n_dimensional_index(state, offset_coefficient, stride);
                Some(#from_u16)
            }
        });

        let set = ident(format!("set_{}", name));
        let doc = format!("Updates the state value for the given block kind such that its `{}` value is updated. Returns the new state,
        or `None` if the block does not have this property.", name);
        fns.push(quote! {
            #[doc = #doc]
            pub fn #set(&self, kind: BlockKind, state: u16, value: #property) -> Option<u16> {
                let (offset_coefficient, stride) = self.#name[kind as u16 as usize];

                if offset_coefficient == 0 {
                    return None;
                }

                let base = state % offset_coefficient;
                let multiplier = state / offset_coefficient;

                let mut new = (value as u16 - (base / stride)) * stride + base;
                new += multiplier * offset_coefficient;
                Some(new)
            }
        });
    }

    quote! {
        use crate::BlockKind;
        use std::convert::TryFrom;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        pub struct BlockTable {
            #(#fields,)*
        }

        impl BlockTable {
            #(#fns)*
        }

        #(#types)*
    }
}

/// Generated functions for `BlockId`.
fn generate_block_fns(blocks: &Blocks) -> TokenStream {
    let mut fns = vec![];

    for block in &blocks.blocks {
        let name = &block.name;
        let name_camel_case = &block.name_camel_case;

        let default_state = &block.default_state;

        let mut state_intializers = vec![];
        for (name, value) in default_state {
            let value_expr = blocks.property_types[name].expr_for_value(value);

            let name_fn = ident(format!("with_{}", name));
            state_intializers.push(quote! {
                block = block.#name_fn(#value_expr).unwrap();
            });
        }

        let mut doc = format!(
            "Returns an instance of `{}` with default state values.
        
        The default state values are as follows:",
            block.name
        );
        for (name, value) in default_state {
            doc.push_str(&format!("* `{}`: {}", name, value));
        }

        fns.push(quote! {
            #[doc = #doc]
            pub fn #name() -> Self {
                let mut block = Self {
                    kind: BlockKind::#name_camel_case,
                    state: 0,
                };
                #(#state_intializers)*
                block
            }
        })
    }

    let res = quote! {
        use crate::{BlockId, BlockKind, BLOCK_TABLE};

        impl BlockId {
            #(#fns)*
        }
    };
    res
}

/// Returns the serialized `BlockTable`.
fn serialize_block_table(blocks: &Blocks) -> Vec<u8> {
    let table = BlockTableSerialize::new(&blocks.blocks, &blocks.property_types);

    bincode::serialize(&table).expect("bincode failed to serialize block table")
}

/// Serializable form of the generated `BlockTable`.
#[derive(Debug)]
struct BlockTableSerialize {
    fields: BTreeMap<String, Vec<(u16, u16)>>,
}

// custom serialize impl needed because of https://github.com/servo/bincode/issues/245
impl Serialize for BlockTableSerialize {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("BlockTable", self.fields.len())?;

        for (name, value) in &self.fields {
            // Leak memory! This is a build script; it doesn't matter.
            let name = Box::leak(name.clone().into_boxed_str());
            state.serialize_field(name, value)?;
        }

        state.end()
    }
}

impl BlockTableSerialize {
    pub fn new(blocks: &[Block], property_types: &BTreeMap<String, Property>) -> Self {
        let mut fields: BTreeMap<String, Vec<(u16, u16)>> = BTreeMap::new();

        for block in blocks {
            for property_name in property_types.keys() {
                let index_parameters = match block.index_parameters.get(property_name) {
                    Some(params) => *params,
                    None => (0, 0),
                };

                fields
                    .entry(property_name.clone())
                    .or_default()
                    .push(index_parameters);
            }
        }

        assert!(fields.values().map(Vec::len).all(|len| len == blocks.len()));

        Self { fields }
    }
}
