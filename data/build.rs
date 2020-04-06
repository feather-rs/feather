use reqwest;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/minecraft"));
    dbg!(path);
    let path_server = path.join("server.jar");
    println!("cargo:rerun-if-changed={}", &path.display());

    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;

    donwload(&path_server)?;
    generate(&path)?;
    extract(&path)?;
    Ok(())
}

fn donwload<P: AsRef<Path>>(server: P) -> Result<(), Box<dyn Error>> {
    if File::open(&server).is_ok() {
        return Ok(());
    }

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
