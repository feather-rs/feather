use convert_case::{Case, Casing};
use indexmap::IndexMap;

use crate::utils::*;

pub fn generate() {
    let item_names_by_id = load_minecraft_json::<Vec<IdAndName>>("items.json")
        .unwrap()
        .into_iter()
        .map(|IdAndName { id, name }| (id, name))
        .collect::<IndexMap<_, _>>();
    let material_dig_multipliers: IndexMap<String, IndexMap<u32, f32>> =
        load_minecraft_json("materials.json").unwrap();
    let blocks: Vec<Block> = load_minecraft_json("blocks.json").unwrap();

    let mut material_constant_refs = IndexMap::new();

    let mut dig_multiplier_constants = Vec::new();
    for (name, dig_multipliers) in material_dig_multipliers {
        let multipliers = dig_multipliers
            .into_iter()
            .map(|(item, multiplier)| {
                let item_name =
                    format_ident!("{}", item_names_by_id[&item].to_case(Case::UpperCamel));
                quote! { (libcraft_items::Item::#item_name, #multiplier) }
            })
            .collect::<Vec<_>>();

        let constant = format_ident!(
            "{}",
            name.to_ascii_uppercase()
                .replace(';', "_AND_")
                .replace('/', "_WITH_")
        );
        dig_multiplier_constants.push(quote! {
            pub const #constant: &[(libcraft_items::Item, f32)] = &[#(#multipliers),*];
        });
        material_constant_refs.insert(name, quote! { #constant });
    }
    let mut out = quote! {
        #[allow(dead_code)]
        pub mod dig_multipliers {
            #(#dig_multiplier_constants)*
        }
    };

    out.extend(generate_enum!(
        BlockKind,
        blocks
            .iter ()
            .map(|block| block.name.to_case(Case::UpperCamel))
            .collect::<Vec<_>>(),
        [
            num_derive::FromPrimitive,
            num_derive::ToPrimitive,
            serde::Serialize,
            serde::Deserialize,
            bincode::Encode,
            bincode::Decode
        ],
        #[serde(rename_all = "snake_case")]
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "id",
        u32,
        blocks
            .iter()
            .map(|block| (block.name.to_case(Case::UpperCamel), {
                let id = block.id;
                quote! { #id }
            }))
            .collect(),
        true,
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "name",
        &str,
        blocks
            .iter()
            .map(|block| (block.name.to_case(Case::UpperCamel), {
                let name = &block.name;
                quote! { #name }
            }))
            .collect(),
        true,
        &'static str
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "namespaced_id",
        &str,
        blocks
            .iter()
            .map(|block| (block.name.to_case(Case::UpperCamel), {
                let name = format!("minecraft:{}", block.name);
                quote! { #name }
            }))
            .collect(),
        true,
        &'static str
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "resistance",
        f32,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let resistance = block.resistance;
                    quote! { #resistance }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "hardness",
        f32,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let hardness = block.hardness.unwrap_or(-1.0);
                    quote! { #hardness }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "stack_size",
        u32,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let stack_size = block.stack_size;
                    quote! { #stack_size }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "diggable",
        bool,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let diggable = block.diggable;
                    quote! { #diggable }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "transparent",
        bool,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let transparent = block.transparent;
                    quote! { #transparent }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "default_state_id",
        u16,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let default_state = block.default_state;
                    quote! { #default_state }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "min_state_id",
        u16,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let min_state_id = block.min_state_id;
                    quote! { #min_state_id }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "max_state_id",
        u16,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let max_state_id = block.max_state_id;
                    quote! { #max_state_id }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "light_emission",
        u8,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let emit_light = block.emit_light;
                    quote! { #emit_light }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "light_filter",
        u8,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let filter_light = block.filter_light;
                    quote! { #filter_light }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "solid",
        bool,
        blocks
            .iter()
            .map(|block| {
                (block.name.to_case(Case::UpperCamel), {
                    let solid = block.bounding_box == "block";
                    quote! { #solid }
                })
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "dig_multipliers",
        &'static [(libcraft_items::Item, f32)],
        blocks
            .iter()
            .map(|block| {
                (
                    block.name.to_case(Case::UpperCamel),
                    material_constant_refs
                        .get(&block.name)
                        .cloned()
                        .unwrap_or_else(|| quote! { &[] }),
                )
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "harvest_tools",
        Option<&'static [libcraft_items::Item]>,
        blocks
            .iter()
            .map(|block| {
                (
                    block.name.to_case(Case::UpperCamel),
                    if let Some(harvest_tools) = block.harvest_tools.as_ref() {
                        let items = harvest_tools
                            .keys()
                            .map(|item| {
                                format_ident!(
                                    "{}",
                                    item_names_by_id[&item.parse::<u32>().unwrap()]
                                        .to_case(Case::UpperCamel)
                                )
                            })
                            .collect::<Vec<_>>();
                        quote! {
                            Some(&[#(libcraft_items::Item::#items),*])
                        }
                    } else {
                        quote! { None }
                    },
                )
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        BlockKind,
        "drops",
        &'static [libcraft_items::Item],
        blocks
            .iter()
            .map(|block| {
                let items = block
                    .drops
                    .iter()
                    .map(|item| {
                        format_ident!("{}", item_names_by_id[item].to_case(Case::UpperCamel))
                    })
                    .collect::<Vec<_>>();
                (
                    block.name.to_case(Case::UpperCamel),
                    quote! {
                        &[#(libcraft_items::Item::#items),*]
                    },
                )
            })
            .collect()
    ));

    output("libcraft/blocks/src/block.rs", out.to_string().as_str());
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Block {
    id: u32,
    name: String,
    #[allow(dead_code)]
    display_name: String,
    hardness: Option<f32>,
    resistance: f32,
    stack_size: u32,
    diggable: bool,
    #[allow(dead_code)]
    material: String,
    transparent: bool,
    emit_light: u8,
    filter_light: u8,
    default_state: u16,
    min_state_id: u16,
    max_state_id: u16,
    #[allow(dead_code)]
    states: Vec<State>,
    harvest_tools: Option<IndexMap<String, bool>>,
    drops: Vec<u32>,
    bounding_box: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct State {
    name: String,
    r#type: StateType,
    num_values: usize,
    values: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
enum StateType {
    Bool,
    Int,
    Enum,
}
