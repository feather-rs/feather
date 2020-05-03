use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    match feather_blocks_generator::generate() {
        Ok(code) => {
            let base = concat!(env!("CARGO_MANIFEST_DIR"), "/src/generated");

            let _ = std::fs::create_dir_all(base);

            let kind = format!("{}/kind.rs", base);
            let block_fns = format!("{}/block_fns.rs", base);
            let table = format!("{}/table.rs", base);

            write_to_file(&kind, &code.kind);
            write_to_file(&block_fns, &code.block_fns);
            write_to_file(&table, &code.block_table);

            [kind, block_fns, table].iter().for_each(|path| {
                Command::new("rustfmt").arg(path).output().unwrap();
            });

            let data = format!("{}/table.dat", base);
            File::create(&data)
                .unwrap()
                .write_all(&code.block_table_serialized)
                .unwrap();

            let data = format!("{}/vanilla_ids.dat", base);
            File::create(&data)
                .unwrap()
                .write_all(&code.vanilla_ids_serialized)
                .unwrap();

            println!(
                "cargo:rerun-if-changed={}",
                concat!(env!("CARGO_MANIFEST_DIR"), "/../../data")
            );
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            std::process::exit(1);
        }
    }
}

fn write_to_file(path: impl AsRef<str>, s: impl AsRef<str>) {
    File::create(path.as_ref())
        .unwrap()
        .write_all(s.as_ref().as_bytes())
        .unwrap();
}
