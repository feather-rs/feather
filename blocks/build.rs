use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    match feather_blocks_generator::generate() {
        Ok(code) => {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated.rs");
            let mut file = File::create(path).unwrap();
            file.write_all(code.as_bytes()).unwrap();
            file.flush().unwrap();

            Command::new("rustfmt").arg(path).output().unwrap();

            println!(
                "cargo:rerun-if-changed={}",
                concat!(env!("CARGO_MANIFEST_DIR"), "/../data/minecraft")
            );
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            std::process::exit(1);
        }
    }
}
