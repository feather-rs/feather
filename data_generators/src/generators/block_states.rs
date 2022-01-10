use std::collections::BTreeMap;
use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;
use std::ops::RangeInclusive;
use std::str::FromStr;

use convert_case::{Case, Casing};
use indexmap::map::IndexMap;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use quote::{format_ident, quote};
use serde::ser::SerializeStruct;
use serde::Deserialize;
use serde::{Serialize, Serializer};

use crate::utils::{output, output_bytes};

pub fn generate() {
    let blocks = load();

    let table_src = generate_table(&blocks);
    let properties_src = generate_properties(&blocks);
    let block_fns_src = generate_block_fns(&blocks);

    output_bytes(
        "libcraft/blocks/src/generated/table.dat",
        bincode::serialize(&BlockTableSerialize::new(
            &blocks.blocks,
            &blocks.property_types,
        ))
        .unwrap(),
    );
    output_bytes(
        "libcraft/blocks/src/generated/vanilla_ids.dat",
        bincode::serialize(&VanillaStateIdSerialize::new(&blocks)).unwrap(),
    );

    output(
        "libcraft/blocks/src/generated/table.rs",
        table_src.to_string().as_str(),
    );
    output(
        "libcraft/blocks/src/generated/properties.rs",
        properties_src.to_string().as_str(),
    );
    output(
        "libcraft/blocks/src/generated/block_fns.rs",
        block_fns_src.to_string().as_str(),
    );
}

#[derive(Debug)]
struct Blocks {
    property_types: BTreeMap<String, Property>,
    blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    /// Lowercase name of this block, minecraft: prefix removed.
    name: Ident,
    /// `name.to_case(Case::UpperCamel)`
    name_camel_case: Ident,
    /// This block's properties.
    properties: Vec<String>,
    /// Default state and its property values.
    default_state: Vec<(String, String)>,
    /// Block states mapped to vanilla state IDs.
    ids: Vec<(Vec<(String, String)>, u16)>,
    /// Strides and offset coefficients for each property of this block.
    index_parameters: BTreeMap<String, (u16, u16)>,
}

#[derive(Debug)]
struct Property {
    /// Name of this property, with Rust keywords removed. (e.g. "type" => "kind")
    name: Ident,
    /// Actual name of this property before Feather renaming is applied.
    real_name: String,
    /// CamelCase name of this property if it were a struct or enum.
    ///
    /// Often prefixed with the name of the block to which this property belongs.
    _name_camel_case: Ident,
    /// The kind of this property.
    kind: PropertyKind,
    /// Possible values of this property.
    possible_values: Vec<String>,
}

impl Property {
    /// Returns the tokens to create an instance of this property from a `u16`.
    fn tokens_for_from_u16(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { range } => {
                let min = *range.start();
                quote! {{ #input as i32 + #min }}
            }
            PropertyKind::Boolean { .. } => quote! { if #input == 0 { false } else { true } },
            PropertyKind::Enum { name, .. } => {
                quote! { #name::try_from(#input).expect("invalid block state") }
            }
        }
    }

    fn tokens_for_to_u16(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { range } => {
                let min = *range.start() as u16;
                quote! {
                    #input as u16 - #min
                }
            }
            _ => quote! { #input as u16 },
        }
    }

    fn tokens_for_as_str(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { range } => {
                let nums = range.clone().collect::<Vec<_>>();
                let strs = range.clone().map(|x| x.to_string()).collect::<Vec<_>>();

                quote! {
                    match #input {
                        #(
                            #nums => #strs,
                        )*
                        _ => "unknown",
                    }
                }
            }
            PropertyKind::Boolean => quote! {
                match #input {
                    true => "true",
                    false => "false",
                }
            },
            PropertyKind::Enum { .. } => quote! { #input.as_str() },
        }
    }

