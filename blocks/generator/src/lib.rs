use crate::load::ident;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, HashMap};
use std::ops::RangeInclusive;
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
}

/// Generates code for the block report.
pub fn generate() -> anyhow::Result<Output> {
    let blocks = load::load()?;

    let mut output = Output::default();

    output.kind.push_str(&generate_kind(&blocks).to_string());
    let table_src = generate_table(&blocks);
    output.block_table.push_str(&table_src.to_string());

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

        #[derive(Debug)]
        pub struct BlockTable {
            #(#fields,)*
        }

        impl BlockTable {
            #(#fns)*
        }

        #(#types)*
    }
}
