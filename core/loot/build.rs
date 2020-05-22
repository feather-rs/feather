use anyhow::Context;
use feather_loot_model::LootTable;
use std::io::{Read, Write};
use std::{env, fs::File};
use walkdir::WalkDir;

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
    let input = format!(
        "{}/minecraft-1.15/data/minecraft/loot_tables",
        feather_data::minecraft::PATH
    );

    let mut map = feather_loot_model::LootTableSet::default();

    for entry in WalkDir::new(&input) {
        let entry = entry.context("entry access failed")?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        // Determine path of file relative to the base directory
        let mut relative_path = entry
            .path()
            .strip_prefix(&input)
            .with_context(|| format!("failed to strip prefix for `{}`", entry.path().display()))?
            .to_str()
            .context("path contains invalid UTF-8")?;
        // strip .json suffix
        relative_path = &relative_path[..relative_path.len() - 5];

        // replace \\ with / for windows
        let relative_path = relative_path.replace("\\", "/");

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
    let dump_path = format!("{}/dump.json", env::var("OUT_DIR")?);
    let mut dump = File::create(&dump_path)?;
    let vec = serde_json::to_vec(&map).unwrap();
    dump.write_all(vec.as_slice())?;

    Ok(())
}