    fn tokens_for_from_str(&self, input: TokenStream) -> TokenStream {
        match &self.kind {
            PropertyKind::Integer { range } => {
                let start = *range.start();
                let end = *range.end();
                quote! {
                    {
                        let x = i32::from_str(#input).ok()?;
                        if !(#start..=#end).contains(&x) {
                            return None;
                        }
                        x
                    }
                }
            }
            PropertyKind::Boolean => quote! {
                bool::from_str(#input).ok()?
            },
            PropertyKind::Enum { name, .. } => quote! { #name::from_str(#input).ok()?},
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
                let variant = format_ident!("{}", value.to_case(Case::UpperCamel));
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
                let as_str: Vec<_> = variants
                    .iter()
                    .map(|ident| ident.to_string())
                    .map(|x| x.to_case(Case::Snake))
                    .collect();

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

                    impl FromStr for #name {
                        type Err = anyhow::Error;

                        fn from_str(s: &str) -> anyhow::Result<Self> {
                            match s {
                                #(
                                    #as_str => Ok(#name::#variants),
                                )*
                                _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(#name))),
                            }
                        }
                    }

                    impl #name {
                        pub fn as_str(self) -> &'static str {
                            match self {
                                #(
                                    #name::#variants => #as_str,
                                )*
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
    pub block_fns: String,
    pub block_properties: String,
    pub block_table: String,
    pub block_table_serialized: Vec<u8>,
    pub vanilla_ids_serialized: Vec<u8>,
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

                let x = n_dimensional_index(state, offset_coefficient, stride);
                Some(#from_u16)
            }
        });

        let set = format_ident!("set_{}", name);
        let doc = format!("Updates the state value for the given block kind such that its `{}` value is updated. Returns the new state,
        or `None` if the block does not have this property.", name);
        let to_u16 = property.tokens_for_to_u16(quote! { value });
        fns.push(quote! {
            #[doc = #doc]
            pub fn #set(&self, kind: BlockKind, state: u16, value: #property) -> Option<u16> {
                let (offset_coefficient, stride) = self.#name[kind as u16 as usize];

                if offset_coefficient == 0 {
                    return None;
                }

                let old = n_dimensional_index(state, offset_coefficient, stride) as i32;
                let new = ({ #to_u16 } as i32 - old) * stride as i32 + state as i32;
                Some(new as u16)
            }
        });
    }

    quote! {
        use crate::*;
        use std::convert::TryFrom;
        use std::str::FromStr;
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

        let mut state_initializers = vec![];
        for (name, value) in default_state {
            let value_expr = blocks.property_types[name].expr_for_value(value);

            let name_fn = format_ident!("set_{}", name);
            state_initializers.push(quote! {
                block.#name_fn(#value_expr);
            });
        }

        let mut doc = format!(
            "Returns an instance of `{}` with default state values.",
            block.name
        );

        if !default_state.is_empty() {
            doc.push_str("\nThe default state values are as follows:\n");
        }

        for (name, value) in default_state {
            doc.push_str(&format!("* `{}`: {}\n", name, value));
        }

        fns.push(quote! {
            #[doc = #doc]
            pub fn #name() -> Self {
                let mut block = Self {
                    kind: BlockKind::#name_camel_case,
                    state: BlockKind::#name_camel_case.default_state_id() - BlockKind::#name_camel_case.min_state_id(),
                };
                #(#state_initializers)*
                block
            }
        })
    }

    for property in blocks.property_types.values() {
        let property_name = &property.name;
        let set = format_ident!("set_{}", property_name);
        let with = format_ident!("with_{}", property_name);

        let f = quote! {
            pub fn #property_name(self) -> Option<#property> {
                BLOCK_TABLE.#property_name(self.kind, self.state)
            }

            pub fn #set(&mut self, #property_name: #property) -> bool {
                match BLOCK_TABLE.#set(self.kind, self.state, #property_name) {
                    Some(new_state) => {
                        self.state = new_state;
                        true
                    }
                    None => false,
                }
            }

            pub fn #with(mut self, #property_name: #property) -> Self {
                self.#set(#property_name);
                self
            }
        };
        fns.push(f);
    }

    fns.extend(generate_block_serializing_fns(blocks));

    let res = quote! {
        use crate::*;
        use std::collections::BTreeMap;
        use std::str::FromStr;

        impl BlockId {
            #(#fns)*
        }
    };
    res
}

