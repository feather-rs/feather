use fs_extra::dir::CopyOptions;
use std::{io::Read, path::PathBuf};

use libcraft_core::{SERVER_DOWNLOAD_URL, VERSION_STRING};

pub fn extract_vanilla_data() {
    const SERVER_JAR: &str = "server.jar";

    if std::fs::read_to_string("generated/.version").ok() != Some(VERSION_STRING.to_string()) {
        let _ = std::fs::remove_dir_all("generated");
        if !PathBuf::from(SERVER_JAR).is_file() {
            log::info!("Downloading Minecraft server jar");
            let mut buf = Vec::new();
            ureq::get(SERVER_DOWNLOAD_URL)
                .call()
                .unwrap()
                .into_reader()
                .read_to_end(&mut buf)
                .unwrap();
            std::fs::write(SERVER_JAR, &buf).unwrap();
        }

        log::info!("Running vanilla data generators");
        std::process::Command::new("java")
            .args(
                format!(
                    "-DbundlerMainClass=net.minecraft.data.Main -jar {} --all",
                    SERVER_JAR
                )
                .split_whitespace(),
            )
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        std::fs::write("generated/.version", VERSION_STRING).unwrap();
        std::fs::remove_file(SERVER_JAR).unwrap();
        std::fs::remove_dir_all("libraries").unwrap();
        std::fs::remove_dir_all("logs").unwrap();
        std::fs::remove_dir_all("versions").unwrap();

        log::info!("Copying ./generated/reports/worldgen/ to ./worldgen/");
        fs_extra::dir::create("worldgen", true).unwrap();
        fs_extra::dir::copy("generated/reports/worldgen/", "", &CopyOptions::default())
            .expect("Cannot copy ./generated/reports/worldgen/ to ./worldgen/");
    }
}
