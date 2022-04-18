//! Generates the default `BiomeList`, serialized with `bincode`.

use std::path::PathBuf;

use anyhow::Context;
use libcraft_chunk::biome::{BiomeGeneratorInfo, BiomeList};

use crate::common::compress_and_write;

pub fn generate() -> anyhow::Result<()> {
    let mut biomes = BiomeList::default();

    let worldgen = PathBuf::from("worldgen");
    for dir in std::fs::read_dir(&worldgen)?.flatten() {
        let namespace = dir
            .file_name()
            .to_str()
            .context(format!(
                "Non-UTF8 characters in namespace directory: {:?}",
                dir.file_name()
            ))?
            .to_string();
        let namespace_dir = dir.path();
        let namespace_worldgen = namespace_dir.join("worldgen");
        for file in std::fs::read_dir(namespace_worldgen.join("biome"))?.flatten() {
            if let Some(file_name) = file.file_name().to_str() {
                if file_name.ends_with(".json") {
                    let biome: BiomeGeneratorInfo =
                        serde_json::from_str(&std::fs::read_to_string(file.path())?)?;
                    let name =
                        format!("{}:{}", namespace, file_name.strip_suffix(".json").unwrap());
                    println!("Loaded biome: {}", name);
                    biomes.insert(name, biome);
                }
            } else {
                // non-utf8 namespaces are errors, but non-utf8 values are just ignored
                println!(
                    "Ignoring a biome file with non-UTF8 characters in name: {:?}",
                    file.file_name()
                )
            }
        }
    }

    compress_and_write(biomes, "libcraft/assets/vanilla_biomes.bc.gz")?;
    Ok(())
}