/// Generates `BlockId::identifier()`, `BlockId::to_properties_map()`, and `BlockId::from_properties_and_identifier()`.
fn generate_block_serializing_fns(blocks: &Blocks) -> Vec<TokenStream> {
    let mut fns = vec![];

    let mut identifier_fn_match_arms = vec![];
    for block in &blocks.blocks {
        let name_camel_case = &block.name_camel_case;

        let name = format!("minecraft:{}", block.name);

        identifier_fn_match_arms.push(quote! {
            BlockKind::#name_camel_case => #name
        });
    }

    fns.push(quote! {
        #[doc = "Returns the identifier of this block. For example, returns `minecraft::air` for an air block."]
        pub fn identifier(self) -> &'static str {
            match self.kind {
                #(#identifier_fn_match_arms,)*
            }
        }
    });

    let mut to_properties_map_fn_match_arms = vec![];
    let mut to_properties_map_util_fns = vec![];
    for block in &blocks.blocks {
        let name_camel_case = &block.name_camel_case;
        let fn_to_call = format_ident!("{}_to_properties_map", block.name);

        to_properties_map_fn_match_arms.push(quote! {
            BlockKind::#name_camel_case => self.#fn_to_call()
        });

        let mut inserts = vec![];
        for property_name in &block.properties {
            let property = &blocks.property_types[property_name];
            // Use the vanilla name of the property rather than our custom
            // mapping, to ensure world saves are compatible with vanilla.
            let property_real_name = &property.real_name;

            let name = &property.name;
            let as_str = property.tokens_for_as_str(quote! { #name });

            inserts.push(quote! {
                let #name = self.#name().unwrap();
                map.insert(#property_real_name, { #as_str });
            })
        }

        to_properties_map_util_fns.push(quote! {
            fn #fn_to_call(self) -> BTreeMap<&'static str, &'static str> {
                let mut map = BTreeMap::new();
                #(#inserts)*
                map
            }
        });
    }

    fns.push(quote! {
        #[doc = "Returns a mapping from property name to property value for this block. Used to serialize blocks in vanilla world saves."]
        pub fn to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
            match self.kind {
                #(#to_properties_map_fn_match_arms,)*
            }
        }

        #(#to_properties_map_util_fns)*
    });

    let mut from_identifier_and_properties_fn_match_arms = vec![];
    let mut from_identifier_and_properties_util_fns = vec![];
    for block in &blocks.blocks {
        let name = &block.name;
        let name_str = format!("minecraft:{}", name);
        let fn_to_call = format_ident!("{}_from_identifier_and_properties", block.name);

        from_identifier_and_properties_fn_match_arms.push(quote! {
            #name_str => Self::#fn_to_call(properties)
        });

        let mut retrievals = vec![];
        for property_name in &block.properties {
            let property = &blocks.property_types[property_name];
            let property_real_name = &property.real_name;

            let name = &property.name;
            let from_str = property.tokens_for_from_str(quote! { #name });
            let set_fn = format_ident!("set_{}", name);

            retrievals.push(quote! {
                let #name = map.get(#property_real_name)?;
                let #name = #from_str;
                block.#set_fn(#name);
            });
        }

        from_identifier_and_properties_util_fns.push(quote! {
            fn #fn_to_call(map: &BTreeMap<String, String>) -> Option<Self> {
                let mut block = BlockId::#name();
                #(#retrievals)*
                Some(block)
            }
        });
    }

    fns.push(quote! {
        #[doc = "Attempts to convert a block kind identifier (e.g. `minecraft::air`) and properties map to a `BlockId`."]
        pub fn from_identifier_and_properties(identifier: &str, properties: &BTreeMap<String, String>) -> Option<Self> {
            match identifier {
                #(#from_identifier_and_properties_fn_match_arms,)*
                _ => None,
            }
        }

       #(#from_identifier_and_properties_util_fns)*
    });

    let mut from_identifier_and_default_props_match_arms = vec![];
    for block in &blocks.blocks {
        let name_str = format!("minecraft:{}", block.name);
        let name = &block.name;

        from_identifier_and_default_props_match_arms.push(quote! {
            #name_str => Some(Self::#name())
        });
    }
    fns.push(quote! {
        #[doc = "Attempts to convert a block identifier to a block with default property values."]
        pub fn from_identifier(identifier: &str) -> Option<Self> {
            match identifier {
                #(#from_identifier_and_default_props_match_arms,)*
                _ => None,
            }
        }
    });

    fns
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

/// Serializable state ID table.
#[derive(Debug, Serialize)]
struct VanillaStateIdSerialize(Vec<Vec<u16>>); // indexed by [kind as u16 as usize][state as usize]

impl VanillaStateIdSerialize {
    pub fn new(blocks: &Blocks) -> Self {
        let mut ids: Vec<Vec<u16>> = std::iter::repeat_with(Vec::new)
            .take(blocks.blocks.len())
            .collect();

        for (i, block) in blocks.blocks.iter().enumerate() {
            for (state, id) in &block.ids {
                let mut internal_id = 0;

                for (property_name, property_value) in state {
                    let (offset_coefficient, stride) = block.index_parameters[property_name];

                    let index = blocks.property_types[property_name]
                        .possible_values
                        .iter()
                        .position(|val| val == property_value)
                        .unwrap();

                    let multiplier = internal_id / offset_coefficient;
                    let mut new = property_value_as_u16(
                        property_value,
                        index,
                        &blocks.property_types[property_name].kind,
                    ) * stride;
                    new += multiplier * offset_coefficient;
                    internal_id = new;
                }

                let internal_id = internal_id as usize;
                // pad with zeroes
                if internal_id >= ids[i].len() {
                    let to_extend = internal_id - ids[i].len() + 1;
                    ids[i].extend(std::iter::repeat(0).take(to_extend));
                }
                assert_eq!(ids[i][internal_id], 0, "failed for {}", block.name);
                ids[i][internal_id] = *id;
            }
        }

        Self(ids)
    }
}

fn property_value_as_u16(value: &str, index: usize, kind: &PropertyKind) -> u16 {
    let start = match kind {
        PropertyKind::Integer { range } => *range.start() as u16,
        _ => 0,
    };

    if let Ok(x) = i32::from_str(value) {
        x as u16 - start
    } else if let Ok(x) = bool::from_str(value) {
        x as u16
    } else {
        index as u16
    }
}

fn generate_properties(blocks: &Blocks) -> TokenStream {
    let mut fns = vec![];

    for property in blocks.property_types.values() {
        let name = &property.name;

        let doc = format!(
            "Determines whether or not a block has the `{}` property.",
            name
        );

        let kinds = blocks
            .blocks
            .iter()
            .filter(|block| block.default_state.iter().any(|(prop, _)| name == prop))
            .map(|block| {
                let name_camel_case = &block.name_camel_case;

                quote! { BlockKind::#name_camel_case }
            });

        let fn_name = format_ident!("has_{}", name);
        fns.push(quote! {
            #[doc = #doc]
            pub fn #fn_name(self) -> bool {
                match self.kind() {
                    #(#kinds)|* => true,
                    _ => false
                }
            }
        });
    }

    quote! {
        use crate::*;

        impl BlockId {
            #(#fns)*
        }
    }
}

/// Special property name overrides, to avoid names like "shape_neaaaassnn."
static NAME_OVERRIDES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from_iter([
        ("east_tf", "east_connected"),
        ("east_usn", "east_wire"),
        ("north_tf", "north_connected"),
        ("north_usn", "north_wire"),
        ("west_tf", "west_connected"),
        ("west_usn", "west_wire"),
        ("south_tf", "south_connected"),
        ("south_usn", "south_wire"),
        ("facing_dnswe", "facing_cardinal_and_down"),
        ("facing_neswud", "facing_cubic"),
        ("facing_nswe", "facing_cardinal"),
        ("half_ul", "half_upper_lower"),
        ("half_tb", "half_top_bottom"),
        ("kind_slr", "chest_kind"),
        ("kind_tbd", "slab_kind"),
        ("kind_ns", "piston_kind"),
        ("mode_cs", "comparator_mode"),
        ("mode_slcd", "structure_block_mode"),
        ("shape_neaaaa", "powered_rail_shape"),
        ("shape_siioo", "stairs_shape"),
        ("shape_neaaaassnn", "rail_shape"),
        ("level_0_3", "cauldron_level"),
        ("level_0_15", "water_level"),
        ("type_slr", "chest_kind"),
        ("type_tbd", "slab_kind"),
        ("type_ns", "piston_kind"),
    ])
});

