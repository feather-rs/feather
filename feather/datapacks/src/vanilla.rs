use anyhow::Context;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::SystemTime;
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
const DOWNLOAD_WARNING_MINUTES: f64 = 2.0;
const DOWNLOAD_TIMEOUT_MINUTES: u64 = 30;

/// Downloads vanilla server/client JARs and assets, used
/// for loot tables, recipes, etc.
///
/// The server JAR will be placed in `$base/downloaded`.
/// The vanilla datapack will be extracted to `$base/datapacks`.
pub fn download_vanilla_assets(base: &Path) -> anyhow::Result<()> {
    let jar = download_jar(base)
        .context(format!(
            "failed to download vanilla server JAR in {} minutes",
            DOWNLOAD_TIMEOUT_MINUTES
        ))
        .context("please make sure you have an Internet connection.")?;

    // NB: JAR files are just glorified ZIP files, so we can use the zip crate
    // to process the data.
    let mut zip = ZipArchive::new(jar)?;

    create_minecraft_datapack(base, &mut zip)?;

    Ok(())
}

fn download_jar(base: &Path) -> anyhow::Result<File> {
    log::info!("Downloading vanilla server JAR from {}", JAR_URL);

    let downloaded = Arc::new(AtomicBool::new(false));
    let download_start_time = SystemTime::now();
    let mut download_time = 1.0;
    let mut warning = 1;
    let downloaded1 = downloaded.clone();
    std::thread::spawn(move || {
        while !downloaded1.load(Ordering::Relaxed) {
            let elapsed = download_start_time.elapsed().unwrap().as_secs_f64();
            if elapsed / 60.0 >= warning as f64 * DOWNLOAD_WARNING_MINUTES
                && download_time / 60.0 < warning as f64 * DOWNLOAD_WARNING_MINUTES
            {
                log::warn!("Looks like you have a slow internet connection! Downloading vanilla server JAR for {} minutes", warning as f64 * DOWNLOAD_WARNING_MINUTES);
                warning += 1;
            }
            download_time = elapsed;
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    let mut data = ureq::get(JAR_URL)
        .timeout(Duration::from_secs(60 * DOWNLOAD_TIMEOUT_MINUTES))
        .call()
        .map_err(|err| {
            // stop download time counter
            downloaded.store(true, Ordering::Relaxed);
            err
        })?
        .into_reader();
    downloaded.store(true, Ordering::Relaxed);
    log::info!("Downloaded vanilla server JAR successfully");

    let downloaded_dir = base.join("downloaded");
    fs::create_dir_all(&downloaded_dir)?;
    let mut file = File::create(downloaded_dir.join(JAR_NAME))?;

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
    for file_name in zip.file_names() {
        let path = Path::new(file_name);
        let path_in_target = fs::canonicalize(target.join(path))?;
        if path.starts_with("data") && path_in_target.starts_with(&target) {
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
