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

    map.insert(
        String::from("Heightmaps"),
        Value::LongArray(level.heightmaps),
    );

    let sections = level.sections.into_iter().map(section_to_value).collect();
    map.insert(String::from("Sections"), Value::List(sections));

    map.insert(String::from("TileEntities"), Value::List(vec![])); // TODO
    map.insert(String::from("ToBeTicked"), Value::List(vec![])); // TODO

    let mut liquids_to_be_ticked = vec![];
    (0..16).for_each(|_| liquids_to_be_ticked.push(Value::List(vec![])));
    map.insert(
        String::from("LiquidsToBeTicked"),
        Value::List(liquids_to_be_ticked),
    );

    let mut tile_ticks = vec![];
    (0..16).for_each(|_| tile_ticks.push(Value::List(vec![])));
    map.insert(String::from("TileTicks"), Value::List(tile_ticks));

    let mut post_processing = vec![];
    (0..16).for_each(|_| post_processing.push(Value::List(vec![])));
    map.insert(String::from("PostProcessing"), Value::List(post_processing));

    map.insert(String::from("LiquidTicks"), Value::List(vec![]));

    let mut structures = HashMap::new();

    {
        let mut references = HashMap::new();
        references.insert(String::from("EndCity"), Value::LongArray(vec![]));
        references.insert(String::from("Fortress"), Value::LongArray(vec![]));
        references.insert(String::from("Monument"), Value::LongArray(vec![]));
        references.insert(String::from("Stronghold"), Value::LongArray(vec![]));
        references.insert(String::from("Swamp_Hut"), Value::LongArray(vec![]));

        structures.insert(String::from("References"), Value::Compound(references));
        structures.insert(String::from("Starts"), Value::Compound(HashMap::new()));
    }

    map.insert(
        String::from("Status"),
        Value::String(String::from("postprocessed")),
    );

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
        map.insert(String::from("Name"), Value::String(entry.name.into_owned()));

        if let Some(props) = entry.props {
            let mut props_map = HashMap::new();
            props.props.into_iter().for_each(|(name, value)| {
                props_map.insert(name, Value::String(value.into_owned()));
            });
            map.insert(
                String::from("Properties"),
                Value::Compound(
                    props_map
                        .into_iter()
                        .map(|(k, v)| (k.into_owned(), v))
                        .collect(),
                ),
            );
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
                heightmaps: vec![],
            },
        };

        let blob = chunk_root_to_blob(root);

        let mut buf = vec![];
        blob.to_writer(&mut buf).unwrap();

        let _: ChunkRoot = nbt::from_reader(Cursor::new(&buf)).unwrap();
    }
}
