use feather_core::loot_table::LootTable;
use glob::glob;
use std::fs::File;

#[test]
fn test_vanilla_loot_tables() {
    let glob = glob(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/loot_tables/**/*.json"));
    for entry in glob.unwrap() {
        let path = entry.unwrap();
        let file = File::open(&path).unwrap();
        match LootTable::from_reader(file) {
            Ok(_) => {},
            Err(err) => {
                panic!("Failed for {:?} with {:?}", path, err);
            }
        }
        
    }
}