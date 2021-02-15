use std::{collections::HashMap, str::FromStr};

use libcraft_core::block::{
    AttachedFace, Axis, BambooLeaves, BedPart, BellAttachment, BlockFace, BlockHalf, ChestType,
    ComparatorMode, DoorHinge, Instrument, Orientation, PistonType, RailShape, RedstoneConnection,
    SlabType, StairHalf, StairShape, StructureBlockMode, WallConnection,
};
use serde::{Deserialize, Serialize};

/// Defines all possible data associated with a block state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RawBlockState {
    /// Block state ID
    pub id: u16,
    /// Whether this is the default state for this block kind
    pub default: bool,
    pub properties: RawBlockStateProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RawBlockStateProperties {
    pub facing: Option<BlockFace>,
    pub bamboo_leaves: Option<BambooLeaves>,
    pub age: Option<u8>,
    pub stage: Option<u8>,
    pub rotation: Option<u32>,
    pub open: Option<bool>,
    pub occupied: Option<bool>,
    pub part: Option<BedPart>,
    pub honey_level: Option<u8>,
    pub bell_attachment: Option<BellAttachment>,
    pub powered: Option<bool>,
    pub lit: Option<bool>,
    pub axis: Option<Axis>,
    pub has_bottle_0: Option<bool>,
    pub has_bottle_1: Option<bool>,
    pub has_bottle_2: Option<bool>,
    pub drag: Option<u8>,
    pub attached_face: Option<AttachedFace>,
    pub signal_fire: Option<bool>,
    pub waterlogged: Option<bool>,
    pub bites: Option<u8>,
    pub level: Option<u8>,
    pub chest_type: Option<ChestType>,
    pub down: Option<bool>,
    pub east: Option<bool>,
    pub north: Option<bool>,
    pub south: Option<bool>,
    pub up: Option<bool>,
    pub west: Option<bool>,
    pub conditional: Option<bool>,
    pub inverted: Option<bool>,
    pub power: Option<u8>,
    pub triggered: Option<bool>,
    pub hinge: Option<DoorHinge>,
    pub half: Option<BlockHalf>,
    pub eye: Option<bool>,
    pub moisture: Option<u8>,
    pub in_wall: Option<bool>,
    pub snowy: Option<bool>,
    pub enabled: Option<bool>,
    pub orientation: Option<Orientation>,
    pub has_record: Option<bool>,
    pub hanging: Option<bool>,
    pub distance: Option<u8>,
    pub persistent: Option<bool>,
    pub has_book: Option<bool>,
    pub instrument: Option<Instrument>,
    pub note: Option<u8>,
    pub extended: Option<bool>,
    pub piston_type: Option<PistonType>,
    pub short: Option<bool>,
    pub rail_shape: Option<RailShape>,
    pub comparator_mode: Option<ComparatorMode>,
    pub dust_east: Option<RedstoneConnection>,
    pub dust_north: Option<RedstoneConnection>,
    pub dust_south: Option<RedstoneConnection>,
    pub dust_west: Option<RedstoneConnection>,
    pub delay: Option<u8>,
    pub locked: Option<bool>,
    pub charges: Option<u8>,
    pub bottom: Option<bool>,
    pub pickles: Option<u8>,
    pub slab_type: Option<SlabType>,
    pub layers: Option<u8>,
    pub stair_half: Option<StairHalf>,
    pub stair_shape: Option<StairShape>,
    pub structure_block_mode: Option<StructureBlockMode>,
    pub unstable: Option<bool>,
    pub attached: Option<bool>,
    pub disarmed: Option<bool>,
    pub eggs: Option<u8>,
    pub hatch: Option<u8>,
    pub wall_east: Option<WallConnection>,
    pub wall_north: Option<WallConnection>,
    pub wall_south: Option<WallConnection>,
    pub wall_up: Option<WallConnection>,
    pub wall_west: Option<WallConnection>,
}

/// The Minecraft data report read from
/// `blocks.json`.
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockReport {
    #[serde(flatten)]
    pub blocks: HashMap<String, BlockReportEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockReportEntry {
    pub states: Vec<BlockReportState>,
    #[serde(default)]
    pub properties: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockReportState {
    #[serde(default)]
    pub properties: HashMap<String, String>,
    pub id: u16,
    #[serde(default)]
    pub default: bool,
}

impl BlockReportState {
    fn property<T: FromStr>(&self, name: &str) -> Option<T> {
        let s = self.properties.get(name)?;
        T::from_str(s).ok()
    }

    pub fn to_raw_state(&self) -> RawBlockState {
        RawBlockState {
            id: self.id,
            default: self.default,
            properties: RawBlockStateProperties {
                facing: self.property("facing"),
                bamboo_leaves: self.property("leaves"),
                age: self.property("age"),
                stage: self.property("stage"),
                rotation: self.property("rotation"),
                open: self.property("open"),
                occupied: self.property("occupied"),
                part: self.property("part"),
                honey_level: self.property("honey_level"),
                bell_attachment: self.property("attachment"),
                powered: self.property("powered"),
                lit: self.property("lit"),
                axis: self.property("axis"),
                has_bottle_0: self.property("has_bottle_0"),
                has_bottle_1: self.property("has_bottle_1"),
                has_bottle_2: self.property("has_bottle_2"),
                drag: self.property("drag"),
                attached_face: self.property("face"),
                signal_fire: self.property("signal_fire"),
                waterlogged: self.property("waterlogged"),
                bites: self.property("bites"),
                level: self.property("level"),
                chest_type: self.property("type"),
                down: self.property("down"),
                east: self.property("east"),
                north: self.property("north"),
                south: self.property("south"),
                up: self.property("up"),
                west: self.property("west"),
                conditional: self.property("conditional"),
                inverted: self.property("inverted"),
                power: self.property("power"),
                triggered: self.property("triggered"),
                hinge: self.property("hinge"),
                half: self.property("half"),
                eye: self.property("eye"),
                moisture: self.property("moisture"),
                in_wall: self.property("in_wall"),
                snowy: self.property("snowy"),
                enabled: self.property("enabled"),
                orientation: self.property("orientation"),
                has_record: self.property("has_record"),
                hanging: self.property("hanging"),
                distance: self.property("distance"),
                persistent: self.property("persistent"),
                has_book: self.property("has_book"),
                instrument: self.property("instrument"),
                note: self.property("note"),
                extended: self.property("extended"),
                piston_type: self.property("type"),
                short: self.property("short"),
                rail_shape: self.property("shape"),
                comparator_mode: self.property("mode"),
                dust_east: self.property("east"),
                dust_north: self.property("north"),
                dust_south: self.property("south"),
                dust_west: self.property("west"),
                delay: self.property("delay"),
                locked: self.property("locked"),
                charges: self.property("charges"),
                bottom: self.property("bottom"),
                pickles: self.property("pickles"),
                slab_type: self.property("type"),
                layers: self.property("layers"),
                stair_half: self.property("half"),
                stair_shape: self.property("shape"),
                structure_block_mode: self.property("mode"),
                unstable: self.property("unstable"),
                attached: self.property("attached"),
                disarmed: self.property("disarmed"),
                eggs: self.property("eggs"),
                hatch: self.property("hatch"),
                wall_east: self.property("east"),
                wall_north: self.property("north"),
                wall_south: self.property("south"),
                wall_up: self.property("up"),
                wall_west: self.property("west"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn block_sizes() {
        println!("Raw block state size: {} bytes", size_of::<RawBlockState>());
    }
}
