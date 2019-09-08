//! Generation of biome mappings from 1.14 registry report.

use failure::Error;
use heck::CamelCase;
use indexmap::IndexMap;
use proc_macro2::{Ident, Span};
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

#[derive(Deserialize, Clone)]
pub struct BiomeReport {
    #[serde(flatten)]
    biomes: IndexMap<String, Biome>,
}

#[derive(Deserialize, Clone)]
pub struct Biome {
    protocol_id: i32,
}

fn load_report(path: &str) -> Result<BiomeReport, Error> {
    let mut file = File::open(path)?;

    let json: Value = {
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        serde_json::from_str(&string)?
    };

    // Hack to get around the format of the registries.json
    // file.
    let biome_report: BiomeReport = {
        let top = &json["minecraft:biome"];
        let entries = &top["entries"];

        let as_string = serde_json::to_string(entries)?;
        serde_json::from_str(&as_string)?
    };

    Ok(biome_report)
}

pub fn generate_rust(input: &str, output: &str) -> Result<(), Error> {
    let report = load_report(input)?;

    let mut enum_variants = vec![];
    let mut to_protocol_id_match_arms = vec![];
    let mut from_protocol_id_match_arms = vec![];
    let mut to_identifier_match_arms = vec![];
    let mut from_identifier_match_arms = vec![];

    // These biomes don't exist in 1.13.2, only in 1.14.
    let exclude = ["minecraft:bamboo_jungle", "minecraft:bamboo_jungle_hills"];

    for (name, biome) in report.biomes {
        if exclude.iter().any(|e| e == &name) {
            continue;
        }
        let protocol_id = biome.protocol_id;

        let ident = Ident::new(&name[10..].to_camel_case(), Span::call_site());

        enum_variants.push(quote! {
            #ident,
        });

        to_protocol_id_match_arms.push(quote! {
            Biome::#ident => #protocol_id,
        });

        from_protocol_id_match_arms.push(quote! {
            #protocol_id => Some(Biome::#ident),
        });

        to_identifier_match_arms.push(quote! {
            Biome::#ident => #name,
        });

        from_identifier_match_arms.push(quote! {
            #name => Some(Biome::#ident),
        });
    }

    let code = quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount)]
        pub enum Biome {
            #(#enum_variants)*
        }

        impl Biome {
            pub fn protocol_id(self) -> i32 {
                match self {
                    #(#to_protocol_id_match_arms)*
                }
            }

            pub fn from_protocol_id(protocol_id: i32) -> Option<Self> {
                match protocol_id {
                    #(#from_protocol_id_match_arms)*
                    _ => None,
                }
            }

            pub fn identifier(self) -> &'static str {
                match self {
                    #(#to_identifier_match_arms)*
                }
            }

            pub fn from_identifier(identifier: &str) -> Option<Self> {
                match identifier {
                    #(#from_identifier_match_arms)*
                    _ => None,
                }
            }
        }
    };
    let mut file = File::create(output)?;
    file.write_all(code.to_string().as_bytes())?;

    Command::new("rustfmt").arg(output).output()?;

    Ok(())
}
