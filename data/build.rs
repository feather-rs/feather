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
    let path_server = path.join("server.jar");
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path)?;
    donwload(&path_server)?;
    generate(&path)?;
    extract(&path)?;
    embed(&path)?;

    println!("cargo:rerun-if-changed={}", &path_server.display());
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

fn embed<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("minecraft.rs");

    let path = path.as_ref();

    let code = &["assets/minecraft", "data/minecraft", "generated/reports"]
        .iter()
        .map(|e| path.join(e))
        .map(|p| p.read_dir())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(fs::DirEntry::path)
        .filter(|p| p.is_dir())
        .map(|p| embeded_code(p, 0))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    fs::write(&dest_path, code)?;
    Ok(())
}

fn embeded_code<P: AsRef<Path>>(path: P, indent_n: usize) -> Result<String, Box<dyn Error>> {
    let path = path.as_ref();
    let name = path.file_stem().unwrap().to_str().unwrap();
    let indent: String = std::iter::repeat(" ").take(indent_n).collect();
    let code = if path.is_file() {
        let mut name = name.to_uppercase();
        if name.starts_with(char::is_numeric) {
            name = format!("N_{}", name);
        }
        if let Some("json") = path.extension().and_then(std::ffi::OsStr::to_str) {
            format!("{}pub const {}: &'static str = include_str!(\"{}\");", indent, name, path.display())
        } else {
            format!("{}pub const {}: &'static [u8] = include_bytes!(\"{}\");", indent, name, path.display())
        }
    } else {
        let nested = path
            .read_dir()?
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(fs::DirEntry::path)
            .map(|p| embeded_code(p, indent_n + 4))
            .collect::<Result<Vec<_>, _>>()?
            .join("\n");
        format!("{indent}pub mod {name} {{\n{code}\n{indent}}}", indent = indent, name = name.to_lowercase(), code = nested)
    };
    Ok(code)
}
