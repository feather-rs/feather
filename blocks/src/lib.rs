//! The block system used by Feather is a rather
//! complex topic.
//!
//! There are three different sets of block state IDs,
//! native IDs, internal IDs, and versioned IDs.
//!
//! Native IDs are the block state IDs used by the Minecraft
//! version corresponding to Feather's "native" version, 1.13.2.
//! Native IDs are used across the codebase—for example, the `Chunk`
//! struct in `feather_core` uses native IDs to store blocks.
//!
//! "Internal" IDs are only used inside the `feather_blocks` crate.
//! The benefit of internal IDs is that they are calculated based on
//! the block's data in constant time - there is no need for any sort
//! of lookup. As a result, these internal IDs are used to find versioned and native IDs
//! in a vector. (The internal ID is used as an index into the vector, allowing
//! for efficient constant-time lookup).
//!
//! Versioned IDs mean any set of block state IDs used by Minecraft versions
//! other than the native version (1.13.2). For example, when a Chunk Data packet
//! is sent to a non-native client (e.g. a client on 1.14.4), the block state IDs
//! in the chunk need to be converted to 1.14.4 block state IDs, which may differ
//! from those in the native version. As a result, `feather_blocks` also provides
//! functions to efficiently convert native IDs to versioned IDs.

#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate num_derive;

#[allow(clippy::all)] // No, generated code isn't idiomatic. Too bad
mod blocks;
mod mappings;

use crate::mappings::NativeMappings;
pub use blocks::*;
use std::collections::HashMap;
use std::hash::Hash;

const MAPPINGS_1_13_2: &[u8] = include_bytes!("../data/1.13.2.dat");
//const MAPPINGS_1_14_4: &[u8] = include_bytes!("../data/1.14.4.dat");

const P1_13_2: u32 = 404;
//const P1_14_4: u32 = 498;

lazy_static! {
    static ref NATIVE_MAPPINGS: NativeMappings =
        { mappings::load_native(MAPPINGS_1_13_2).unwrap() };
    static ref INTERNAL_TO_NATIVE: Vec<u16> = { init_native_id_mappings(&NATIVE_MAPPINGS).0 };
    static ref NATIVE_TO_INTERNAL: Vec<u16> = { init_native_id_mappings(&NATIVE_MAPPINGS).1 };
}

pub trait BlockExt {
    fn from_state_id(id: u16, proto_version: u32) -> Option<Self>
    where
        Self: Sized;
    fn from_native_state_id(id: u16) -> Option<Self>
    where
        Self: Sized;
    fn state_id(&self, proto_version: u32) -> u16;

    fn native_state_id(&self) -> u16;

    /// Returns whether this block is "solid."
    fn is_solid(&self) -> bool;

    /// Returns the light level emitted by this block.
    fn light_emission(&self) -> u8;
}

impl BlockExt for Block {
    fn from_state_id(_id: u16, _proto_version: u32) -> Option<Self> {
        unimplemented!()
    }

    fn from_native_state_id(id: u16) -> Option<Self> {
        if id as usize >= NATIVE_TO_INTERNAL.len() {
            return None;
        }

        let internal = NATIVE_TO_INTERNAL[id as usize];
        Block::from_internal_state_id(internal as usize)
    }

    fn state_id(&self, proto_version: u32) -> u16 {
        let internal = self.internal_state_id();
        match proto_version {
            P1_13_2 => INTERNAL_TO_NATIVE[internal],
            _ => panic!("Invalid protocol version {}", proto_version),
        }
    }

    fn native_state_id(&self) -> u16 {
        let internal = self.internal_state_id();
        INTERNAL_TO_NATIVE[internal]
    }

    fn is_solid(&self) -> bool {
        // TODO: there are likely a few missing in this list
        match self {
            Block::Air
            | Block::OakSapling(_)
            | Block::SpruceSapling(_)
            | Block::BirchSapling(_)
            | Block::JungleSapling(_)
            | Block::AcaciaSapling(_)
            | Block::DarkOakSapling(_)
            | Block::Water(_)
            | Block::Lava(_)
            | Block::Grass
            | Block::Fern
            | Block::DeadBush
            | Block::Seagrass
            | Block::TallSeagrass(_)
            | Block::Dandelion
            | Block::Poppy
            | Block::BlueOrchid
            | Block::Allium
            | Block::AzureBluet
            | Block::RedTulip
            | Block::OrangeTulip
            | Block::WhiteTulip
            | Block::PinkTulip
            | Block::OxeyeDaisy
            | Block::BrownMushroom
            | Block::RedMushroom
            | Block::Torch
            | Block::WallTorch(_)
            | Block::Fire(_)
            | Block::Wheat(_)
            | Block::Sign(_)
            | Block::Ladder(_)
            | Block::Rail(_)
            | Block::WallSign(_)
            | Block::Lever(_)
            | Block::StonePressurePlate(_)
            | Block::OakPressurePlate(_)
            | Block::SprucePressurePlate(_)
            | Block::BirchPressurePlate(_)
            | Block::JunglePressurePlate(_)
            | Block::AcaciaPressurePlate(_)
            | Block::DarkOakPressurePlate(_)
            | Block::RedstoneTorch(_)
            | Block::RedstoneWallTorch(_)
            | Block::StoneButton(_)
            | Block::Snow(_)
            | Block::SugarCane(_)
            | Block::Repeater(_)
            | Block::AttachedMelonStem(_)
            | Block::AttachedPumpkinStem(_)
            | Block::MelonStem(_)
            | Block::PumpkinStem(_)
            | Block::Vine(_)
            | Block::Carrots(_)
            | Block::Potatoes(_)
            | Block::OakButton(_)
            | Block::SpruceButton(_)
            | Block::BirchButton(_)
            | Block::JungleButton(_)
            | Block::AcaciaButton(_)
            | Block::DarkOakButton(_)
            | Block::LightWeightedPressurePlate(_)
            | Block::HeavyWeightedPressurePlate(_)
            | Block::Comparator(_)
            | Block::WhiteCarpet
            | Block::OrangeCarpet
            | Block::MagentaCarpet
            | Block::LightBlueCarpet
            | Block::YellowCarpet
            | Block::LimeCarpet
            | Block::PinkCarpet
            | Block::GrayCarpet
            | Block::LightGrayCarpet
            | Block::CyanCarpet
            | Block::PurpleCarpet
            | Block::BlueCarpet
            | Block::BrownCarpet
            | Block::GreenCarpet
            | Block::RedCarpet
            | Block::BlackCarpet
            | Block::Sunflower(_)
            | Block::Lilac(_)
            | Block::RoseBush(_)
            | Block::Peony(_)
            | Block::TallGrass(_)
            | Block::LargeFern(_)
            | Block::Kelp(_)
            | Block::KelpPlant
            | Block::DriedKelpBlock
            | Block::VoidAir
            | Block::CaveAir => false,
            _ => true,
        }
    }

