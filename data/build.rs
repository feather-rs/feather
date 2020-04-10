use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/minecraft"));
    let path_server = path.join("server.jar");

    if data_exists(path).unwrap_or(false) {
        println!("cargo:rerun-if-changed={}", &path.display());
        return Ok(());
    }

    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;

    donwload(&path_server)?;
    generate(&path)?;
    extract(&path)?;

    println!("cargo:rerun-if-changed={}", &path.display());
    Ok(())
}

fn data_exists(path: &Path) -> Result<bool, Box<dyn Error>> {
    Ok(File::open(path.join("server.jar")).is_ok()
        && File::open(path.join("assets")).is_ok()
        && File::open(path.join("data")).is_ok()
        && File::open(path.join("generated")).is_ok())
}

fn donwload<P: AsRef<Path>>(server: P) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get("https://launcher.mojang.com/v1/objects/3737db93722a9e39eeada7c27e7aca28b144ffa7/server.jar")?;
    let mut dest = File::create(server)?;
    copy(&mut response, &mut dest)?;
    Ok(())
}

fn generate<P: AsRef<Path>>(working: P) -> Result<(), Box<dyn Error>> {
    let status = Command::new("java")
        .current_dir(working)
        .args(&["-cp", "server.jar", "net.minecraft.data.Main", "--reports"])
        .status()?;
    if !status.success() {
        panic!("Failed to generate data from server jar.")
    }
    Ok(())
}

fn extract<P: AsRef<Path>>(working: P) -> Result<(), Box<dyn Error>> {
    let status = Command::new("jar")
        .current_dir(working)
        .args(&["xf", "server.jar", "assets/", "data/"])
        .status()?;
    if !status.success() {
        panic!("Failed to generate data from server jar.")
    }
    Ok(())
}
