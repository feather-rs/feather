use std::{collections::HashMap, str::FromStr};

use crate::BlockKind;
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
    pub kind: BlockKind,
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
    pub rotation: Option<BlockFace>,
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

impl BlockReportEntry {
    fn properties<T: FromStr>(&self, name: &str) -> Vec<T>
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        if let Some(vec) = self.properties.get(name) {
            vec.iter().map(|s| T::from_str(s).ok()).flatten().collect()
        } else {
            Vec::new()
        }
    }
    pub fn to_raw_properties(&self, block_kind: BlockKind) -> RawBlockProperties {
        RawBlockProperties {
            kind: block_kind,
            valid_properties: ValidProperties {
                facing: self.properties("facing"),
                bamboo_leaves: self.properties("leaves"),
                age: self.properties("age"),
                stage: self.properties("stage"),
                rotation: self.properties("rotation"),
                open: self.properties("open"),
                occupied: self.properties("occupied"),
                part: self.properties("part"),
                honey_level: self.properties("honey_level"),
                bell_attachment: self.properties("attachment"),
                powered: self.properties("powered"),
                lit: self.properties("lit"),
                axis: self.properties("axis"),
                has_bottle_0: self.properties("has_bottle_0"),
                has_bottle_1: self.properties("has_bottle_1"),
                has_bottle_2: self.properties("has_bottle_2"),
                drag: self.properties("drag"),
                attached_face: self.properties("face"),
                signal_fire: self.properties("signal_fire"),
                waterlogged: self.properties("waterlogged"),
                bites: self.properties("bites"),
                level: self.properties("level"),
                chest_type: self.properties("type"),
                down: self.properties("down"),
                east: self.properties("east"),
                north: self.properties("north"),
                south: self.properties("south"),
                up: self.properties("up"),
                west: self.properties("west"),
                conditional: self.properties("conditional"),
                inverted: self.properties("inverted"),
                power: self.properties("power"),
                triggered: self.properties("triggered"),
                hinge: self.properties("hinge"),
                half: self.properties("half"),
                eye: self.properties("eye"),
                moisture: self.properties("moisture"),
                in_wall: self.properties("in_wall"),
                snowy: self.properties("snowy"),
                enabled: self.properties("enabled"),
                orientation: self.properties("orientation"),
                has_record: self.properties("has_record"),
                hanging: self.properties("hanging"),
                distance: self.properties("distance"),
                persistent: self.properties("persistent"),
                has_book: self.properties("has_book"),
                instrument: self.properties("instrument"),
                note: self.properties("note"),
                extended: self.properties("extended"),
                piston_type: self.properties("type"),
                short: self.properties("short"),
                rail_shape: self.properties("shape"),
                comparator_mode: self.properties("mode"),
                dust_east: self.properties("east"),
                dust_north: self.properties("north"),
                dust_south: self.properties("south"),
                dust_west: self.properties("west"),
                delay: self.properties("delay"),
                locked: self.properties("locked"),
                charges: self.properties("charges"),
                bottom: self.properties("bottom"),
                pickles: self.properties("pickles"),
                slab_type: self.properties("type"),
                layers: self.properties("layers"),
                stair_half: self.properties("half"),
                stair_shape: self.properties("shape"),
                structure_block_mode: self.properties("mode"),
                unstable: self.properties("unstable"),
                attached: self.properties("attached"),
                disarmed: self.properties("disarmed"),
                eggs: self.properties("eggs"),
                hatch: self.properties("hatch"),
                wall_east: self.properties("east"),
                wall_north: self.properties("north"),
                wall_south: self.properties("south"),
                wall_up: self.properties("up"),
                wall_west: self.properties("west"),
            },
        }
    }
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

    pub fn to_raw_state(&self, block_kind: BlockKind) -> RawBlockState {
        RawBlockState {
            id: self.id,
            kind: block_kind,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RawBlockProperties {
    pub kind: BlockKind,
    pub valid_properties: ValidProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ValidProperties {
    pub facing: Vec<BlockFace>,
    pub bamboo_leaves: Vec<BambooLeaves>,
    pub age: Vec<u8>,
    pub stage: Vec<u8>,
    pub rotation: Vec<BlockFace>,
    pub open: Vec<bool>,
    pub occupied: Vec<bool>,
    pub part: Vec<BedPart>,
    pub honey_level: Vec<u8>,
    pub bell_attachment: Vec<BellAttachment>,
    pub powered: Vec<bool>,
    pub lit: Vec<bool>,
    pub axis: Vec<Axis>,
    pub has_bottle_0: Vec<bool>,
    pub has_bottle_1: Vec<bool>,
    pub has_bottle_2: Vec<bool>,
    pub drag: Vec<u8>,
    pub attached_face: Vec<AttachedFace>,
    pub signal_fire: Vec<bool>,
    pub waterlogged: Vec<bool>,
    pub bites: Vec<u8>,
    pub level: Vec<u8>,
    pub chest_type: Vec<ChestType>,
    pub down: Vec<bool>,
    pub east: Vec<bool>,
    pub north: Vec<bool>,
    pub south: Vec<bool>,
    pub up: Vec<bool>,
    pub west: Vec<bool>,
    pub conditional: Vec<bool>,
    pub inverted: Vec<bool>,
    pub power: Vec<u8>,
    pub triggered: Vec<bool>,
    pub hinge: Vec<DoorHinge>,
    pub half: Vec<BlockHalf>,
    pub eye: Vec<bool>,
    pub moisture: Vec<u8>,
    pub in_wall: Vec<bool>,
    pub snowy: Vec<bool>,
    pub enabled: Vec<bool>,
    pub orientation: Vec<Orientation>,
    pub has_record: Vec<bool>,
    pub hanging: Vec<bool>,
    pub distance: Vec<u8>,
    pub persistent: Vec<bool>,
    pub has_book: Vec<bool>,
    pub instrument: Vec<Instrument>,
    pub note: Vec<u8>,
    pub extended: Vec<bool>,
    pub piston_type: Vec<PistonType>,
    pub short: Vec<bool>,
    pub rail_shape: Vec<RailShape>,
    pub comparator_mode: Vec<ComparatorMode>,
    pub dust_east: Vec<RedstoneConnection>,
    pub dust_north: Vec<RedstoneConnection>,
    pub dust_south: Vec<RedstoneConnection>,
    pub dust_west: Vec<RedstoneConnection>,
    pub delay: Vec<u8>,
    pub locked: Vec<bool>,
    pub charges: Vec<u8>,
    pub bottom: Vec<bool>,
    pub pickles: Vec<u8>,
    pub slab_type: Vec<SlabType>,
    pub layers: Vec<u8>,
    pub stair_half: Vec<StairHalf>,
    pub stair_shape: Vec<StairShape>,
    pub structure_block_mode: Vec<StructureBlockMode>,
    pub unstable: Vec<bool>,
    pub attached: Vec<bool>,
    pub disarmed: Vec<bool>,
    pub eggs: Vec<u8>,
    pub hatch: Vec<u8>,
    pub wall_east: Vec<WallConnection>,
    pub wall_north: Vec<WallConnection>,
    pub wall_south: Vec<WallConnection>,
    pub wall_up: Vec<WallConnection>,
    pub wall_west: Vec<WallConnection>,
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
