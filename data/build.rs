use anyhow::Context;
use std::env;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => panic!("{:?}", e),
    }
}

fn run() -> anyhow::Result<()> {
    let path = format!("{}/minecraft", env::var("OUT_DIR").unwrap());
    let path = Path::new(&path);
    let path_server = path.join("server.jar");

    if data_exists(path).unwrap_or(false) {
        println!("cargo:rerun-if-changed={}", &path.display());
        return Ok(());
    }

    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).context("failed to create target directory for downloaded data")?;

    download(&path_server).context("failed to download vanilla server JAR")?;
    generate(&path).context(
        "failed to generate vanilla server reports. (is Java installed and in your PATH?)",
    )?;
    extract(&path).context("failed to extract vanilla assets. (are the Java developer tools (`jar`) installed and in your PATH?)")?;

    clone_minecraft_data().context("failed to clone PrismarineJS/minecraft-data")?;

    println!(
        "cargo:rerun-if-changed={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/build.rs")
    );
    Ok(())
}

fn data_exists(path: &Path) -> anyhow::Result<bool> {
    Ok(File::open(path.join("server.jar")).is_ok()
        && File::open(path.join("assets")).is_ok()
        && File::open(path.join("data")).is_ok()
        && File::open(path.join("generated")).is_ok())
}

fn download<P: AsRef<Path>>(server: P) -> anyhow::Result<()> {
    let mut response = reqwest::blocking::get("https://launcher.mojang.com/v1/objects/3737db93722a9e39eeada7c27e7aca28b144ffa7/server.jar")?;
    let mut dest = File::create(server)
        .context("failed to create destination file for server JAR download")?;
    copy(&mut response, &mut dest)?;
    Ok(())
}

fn generate<P: AsRef<Path>>(working: P) -> anyhow::Result<()> {
    let status = Command::new("java")
        .current_dir(working)
        .args(&["-cp", "server.jar", "net.minecraft.data.Main", "--reports"])
        .status()?;
    if !status.success() {
        anyhow::bail!(
            "process to generate server reports was not successful (exit status {})",
            status
        )
    }
    Ok(())
}

fn extract<P: AsRef<Path>>(working: P) -> anyhow::Result<()> {
    let status = Command::new("jar")
        .current_dir(working)
        .args(&["xf", "server.jar", "assets/", "data/"])
        .status()?;
    if !status.success() {
        anyhow::bail!(
            "JAR extraction process was not successful (exit status {})",
            status
        )
    }
    Ok(())
}

fn clone_minecraft_data() -> anyhow::Result<()> {
    let path = format!("{}/minecraft-data", env::var("OUT_DIR")?);
    if Path::new(&path).exists() {
        // Already cloned - no need to do so again
        return Ok(());
    }

    if !Command::new("git")
        .arg("clone")
        .arg("https://github.com/PrismarineJS/minecraft-data.git")
        .arg(&path)
        .status()?
        .success()
    {
        Err(anyhow::anyhow!(
            "failed to clone minecraft-data repository: please ensure git is installed"
        ))
    } else {
        Ok(())
    }
}
