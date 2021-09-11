use anyhow::Context;
use std::{
    fs::{self, File},
    io,
    path::Path,
    time::Duration,
};
use zip::ZipArchive;

// Taken from https://www.minecraft.net/en-us/download/server
const JAR_URL: &str =
    "https://launcher.mojang.com/v1/objects/c5f6fb23c3876461d46ec380421e42b289789530/server.jar";
const JAR_NAME: &str = "server-1.16.2.jar";

/// Downloads vanilla server/client JARs and assets, used
/// for loot tables, recipes, etc.
///
/// The server JAR will be placed in `$base/downloaded`.
/// The vanilla datapack will be extracted to `$base/datapacks`.
pub fn download_vanilla_assets(base: &Path) -> anyhow::Result<()> {
    let jar = download_jar(base)
        .context("failed to download vanilla server JAR")
        .context("please make sure you have an Internet connection.")?;
    // NB: JAR files are just glorified ZIP files, so we can use the zip crate
    // to process the data.
    let mut zip = ZipArchive::new(jar)?;

    create_minecraft_datapack(base, &mut zip)?;

    Ok(())
}

fn download_jar(base: &Path) -> anyhow::Result<File> {
    log::info!("Downloading vanilla server JAR from {}", JAR_URL);
    let mut data = ureq::get(JAR_URL)
        .timeout(Duration::from_secs(10))
        .call()?
        .into_reader();

    let downloaded_dir = base.join("downloaded");
    fs::create_dir_all(&downloaded_dir)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .truncate(true)
        .create(true)
        .open(downloaded_dir.join(JAR_NAME))?;

    io::copy(&mut data, &mut file)?;
    Ok(file)
}

fn create_minecraft_datapack(base: &Path, zip: &mut ZipArchive<File>) -> anyhow::Result<()> {
    log::info!("Extracting vanilla JAR");
    // We'll create a datapack by copying `pack.mcmeta` and the `data`
    // directory to a new datapack called `minecraft`.
    let target = base.join("datapacks/minecraft");
    fs::create_dir_all(&target)?;

    // copy pack.mcmeta
    {
        let mut pack_mcmeta = zip.by_name("pack.mcmeta")?;
        let mut pack_mcmeta_target = File::create(target.join("pack.mcmeta"))?;
        io::copy(&mut pack_mcmeta, &mut pack_mcmeta_target)?;
    }

    // copy data directory
    let mut files = Vec::new();
    for file_name in zip
        .file_names()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
    {
        let path = Path::new(&file_name);
        if path.starts_with("data") && zip.by_name(&file_name)?.is_file() {
            let path_in_target = target.join(path);
            files.push((file_name.to_owned(), path_in_target));
        }
    }
    for (file_name, path_in_target) in files {
        let mut reader = zip.by_name(&file_name)?;

        if let Some(parent) = path_in_target.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut writer = File::create(&path_in_target)?;

        io::copy(&mut reader, &mut writer)?;
        log::debug!("Extracted {}", file_name);
    }

    Ok(())
}