#[derive(Debug, Deserialize)]
struct BlocksReport {
    #[serde(flatten)]
    blocks: IndexMap<String, BlockDefinition>,
}

#[derive(Debug, Deserialize)]
struct BlockDefinition {
    states: Vec<StateDefinition>,
    #[serde(default)]
    properties: BTreeMap<String, Vec<String>>, // map from property name => possible values
}

#[derive(Debug, Deserialize)]
struct StateDefinition {
    id: u16,
    #[serde(default)]
    default: bool,
    #[serde(default)]
    properties: BTreeMap<String, String>,
}

#[derive(Debug, Default, Clone)]
struct PropertyStore {
    /// Mapping from property name to the set of different sets
    /// of values known for this property.
    properties: BTreeMap<String, BTreeSet<Vec<String>>>,
}

impl PropertyStore {
    fn register(&mut self, property: String, possible_values: impl IntoIterator<Item = String>) {
        self.properties
            .entry(property)
            .or_default()
            .insert(possible_values.into_iter().collect());
    }

    fn finish(self) -> BTreeMap<String, Property> {
        let mut map = BTreeMap::new();

        for (name, possible_value_sets) in self.properties {
            let name = Self::update_name(&name);

            if possible_value_sets.len() == 1 {
                let possible_values = possible_value_sets.into_iter().next().unwrap();
                map.insert(
                    name.to_owned(),
                    Self::prop_from_possible_values_and_name(name, name, possible_values),
                );
            } else {
                // There are multiple variants of this property, each with their own set of values.
                // Create properties suffixed with an index to differentiate between these variants.
                for possible_values in possible_value_sets {
                    // Name is the name of the property followed by the first letter of each possible value.
                    // If it's an integer, it is the range of possible values.
                    let new_name = if possible_values[0].parse::<i32>().is_ok() {
                        let as_integer = possible_values
                            .iter()
                            .map(String::as_str)
                            .map(i32::from_str)
                            .map(Result::unwrap)
                            .collect::<Vec<_>>();

                        let min = *as_integer.iter().min().unwrap();
                        let max = *as_integer.iter().max().unwrap();

                        format!("{}_{}_{}", name, min, max)
                    } else {
                        let mut name = format!("{}_", name);
                        for value in &possible_values {
                            name.push(value.chars().next().unwrap().to_ascii_lowercase());
                        }
                        name
                    };

                    let new_name = Self::update_name(&new_name);

                    map.insert(
                        new_name.to_owned(),
                        Self::prop_from_possible_values_and_name(new_name, name, possible_values),
                    );
                }
            }
        }

        map
    }

