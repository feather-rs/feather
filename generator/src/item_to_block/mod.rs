//! Handles mapping from items to blocks and vice versa.
//! This functionality is used to perform block placements
//! and breaks.

use crate::item::ItemReport;
use crate::BlockReport;
use failure::Error;
use heck::CamelCase;
use proc_macro2::{Ident, Span};
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

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
    let mut item_to_block_match_arms = vec![];
    let mut block_to_item_match_arms = vec![];

    // Go through item report and find blocks with the same
    // name as the item.
    for (name, _) in &items.mappings {
        for block_name in blocks.blocks.keys() {
            if block_name == name {
                let block_ident = Ident::new(&block_name[10..], Span::call_site());
                let item_ident = Ident::new(&name[10..].to_camel_case(), Span::call_site());
                let block_ident_cc =
                    Ident::new(&block_ident.to_string().to_camel_case(), Span::call_site());

                item_to_block_match_arms.push(quote! {
                    Item::#item_ident => Some(BlockId::#block_ident())
                });
                block_to_item_match_arms.push(quote! {
                    BlockKind::#block_ident_cc => Some(Item::#item_ident)
                })
            }
        }
    }

    let result = quote! {
        use feather_items::Item;
        use feather_blocks::*;

        pub fn item_to_block(item: Item) -> Option<BlockId> {
            match item {
                #(#item_to_block_match_arms ,)*
                _ => None,
            }
        }

        pub fn block_to_item(block: BlockId) -> Option<Item> {
            match block.kind() {
                #(#block_to_item_match_arms ,)*
                _ => None,
            }
        }
    };

    output.write_all(result.to_string().as_bytes())?;
    output.flush()?;

    Ok(())
}
