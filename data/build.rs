use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::io::copy;
use std::process::Command;
use reqwest;

fn main() -> Result<(), Box<dyn Error>> {
    let path_version = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/data/1.13.2"));
    let path_server = path_version.join("server.jar");
    
    println!("cargo:rerun-if-changed={}", path_server.display());
    
    // Create folder data/{version}
    fs::create_dir_all(path_version)?;
    
    donwload(path_server)?;
    generate(path_version)?;
    extract(path_version)?;

    Ok(())
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
        .args(&["-cp", "server.jar", "net.minecraft.data.Main", "--all"])
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