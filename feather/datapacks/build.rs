use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    if !PathBuf::from("./minecraft").exists() {
        vanilla_assets::download_vanilla_assets(&PathBuf::from("../"))?;
    }
    Ok(())
}
