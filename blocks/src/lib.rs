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
pub use blocks::Block;
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
}