    fn update_name(name: &str) -> &str {
        match NAME_OVERRIDES.get(&name) {
            Some(x) => *x,
            None => name,
        }
    }

    fn prop_from_possible_values_and_name(
        name: &str,
        real_name: &str,
        possible_values: Vec<String>,
    ) -> Property {
        Property {
            name: format_ident!("{}", name),
            real_name: real_name.to_owned(),
            _name_camel_case: format_ident!("{}", name.to_case(Case::UpperCamel)),
            kind: guess_property_kind(&possible_values, &name.to_case(Case::UpperCamel)),
            possible_values,
        }
    }
}

/// Parses the vanilla blocks report, returning a `Blocks`.
fn load() -> Blocks {
    let mut report =
        serde_json::from_str(&std::fs::read_to_string("generated/reports/blocks.json").unwrap())
            .unwrap();

    let mut blocks = vec![];
    let properties = fix_property_names(&mut report);

    for (identifier, block) in &report.blocks {
        if let Some(block) = load_block(identifier, block) {
            blocks.push(block);
        }
    }

    Blocks {
        blocks,
        property_types: properties.finish(),
    }
}

fn fix_property_names(report: &mut BlocksReport) -> PropertyStore {
    let mut store = PropertyStore::default();

    for block in report.blocks.values() {
        for (property_name, possible_values) in &block.properties {
            store.register(property_name.to_owned(), possible_values.clone());
        }
    }

    // Correct block property names
    let result = store.clone().finish();

    for block in report.blocks.values_mut() {
        let block: &mut BlockDefinition = block;
        let mut overrides = vec![];
        for (property_name, possible_values) in &mut block.properties {
            if result.get(property_name).is_none() {
                let name = if possible_values[0].parse::<i32>().is_ok() {
                    let as_integer = possible_values
                        .iter()
                        .map(String::as_str)
                        .map(i32::from_str)
                        .map(Result::unwrap)
                        .collect::<Vec<_>>();

                    let min = *as_integer.iter().min().unwrap();
                    let max = *as_integer.iter().max().unwrap();

                    format!("{}_{}_{}", property_name, min, max)
                } else {
                    let mut name = format!("{}_", property_name);
                    for value in possible_values {
                        name.push(value.chars().next().unwrap().to_ascii_lowercase());
                    }
                    name
                };
                let name = if let Some(name) = NAME_OVERRIDES.get(&name.as_str()) {
                    (*name).to_owned()
                } else {
                    name
                };

                overrides.push((property_name.to_owned(), name));
            }
        }

        for (old_name, new_name) in overrides {
            let old_values = block.properties.remove(&old_name).unwrap();
            block.properties.insert(new_name.clone(), old_values);

            for state in &mut block.states {
                let old_value = state.properties.remove(&old_name).unwrap();
                state.properties.insert(new_name.clone(), old_value);
            }
        }
    }

    store
}

