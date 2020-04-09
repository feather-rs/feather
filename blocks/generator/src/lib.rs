use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::collections::{BTreeMap, HashMap};
use std::ops::RangeInclusive;
use std::str::FromStr;
use syn::export::ToTokens;

mod load;

type PropertyIdentifier = String;

macro_rules! detectors {
    ($($kind:ident $rule:literal),+) => {{
        vec![
            $(CategoryDetector::$kind($rule)),+
        ]
    }}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CategoryDetector {
    Yes(&'static str),
    Not(&'static str),
}

/// Set of block "categories." Blocks in a category share same properties
/// struct and values. If a block's identifier contains the category name,
/// then it belongs in the category. Example: there exists jungle_sampling, oak_sampling,
/// etc. all of which will share the Sapling properties struct.
static CATEGORIES: Lazy<BTreeMap<Vec<CategoryDetector>, &'static str>> = Lazy::new(|| {
    maplit::btreemap! {
        detectors!(Yes "sapling") => "Sapling",
        detectors!(Yes "lava", Yes "water") => "Liquid",
        detectors!(Yes "log", Not "stripped") => "Log",
        detectors!(Yes "stripped", Not "wood") => "StrippedLog",
        detectors!(Yes "wood", Not "stripped") => "Wood",
        detectors!(Yes "stripped", Not "log") => "StrippedWood",
        detectors!(Yes "leaves") => "Leaves",
        detectors!(Yes "bed") => "Bed",
        detectors!(Yes "stairs") => "Stairs",
        detectors!(Yes "fence", Not "gate") => "Fence",
        detectors!(Yes "stainedglass", Not "pane") => "StainedGlass",
        detectors!(Yes "trapdoor") => "Trapdoor",
        detectors!(Yes "fencegate") => "FenceGate",
        detectors!(Yes "stainedglasspane") => "StainedGlassPane",
        detectors!(Yes "slab") => "Slab",
        detectors!(Yes "planks") => "Planks",
        detectors!(Yes "concretepowder") => "ConcretePowder",
    }
});

#[derive(Debug)]
struct Blocks(Vec<Block>);

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
    /// Returns tokens for an iterator over all values of this property.
    pub fn tokens_for_value_iter(&self) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { range } => {
                let start = *range.start();
                let end = *range.end();
                quote! {
                    { #start..=#end }
                }
            }
            PropertyKind::Boolean => {
                quote! {
                    [false, true].iter().copied()
                }
            }
            PropertyKind::Enum { name, variants } => {
                quote! {
                    [
                        #(#name::#variants),*
                    ]
                    .iter()
                    .copied()
                }
            }
        }
    }

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

impl ToTokens for PropertyValue {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = match self {
            PropertyValue::Integer { range, value } => {
                let start = *range.start();
                let end = *range.end();
                let value = *value;
                quote! {
                    {
                        static_assertions::const_assert!(#start <= #value && #value <= #end);
                        #value
                    }
                }
            }
            PropertyValue::Boolean(value) => {
                let value = *value;
                quote! { #value }
            }
            PropertyValue::Enum { name, variant } => quote! { #name::#variant },
        };
        tokens.extend(x);
    }
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

impl State {
    /*
    /// Returns the tokens to create an instance of the block
    /// properties struct corresponding to this state.
    fn tokens(&self, block: &Block) -> TokenStream {
        let mut initializers = vec![];
        for (identifier, value) in &self.property_values {
            let name = &block.properties[identifier].name;

            initializers.push(quote! {
                #name: #value
            });
        }

        let struct_ident = &block.property_struct_name;

        quote! {
            #struct_ident {
                #(
                    #initializers,
                )*
            }
        }
    }
    */
}

/// Generates code for the block report.
pub fn generate() -> anyhow::Result<String> {
    let blocks = load::load()?;

    let mut res = String::from(
        "use std::collections::HashMap; use once_cell::sync::Lazy; use std::convert::TryFrom;\n",
    );

    for block in blocks.0 {
        res.push_str(&generate_properties_struct_and_impl(&block).to_string());
    }

    Ok(res)
}

/// Generates the implementation and definition of a block properties struct.
pub fn generate_properties_struct_and_impl(block: &Block) -> TokenStream {
    let definition = generate_properties_struct(block);
    let imp = generate_properties_impl(block);

    let extra_types: Vec<_> = block
        .properties
        .values()
        .map(|prop| prop.tokens_for_definition())
        .collect();

    quote! {
        #definition
        #imp

        #(#extra_types)*
    }
}

fn generate_properties_struct(block: &Block) -> TokenStream {
    let mut fields = vec![];

    for property in block.properties.values() {
        let name = &property.name;
        fields.push(quote! {
            #name: #property
        })
    }

    let name = &block.property_struct_name;

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct #name {
            #(
                #fields,
            )*
        }
    }
}

