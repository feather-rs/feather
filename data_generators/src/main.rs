use data_generators::extract_vanilla_data;

mod generators;
mod utils;

fn main() {
    extract_vanilla_data();

    println!("Generating code");
    generators::generate_all();

    println!("Done!");
}
