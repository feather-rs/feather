//! Handles mapping from items to blocks and vice versa.
//! This functionality is used to perform block placements
//! and breaks.

use crate::item::ItemReport;
use crate::rust::{correct_variable_name, PropValueType};
use crate::{Block, BlockReport, State};
use failure::Error;
use heck::CamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::str::FromStr;
use syn::{Lit, LitBool};

/// Given a block report and an item report, generates
/// mappings from items to blocks and writes them to
/// the file with the given path.
pub fn generate_mappings(blocks: &str, items: &str, output_path: &str) -> Result<(), Error> {
    let blocks = {
        let mut file = File::open(blocks)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        serde_json::from_str(&string)?
    };

    let items = {
        let mut file = File::open(items)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        serde_json::from_str(&string)?
    };

    let mut output = File::create(output_path)?;

    _internal_generate_mappings(&blocks, &items, &mut output)?;

    Command::new("rustfmt").arg(output_path).output()?;

    Ok(())
}

fn _internal_generate_mappings(
    blocks: &BlockReport,
    items: &ItemReport,
    output: &mut File,
) -> Result<(), Error> {
    let mut match_arms = vec![];

    // Go through item report and find blocks with the same
    // name as the item.
    for (name, _) in &items.mappings {
        if let Some(match_arm) = block_state_by_name(&blocks, name.as_str()) {
            match_arms.push(match_arm);
        }
    }

    let result = quote! {
        use feather_items::Item;
        use feather_blocks::*;

        pub fn item_to_block(item: Item) -> Option<Block> {
            match item {
                #(#match_arms ,)*
                _ => None,
            }
        }
    };

    output.write_all(result.to_string().as_bytes())?;
    output.flush()?;

    Ok(())
}

fn block_state_by_name(blocks: &BlockReport, original_name: &str) -> Option<TokenStream> {
    let name = &original_name[10..];

    if let Some(block) = blocks.blocks.get(original_name) {
        // The block state corresponding to the item is labeled
        // in the report as "default."
        let state = default_state(&block);

        let camel_case = name.to_camel_case();
        let item_block_ident = Ident::new(&camel_case, Span::call_site());

        if block.states.len() == 1 {
            Some(quote! {
                Item::#item_block_ident => Some(Block::#item_block_ident)
            })
        } else {
            // Need to declare properties of block state
            let mut props = vec![];
            let state_props = state.properties.as_ref().unwrap(); // we know the block has properties, since it has multiple states

            for (prop_name, prop_value) in &state_props.props {
                let ty = PropValueType::guess_from_value(prop_value);

                let enum_name = format!("{}{}", name.to_camel_case(), prop_name.to_camel_case());
                let enum_name = Ident::new(&enum_name, Span::call_site());

                let field_name =
                    Ident::new(correct_variable_name(prop_name.as_str()), Span::call_site());

                let entry = if ty == PropValueType::Enum {
                    let variant = prop_value.to_camel_case();
                    let variant = Ident::new(&variant, Span::call_site());
                    quote! {
                        #field_name: #enum_name::#variant
                    }
                } else if ty == PropValueType::Bool {
                    let value = Lit::Bool(LitBool {
                        value: bool::from_str(prop_value).unwrap(),
                        span: Span::call_site(),
                    });
                    quote! {
                        #field_name: #value
                    }
                } else {
                    let value = i32::from_str(prop_value).unwrap();
                    quote! {
                        #field_name: #value
                    }
                };
                props.push(entry);
            }

            let data_struct_ident = format!("{}Data", name.to_camel_case());
            let data_struct_ident = Ident::new(&data_struct_ident, Span::call_site());

            Some(quote! {
                Item::#item_block_ident => Some(Block::#item_block_ident(#data_struct_ident {
                    #(#props ,)*
                }))
            })
        }
    } else {
        None
    }
}

fn default_state(block: &Block) -> State {
    block
        .states
        .iter()
        .find(|state| state.default)
        .unwrap()
        .clone()
}
