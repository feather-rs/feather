use std::path::PathBuf;

use anyhow::{bail, Context};
use libcraft::dimension::DimensionInfo;

pub fn generate() -> anyhow::Result<()> {
    let mut dimensions = Vec::new();
    let worldgen = PathBuf::from("worldgen");
    for namespace in std::fs::read_dir(&worldgen)?.flatten() {
        let namespace_path = namespace.path();
        for file in std::fs::read_dir(namespace_path.join("dimension"))?.flatten() {
            if file.path().is_dir() {
                bail!(
                    "worldgen/{}/dimension/ shouldn't contain directories",
                    file.file_name().to_str().unwrap_or("<non-UTF8 characters>")
                )
            }
            let mut dimension_info: DimensionInfo =
                serde_json::from_str(&std::fs::read_to_string(file.path())?)?;

            let (dimension_namespace, dimension_value) =
                dimension_info.r#type.split_once(':').context(format!(
                    "Invalid dimension type `{}`. It should contain `:` once",
                    dimension_info.r#type
                ))?;
            if dimension_value.contains(':') {
                bail!(
                    "Invalid dimension type `{}`. It should contain `:` exactly once",
                    dimension_info.r#type
                );
            }
            let mut dimension_type_path = worldgen.join(dimension_namespace);
            dimension_type_path.push("dimension_type");
            dimension_type_path.push(format!("{}.json", dimension_value));
            dimension_info.info = serde_json::from_str(&std::fs::read_to_string(
                dimension_type_path,
            )?)
            .context(format!(
                "Invalid dimension type format (worldgen/{}/dimension_type/{}.json",
                dimension_namespace, dimension_value
            ))?;
            dimensions.push(dimension_info);
        }
    }

    crate::common::compress_and_write(dimensions, "libcraft/assets/vanilla_dimensions.bc.gz")?;

    Ok(())
}