fn generate_properties_impl(block: &Block) -> TokenStream {
    let possible_states = generate_function_possible_states(block);
    let id_offset = generate_function_id_offset(block);
    let from_id_offset = generate_function_from_id_offset(block);
    let vanilla_id_offset = generate_function_vanilla_id_offset(block);

    let name = &block.property_struct_name;

    quote! {
        impl #name {
            #possible_states
            #id_offset
            #from_id_offset
            #vanilla_id_offset
        }
    }
}

fn generate_function_possible_states(block: &Block) -> TokenStream {
    let name = &block.property_struct_name;

    let body = if block.properties.is_empty() {
        quote! {
            vec![#name {}]
        }
    } else {
        // Output iterator magic.
        let mut iterators: Vec<_> = block
            .properties
            .values()
            .map(Property::tokens_for_value_iter)
            .collect();

        let first = iterators.remove(0);
        let mut body = quote! {
            #first
        };

        for iterator in &iterators {
            body.extend(quote! {
                .flat_map(|state| std::iter::repeat(state).zip(#iterator))
            });
        }

        let capture = if block.properties.len() == 1 {
            let first = &block.properties.values().next().unwrap().name;

            quote! { #first }
        } else {
            let mut iter = block.properties.values().map(|prop| &prop.name);
            let mut capture = format!("({}, {})", iter.next().unwrap(), iter.next().unwrap());

            for next in iter {
                capture = format!("({}, {})", capture, next);
            }

            TokenStream::from_str(&capture).unwrap()
        };

        let initializers: Vec<_> = block.properties.values().map(|prop| &prop.name).collect();

        body.extend(quote! {
            .map(|#capture| #name {
                #(#initializers,)*
            })
            .collect()
        });

        body
    };

    quote! {
        pub fn possible_states() -> Vec<Self> {
            #body
        }
    }
}

fn generate_function_id_offset(block: &Block) -> TokenStream {
    let body = if block.properties.is_empty() {
        quote! { 0 }
    } else {
        let possible_values: Vec<_> = block
            .properties
            .values()
            .map(|prop| prop.num_possible_values())
            .collect();

        let strides = find_property_strides(&possible_values);

        let mut res = TokenStream::new();
        for (i, (name, stride)) in block
            .properties
            .values()
            .map(|prop| &prop.name)
            .zip(strides)
            .enumerate()
        {
            res.extend(quote! {
                self.#name as u16 * #stride
            });

            if i != block.properties.len() - 1 {
                res.extend(quote! { + });
            }
        }

        res
    };

    quote! {
        pub fn id_offset(self) -> u16 {
            #body
        }
    }
}

fn generate_function_from_id_offset(block: &Block) -> TokenStream {
    let name = &block.property_struct_name;
    let body = if block.properties.is_empty() {
        quote! { Ok(#name {}) }
    } else {
        let possible_values: Vec<_> = block
            .properties
            .values()
            .map(|prop| prop.num_possible_values())
            .collect();

        let strides = find_property_strides(&possible_values);

        let from_u16: Vec<_> = block
            .properties
            .values()
            .map(|prop| {
                let name = &prop.name;
                prop.tokens_for_from_u16(quote! { #name })
            })
            .collect();

        let properties: Vec<_> = block.properties.values().map(|prop| &prop.name).collect();
        let initializers: Vec<_> = block.properties.values().map(|prop| &prop.name).collect();

        // Constant-time hackery. Don't try to figure out how this works.
        quote! {
            #(
                let #properties = offset / #strides;
                offset -= #properties * #strides;
                let #properties = #from_u16;
            )*

            Ok(Self {
                #(
                    #initializers,
                )*
            })
        }
    };

    quote! {
        pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
            #body
        }
    }
}

fn generate_function_vanilla_id_offset(block: &Block) -> TokenStream {
    let body = if block.properties.is_empty() {
        quote! { 0 }
    } else {
        let name = &block.property_struct_name;

        let num_ids = block.states.len() as u16;

        quote! {
            static MAP: Lazy<HashMap<#name, u16>> = Lazy::new(|| {
                #name::possible_states()
                    .into_iter()
                    .zip(0..#num_ids)
                    .collect()
            });

            *MAP.get(&self).unwrap()
        }
    };

    quote! {
        pub fn vanilla_id_offset(self) -> u16 {
            #body
        }
    }
}

fn find_property_strides(prop_possible_values: &[u16]) -> Vec<u16> {
    prop_possible_values
        .iter()
        .enumerate()
        .map(|(i, _)| {
            if i == prop_possible_values.len() - 1 {
                1
            } else {
                multiply_slice(&prop_possible_values[i + 1..])
            }
        })
        .collect()
}

fn multiply_slice(slice: &[u16]) -> u16 {
    let mut res = 1;
    slice.iter().copied().for_each(|x| res *= x);
    res
}
