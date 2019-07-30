#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

mod blocks;
mod mappings;

use crate::mappings::NativeMappings;
pub use blocks::Block;
use std::collections::HashMap;
use std::hash::Hash;

const MAPPINGS_1_13_2: &'static [u8] = include_bytes!("../data/1.13.2.dat");
const MAPPINGS_1_14_4: &'static [u8] = include_bytes!("../data/1.14.4.dat");

const P1_13_2: u32 = 404;
const P1_14_4: u32 = 498;

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
    fn state_id(&self, proto_version: u32) -> u16;
}

impl BlockExt for Block {
    fn from_state_id(id: u16, proto_version: u32) -> Option<Self> {
        unimplemented!()
    }

    fn state_id(&self, proto_version: u32) -> u16 {
        let internal = self.internal_state_id();
        match proto_version {
            P1_13_2 => INTERNAL_TO_NATIVE[internal],
            _ => panic!("Invalid protocol version {}", proto_version),
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
