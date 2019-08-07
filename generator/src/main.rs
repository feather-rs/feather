//! This program is used to generate a bunch of code, including block state ID mappings
//! and corresponding Rust code. It reads from vanilla block.json
//! files. See `block_format.md` for more information.

#![forbid(unsafe_code, warnings)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate quote;

mod block_data;
mod item;
mod rust;
mod util;

pub use block_data::{
    Block, BlockProperties, BlockReport, State, StateProperties, DEFAULT_STATE_ID,
};
use byteorder::{LittleEndian, WriteBytesExt};
use clap::App;
use failure::Error;
use heck::CamelCase;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::{BufReader, Write};
use std::process::exit;
use std::str::FromStr;
use syn::export::Span;
use syn::Ident;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    if let Err(e) = run() {
        error!("An error occurred: {}", e);
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("block-mappings") => {
            let args = matches.subcommand_matches("block-mappings").unwrap();
            block_data::generate_mappings_file(
                args.value_of("input").unwrap(),
                args.value_of("output").unwrap(),
                args.value_of("native").unwrap(),
                u32::from_str(args.value_of("proto").unwrap())?,
                args.value_of("ver").unwrap(),
            )?;
        }
        Some("native-block-mappings") => {
            let args = matches.subcommand_matches("native-block-mappings").unwrap();
            block_data::generate_native_mappings_file(
                args.value_of("input").unwrap(),
                args.value_of("output").unwrap(),
                u32::from_str(args.value_of("proto").unwrap())?,
                args.value_of("ver").unwrap(),
            )?;
        }
        Some("block-rust") => {
            let args = matches.subcommand_matches("block-rust").unwrap();
            rust::generate_rust_code(
                args.value_of("input").unwrap(),
                args.value_of("output").unwrap(),
            )?;
        }
        Some("item-mappings") => {
            let args = matches.subcommand_matches("item-mappings").unwrap();
            item::generate_mappings_file(
                args.value_of("input").unwrap(),
                args.value_of("output").unwrap(),
            )?;
        }
        Some("item-rust") => {
            let args = matches.subcommand_matches("item-rust").unwrap();
            item::generate_rust(
                args.value_of("input").unwrap(),
                args.value_of("output").unwrap(),
            )?;
        }
        Some(s) => {
            error!("Invalid subcommand {}", s);
            return Ok(());
        }
        None => {
            error!("No subcommand specified");
            return Ok(());
        }
    }

    Ok(())
}

pub trait WriteExt {
    fn write_string(&mut self, x: &str) -> std::io::Result<()>;
}

impl<W: Write> WriteExt for W {
    fn write_string(&mut self, x: &str) -> std::io::Result<()> {
        self.write_u32::<LittleEndian>(x.len() as u32)?;
        self.write_all(x.as_bytes())?;

        Ok(())
    }
}
