mod common;
mod data;
mod generators;
mod utils;

use data::extract_vanilla_data;

fn main() -> anyhow::Result<()> {
    extract_vanilla_data();

    println!("Generating code");
    generators::generate_all()?;
    println!("Done!");

    Ok(())
}
