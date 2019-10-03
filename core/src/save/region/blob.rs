//! Conversion of `ChunkRoot` to `nbt::Blob`.
//! This is required due to https://github.com/PistonDevelopers/hematite_nbt/issues/27.

use super::ChunkLevel;
use super::{ChunkRoot, LevelSection};
use nbt::{Blob, Value};
use std::collections::HashMap;

pub fn chunk_root_to_blob(root: ChunkRoot) -> Blob {
    let mut blob = Blob::new();
    blob.insert("DataVersion", root.data_version).unwrap();

    blob.insert("Level", level_to_value(root.level)).unwrap();

    blob
}

fn level_to_value(level: ChunkLevel) -> Value {
    let mut map = HashMap::new();

    map.insert(String::from("xPos"), Value::Int(level.x_pos));
    map.insert(String::from("zPos"), Value::Int(level.z_pos));
    map.insert(String::from("LastUpdate"), Value::Long(0)); // TODO
    map.insert(String::from("InhabitedTime"), Value::Long(0)); // TODO
    map.insert(String::from("Biomes"), Value::IntArray(level.biomes));

    let mut hmaps = HashMap::new();
    hmaps.insert(
        String::from("MOTION_BLOCKING"),
        Value::LongArray(vec![0; 32]),
    ); // TODO
    hmaps.insert(
        String::from("MOTION_BLOCKING_NO_LEAVES"),
        Value::LongArray(vec![0; 32]),
    ); // TODO
    hmaps.insert(String::from("OCEAN_FLOOR"), Value::LongArray(vec![0; 32])); // TODO
    hmaps.insert(
        String::from("OCEAN_FLOOR_WG"),
        Value::LongArray(vec![0; 32]),
    ); // TODO
    hmaps.insert(String::from("WORLD_SURFACE"), Value::LongArray(vec![0; 32])); // TODO
    hmaps.insert(
        String::from("WORLD_SURFACE_WG"),
        Value::LongArray(vec![0; 32]),
    ); // TODO
    map.insert(String::from("Heightmaps"), Value::Compound(hmaps));

    let sections = level.sections.into_iter().map(section_to_value).collect();
    map.insert(String::from("Sections"), Value::List(sections));

    map.insert(String::from("TileEntities"), Value::List(vec![])); // TODO
    map.insert(String::from("TileTicks"), Value::List(vec![])); // TODO
    map.insert(String::from("ToBeTicked"), Value::List(vec![])); // TODO

    // Entities
    let mut entities = Vec::with_capacity(level.entities.len());
    level.entities.into_iter().for_each(|entity| {
        entities.push(entity.into_nbt_value());
    });

    map.insert(String::from("Entities"), Value::List(entities));

    Value::Compound(map)
}

fn section_to_value(section: LevelSection) -> Value {
    let mut map = HashMap::new();

    map.insert(String::from("Y"), Value::Byte(section.y));
    map.insert(
        String::from("BlockLight"),
        Value::ByteArray(section.block_light),
    );
    map.insert(
        String::from("SkyLight"),
        Value::ByteArray(section.sky_light),
    );
    map.insert(
        String::from("BlockStates"),
        Value::LongArray(section.states),
    );

    let mut entries = vec![];
    for entry in section.palette {
        let mut map = HashMap::new();
        map.insert(String::from("Name"), Value::String(entry.name));

        if let Some(props) = entry.props {
            let mut props_map = HashMap::new();
            props.props.into_iter().for_each(|(name, value)| {
                props_map.insert(name, Value::String(value));
            });
            map.insert(String::from("Properties"), Value::Compound(props_map));
        }

        entries.push(Value::Compound(map))
    }

    map.insert(String::from("Palette"), Value::List(entries));

    Value::Compound(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::region::DATA_VERSION;
    use std::io::Cursor;

    #[test]
    fn test_to_blob_roundtrip() {
        let root = ChunkRoot {
            data_version: DATA_VERSION,
            level: ChunkLevel {
                x_pos: 0,
                z_pos: 0,
                sections: vec![LevelSection {
                    y: 0,
                    states: vec![0],
                    palette: vec![],
                    block_light: vec![0],
                    sky_light: vec![0],
                }],
                biomes: vec![10],
                entities: vec![],
            },
        };

        let blob = chunk_root_to_blob(root);

        let mut buf = vec![];
        blob.to_writer(&mut buf).unwrap();

        let _: ChunkRoot = nbt::from_reader(Cursor::new(&buf)).unwrap();
    }
}
