use std::path::PathBuf;

fn main() {
    let mut data_path = PathBuf::new();
    data_path.push(env!("CARGO_MANIFEST_DIR"));
    let mut target_path = data_path.clone();

    data_path.push("data");
    target_path.push("src/generated");

    if let Err(e) = feather_definitions_generator::load_directory(
        &data_path,
        target_path.as_os_str().to_str().unwrap(),
    ) {
        panic!("{:?}", e);
    }

    println!("cargo:rerun-if-changed={}", data_path.display());
}