    fn light_emission(&self) -> u8 {
        match self {
            Block::Beacon
            | Block::EndGateway
            | Block::EndPortal
            | Block::Fire(_)
            | Block::Glowstone
            | Block::JackOLantern(_)
            | Block::Lava(_)
            | Block::RedstoneLamp(RedstoneLampData { lit: true })
            | Block::SeaLantern
            | Block::SeaPickle(SeaPickleData {
                waterlogged: true,
                pickles: 4,
            })
            | Block::Conduit(_) => 15,
            Block::EndRod(_) | Block::Torch => 14,
            Block::Furnace(_) => 13,
            Block::SeaPickle(SeaPickleData {
                waterlogged: true,
                pickles: 3,
            }) => 12,
            Block::NetherPortal(_) => 11,
            Block::SeaPickle(SeaPickleData {
                waterlogged: true,
                pickles: 2,
            }) => 9,
            Block::EnderChest(_) | Block::RedstoneTorch(_) => 7,
            Block::SeaPickle(SeaPickleData {
                waterlogged: true,
                pickles: 1,
            }) => 6,
            Block::MagmaBlock => 3,
            Block::BrewingStand(_)
            | Block::BrownMushroom
            | Block::DragonEgg
            | Block::EndPortalFrame(_) => 1,
            _ => 0,
        }
    }
}

/// Creates the internal ID -> native ID
/// mappings vector, where indices into the
/// vector are internal IDs and the values in the vector
/// are native IDs.
///
/// Also creates the opposite vector - native IDs -> internal IDs.
fn init_native_id_mappings(mappings: &NativeMappings) -> (Vec<u16>, Vec<u16>) {
    let mut internal_to_native = vec![1; mappings.blocks.len()];
    let mut native_to_internal = vec![1; mappings.blocks.len()];

    for ((name, props), native_id) in mappings.blocks.clone() {
        let block = Block::from_name_and_props(&name, &vec_to_hash_map(props)).unwrap();
        let internal_id = block.internal_state_id() as u16;

        // Confirm we're not overwriting
        assert_eq!(internal_to_native[internal_id as usize], 1);
        assert_eq!(native_to_internal[native_id as usize], 1);

        internal_to_native[internal_id as usize] = native_id;
        native_to_internal[native_id as usize] = internal_id;
    }

    (internal_to_native, native_to_internal)
}

fn vec_to_hash_map<K, V>(vec: Vec<(K, V)>) -> HashMap<K, V>
where
    K: Eq + Hash,
{
    let mut result = HashMap::new();
    for entry in vec {
        result.insert(entry.0, entry.1);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::GrassBlockData;

    #[test]
    fn test_native_state_id() {
        let block = Block::Stone;
        assert_eq!(block.native_state_id(), 1);

        let block = Block::GrassBlock(GrassBlockData { snowy: true });
        assert_eq!(block.native_state_id(), 8);
    }

    #[test]
    fn test_lots_of_blocks() {
        for id in 0..8595 {
            let block = Block::from_native_state_id(id).unwrap();
            assert_eq!(block.native_state_id(), id);
        }
    }

    #[test]
    fn test_default_props() {
        assert_eq!(
            Block::from_name_and_default_props("minecraft:grass_block").unwrap(),
            Block::GrassBlock(GrassBlockData { snowy: false })
        );
    }

    #[test]
    fn test_to_name_and_props() {
        let block = Block::GrassBlock(GrassBlockData { snowy: true });

        let (name, props) = block.to_name_and_props();

        assert_eq!(name, "minecraft:grass_block");
        assert_eq!(props.len(), 1);
        let (prop_name, prop_value) = props.first().unwrap();

        assert_eq!(*prop_name, "snowy");
        assert_eq!(prop_value, "true");
    }
}
