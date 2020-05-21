use anyhow::Context;
use feather_loot_model::LootTable;
use std::{env, fs::File};
use walkdir::WalkDir;
use std::io::Read;

fn main() {
    if let Err(e) = run() {
        panic!("{:?}", e);
    }

    println!(
        "cargo:rerun-if-changed={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/build.rs")
    );
}

fn run() -> anyhow::Result<()> {
    // Load in all loot tables, then dump them into ${OUT_DIR}/dump.ron
    // for inclusion in `feather-loot`.
    let input = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../data/minecraft/data/minecraft/loot_tables"
    );

    let mut map = feather_loot_model::LootTableSet::default();

    for entry in WalkDir::new(input) {
        let entry = entry.context("entry access failed")?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        // Determine path of file relative to the base directory
        let relative_path = entry
            .path()
            .strip_prefix(input)
            .with_context(|| format!("failed to strip prefix for `{}`", entry.path().display()))?
            .to_str()
            .context("path contains invalid UTF-8")?;

        let mut s = String::new();
        let mut file = File::open(entry.path())?;
        file.read_to_string(&mut s)?;

        let table = serde_json::from_str::<Option<LootTable>>(&s)
            .with_context(|| format!("failed to parse loot table `{}`", relative_path))?;

        if let Some(table) = table {
            map.0.insert(relative_path.into(), table);
        }
    }

    // Write the loot table map out to the dump
    let dump_path = format!("{}/dump.cbor", env::var("OUT_DIR")?);
    let mut dump = File::create(&dump_path)?;
    serde_cbor::to_writer(&mut dump, &map)?;

    Ok(())
}
