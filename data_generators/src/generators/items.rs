use convert_case::{Case, Casing};

use crate::utils::*;

pub fn generate() {
    let data: Vec<Item> = load_minecraft_json("items.json").unwrap();

    let mut out = generate_enum!(
        Item,
        data.par_iter()
            .map(|item| item.name.to_case(Case::UpperCamel))
            .collect::<Vec<_>>(),
        [serde::Serialize, serde::Deserialize],
        #[serde(try_from = "String", into = "&'static str")]
    );

    out.extend(generate_enum_property!(
        Item,
        "id",
        u32,
        data.iter()
            .map(|item| (
                item.name.to_case(Case::UpperCamel),
                item.id.to_string().parse().unwrap()
            ))
            .collect(),
        true
    ));
    out.extend(generate_enum_property!(
        Item,
        "name",
        &str,
        data.iter()
            .map(|item| (
                item.name.to_case(Case::UpperCamel),
                format!("\"{}\"", item.name).parse().unwrap()
            ))
            .collect(),
        true,
        &'static str
    ));
    out.extend(generate_enum_property!(
        Item,
        "namespaced_id",
        &str,
        data.iter()
            .map(|item| (
                item.name.to_case(Case::UpperCamel),
                format!("\"minecraft:{}\"", item.name).parse().unwrap()
            ))
            .collect(),
        true,
        &'static str
    ));
    out.extend(generate_enum_property!(
        Item,
        "stack_size",
        u32,
        data.iter()
            .map(|item| {
                (
                    item.name.to_case(Case::UpperCamel),
                    item.stack_size.to_string().parse().unwrap(),
                )
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        Item,
        "max_durability",
        Option<u32>,
        data.iter()
            .map(|item| {
                (
                    item.name.to_case(Case::UpperCamel),
                    item.max_durability
                        .map_or("None".parse().unwrap(), |durability| {
                            format!("Some({})", durability).parse().unwrap()
                        }),
                )
            })
            .collect(),
    ));
    out.extend(generate_enum_property!(
        Item,
        "fixed_with",
        Vec<&str>,
        data.iter()
            .map(|item| {
                (
                    item.name.to_case(Case::UpperCamel),
                    format!(
                        "vec![{}]",
                        item.fixed_with
                            .par_iter()
                            .flatten()
                            .map(|c| format!("\"{}\", ", c))
                            .collect::<String>()
                    )
                    .parse()
                    .unwrap(),
                )
            })
            .collect(),
    ));
    out.extend(quote! {
        use std::convert::TryFrom;
        use std::str::FromStr;

        impl TryFrom<String> for Item {
            type Error = &'static str;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                if let Some(item) = Item::from_name(value.as_str()) {
                    Ok(item)
                } else {
                    Err("Unknown item name")
                }
            }
        }

        impl From<Item> for &'static str {
            fn from(i: Item) -> Self {
                i.name()
            }
        }

        impl FromStr for Item {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some(item) = Item::from_name(s) {
                    Ok(item)
                } else {
                    Err("Unknown item name")
                }
            }
        }
    });

    output("libcraft/items/src/item.rs", out.to_string().as_str());
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Item {
    id: u32,
    name: String,
    #[allow(dead_code)]
    display_name: String,
    stack_size: i32,
    max_durability: Option<u32>,
    fixed_with: Option<Vec<String>>,
}