fn load_block(identifier: &str, block: &BlockDefinition) -> Option<Block> {
    let identifier = identifier.strip_prefix("minecraft:").unwrap();

    let name_camel_case = identifier.to_case(Case::UpperCamel);

    let properties = load_block_properties(block);

    let index_parameters = load_block_index_parameters(block, &properties);

    let ids = load_block_ids(block);

    let default_state = block
        .states
        .iter()
        .find(|state| state.default)
        .map(|state| state.properties.clone())
        .unwrap_or_default()
        .into_iter()
        .collect();

    let block = Block {
        name: format_ident!("{}", identifier),
        name_camel_case: format_ident!("{}", name_camel_case),
        properties,
        ids,
        default_state,
        index_parameters,
    };

    Some(block)
}

fn load_block_properties(block: &BlockDefinition) -> Vec<String> {
    let mut props = vec![];

    for identifier in block.properties.keys() {
        props.push(identifier.to_owned());
    }

    props
}

fn load_block_index_parameters(
    block: &BlockDefinition,
    block_props: &[String],
) -> BTreeMap<String, (u16, u16)> {
    let mut map = BTreeMap::new();

    let possible_values = block_props
        .iter()
        .map(|block_prop| block.properties.get(block_prop).map(Vec::len).unwrap_or(0))
        .map(|x| x as u16)
        .collect::<Vec<_>>();

    for (i, block_prop) in block_props.iter().enumerate() {
        let stride = possible_values.iter().skip(i + 1).product::<u16>();
        let offset_coefficient = stride * possible_values[i];

        map.insert(block_prop.clone(), (offset_coefficient, stride));
    }

    map
}

fn load_block_ids(block: &BlockDefinition) -> Vec<(Vec<(String, String)>, u16)> {
    let mut res: Vec<(Vec<(String, String)>, u16)> = vec![];

    for state in &block.states {
        let properties = state.properties.clone().into_iter().collect();

        res.push((properties, state.id));
    }

    res
}

fn guess_property_kind(possible_values: &[String], property_struct_name: &str) -> PropertyKind {
    let first = &possible_values[0];

    if i32::from_str(first).is_ok() {
        // integer
        let as_integer: Vec<_> = possible_values
            .iter()
            .map(|x| i32::from_str(x).unwrap())
            .collect();

        let min = *as_integer.iter().min().unwrap();
        let max = *as_integer.iter().max().unwrap();

        PropertyKind::Integer { range: min..=max }
    } else if bool::from_str(first).is_ok() {
        // boolean
        PropertyKind::Boolean
    } else {
        // enum
        let name = format_ident!("{}", property_struct_name);
        let variants: Vec<_> = possible_values
            .iter()
            .map(|variant| variant.to_case(Case::UpperCamel))
            .map(|ident| format_ident!("{}", ident))
            .collect();
        PropertyKind::Enum { name, variants }
    }
}

#[derive(Serialize)]
struct BlockStatesData {
    block_table: BlockTableSerialize,
    state_ids: VanillaStateIdSerialize,
}
