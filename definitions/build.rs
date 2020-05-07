fn main() {
    let data_path = concat!(env!("CARGO_MANIFEST_DIR"), "/data");
    let target_path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated");

    if let Err(e) = feather_definitions_generator::load_directory(data_path, target_path) {
        panic!("{:?}", e);
    }

    println!("cargo:rerun-if-changed={}", data_path);
}
