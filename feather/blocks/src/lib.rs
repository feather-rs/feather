pub use libcraft_blocks::{BlockKind, SimplifiedBlockKind};
use num_traits::FromPrimitive;
use std::convert::TryFrom;
use thiserror::Error;

pub mod categories;
mod directions;
#[allow(warnings)]
#[allow(clippy::all)]
mod generated;
mod wall_blocks;

static BLOCK_TABLE: Lazy<BlockTable> = Lazy::new(|| {
    let bytes = include_bytes!("generated/table.dat");
    bincode::deserialize(bytes).expect("failed to deserialize generated block table (bincode)")
});

static VANILLA_ID_TABLE: Lazy<Vec<Vec<u16>>> = Lazy::new(|| {
    let bytes = include_bytes!("generated/vanilla_ids.dat");
    bincode::deserialize(bytes).expect("failed to deserialize generated vanilla ID table (bincode)")
});

pub const HIGHEST_ID: u16 = 17111;

static FROM_VANILLA_ID_TABLE: Lazy<Vec<BlockId>> = Lazy::new(|| {
    let mut res = vec![BlockId::default(); u16::max_value() as usize];

    for (kind_id, ids) in VANILLA_ID_TABLE.iter().enumerate() {
        let kind = BlockKind::from_u16(kind_id as u16).expect("invalid block kind ID");

        for (state, id) in ids.iter().enumerate() {
            res[*id as usize] = BlockId {
                state: state as u16,
                kind,
            };
        }
    }

    debug_assert!((1..=HIGHEST_ID).all(|id| res[id as usize] != BlockId::default()));
    // Verify distinction
    if cfg!(debug_assertions) {
        let mut known_blocks = HashSet::with_capacity(HIGHEST_ID as usize);
        assert!((1..=HIGHEST_ID).all(|id| known_blocks.insert(res[id as usize])));
    }

    res
});

/// Can be called at startup to pre-initialize the global block table.
pub fn init() {
    Lazy::force(&FROM_VANILLA_ID_TABLE);
    Lazy::force(&BLOCK_TABLE);
}

use once_cell::sync::Lazy;

pub use crate::generated::table::*;

use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockId {
    kind: BlockKind,
    state: u16,
}

impl Default for BlockId {
    fn default() -> Self {
        BlockId {
            kind: BlockKind::Air,
            state: 0,
        }
    }
}

impl BlockId {
    /// Returns the kind of this block.
    pub fn kind(self) -> BlockKind {
        self.kind
    }

    /// Returns the simplified kind of this block.
    /// This is an arbitrary manual mapping that aims to condense the different
    /// vanilla block kinds which have only minor differences (e.g. different colored beds)
    /// and is mainly intended to make `match`ing on the block type easier.
    /// This mapping in no way stable right now.
    pub fn simplified_kind(self) -> SimplifiedBlockKind {
        self.kind.simplified_kind()
    }

    /// Returns the vanilla state ID for this block.
    pub fn vanilla_id(self) -> u16 {
        VANILLA_ID_TABLE[self.kind as u16 as usize][self.state as usize]
    }

    /*
    /// Returns the vanilla fluid ID for this block in case it is a fluid.
    /// The fluid ID is used in the Tags packet.
    pub fn vanilla_fluid_id(self) -> Option<u16> {
        if self.is_fluid() {
            match (self.kind(), self.water_level().unwrap()) {
                // could be swapped?
                (BlockKind::Water, 0) => Some(2), // stationary water
                (BlockKind::Water, _) => Some(1), // flowing water
                // tested those
                (BlockKind::Lava, 0) => Some(4), // stationary lava
                (BlockKind::Lava, _) => Some(3), // flowing lava
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
    */

    /// Returns the block corresponding to the given vanilla ID.
    ///
    /// (Invalid IDs currently return `BlockId::air()`).
    pub fn from_vanilla_id(id: u16) -> Self {
        FROM_VANILLA_ID_TABLE[id as usize]
    }
}

impl From<BlockId> for u32 {
    fn from(id: BlockId) -> Self {
        ((id.kind as u32) << 16) | id.state as u32
    }
}

#[derive(Debug, Error)]
pub enum BlockIdFromU32Error {
    #[error("invalid block kind ID {0}")]
    InvalidKind(u16),
    #[error("invalid block state ID {0} for kind {1:?}")]
    InvalidState(u16, BlockKind),
}

impl TryFrom<u32> for BlockId {
    type Error = BlockIdFromU32Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let kind_id = (value >> 16) as u16;
        let kind = BlockKind::from_u16(kind_id).ok_or(BlockIdFromU32Error::InvalidKind(kind_id))?;

        let state = (value | ((1 << 16) - 1)) as u16;

        // TODO: verify state
        Ok(BlockId { kind, state })
    }
}

// This is where the magic happens.
pub(crate) fn n_dimensional_index(state: u16, offset_coefficient: u16, stride: u16) -> u16 {
    (state % offset_coefficient) / stride
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instrument() {
        let mut block = BlockId {
            kind: BlockKind::NoteBlock,
            state: 0,
        };
        assert!(block.instrument().is_some());

        block.set_instrument(Instrument::Basedrum);
        assert_eq!(block.instrument(), Some(Instrument::Basedrum));
    }

    #[test]
    fn highest_id() {
        assert_eq!(
            HIGHEST_ID,
            *VANILLA_ID_TABLE.last().unwrap().last().unwrap()
        )
    }

    #[test]
    fn vanilla_ids() {
        let block = BlockId::rose_bush().with_half_upper_lower(HalfUpperLower::Lower);

        assert_eq!(block.vanilla_id(), 7894); // will have to be changed whenever we update to a newer MC version
        assert_eq!(BlockId::from_vanilla_id(block.vanilla_id()), block);

        let block =
            BlockId::structure_block().with_structure_block_mode(StructureBlockMode::Corner);

        assert_eq!(block.vanilla_id(), 15745);
        assert_eq!(BlockId::from_vanilla_id(block.vanilla_id()), block);

        let mut block = BlockId::redstone_wire();
        block.set_power(2);
        block.set_south_wire(SouthWire::Side);
        block.set_west_wire(WestWire::Side);
        block.set_east_wire(EastWire::Side);
        block.set_north_wire(NorthWire::Up);

        assert_eq!(block.power(), Some(2));
        assert_eq!(block.south_wire(), Some(SouthWire::Side));
        assert_eq!(block.west_wire(), Some(WestWire::Side));
        assert_eq!(block.east_wire(), Some(EastWire::Side));
        assert_eq!(block.north_wire(), Some(NorthWire::Up));

        assert_eq!(block.vanilla_id(), 2512);
        assert_eq!(BlockId::from_vanilla_id(block.vanilla_id()), block);
    }

    #[test]
    fn vanilla_ids_roundtrip() {
        for id in 0..8598 {
            assert_eq!(BlockId::from_vanilla_id(id).vanilla_id(), id);

            if id != 0 {
                assert_ne!(BlockId::from_vanilla_id(id), BlockId::air());
            }
        }
    }

    #[test]
    fn property_starting_at_1() {
        let block = BlockId::snow().with_layers(1);

        assert_eq!(block.layers(), Some(1));
        assert_eq!(block.to_properties_map()["layers"], "1");
    }
}
