use num_traits::FromPrimitive;
use std::convert::TryFrom;
use thiserror::Error;

#[macro_use]
extern crate num_derive;

#[allow(warnings)]
#[allow(clippy::all)]
mod generated;

static BLOCK_TABLE: Lazy<BlockTable> = Lazy::new(|| {
    let bytes = include_bytes!("generated/block_table.dat");
    bincode::deserialize(bytes).expect("failed to deserialize generated block table (bincode)")
});

use crate::generated::table::{BlockTable, Instrument};
pub use generated::kind::BlockKind;
use once_cell::sync::Lazy;

impl Default for BlockKind {
    fn default() -> Self {
        BlockKind::Air
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct BlockId {
    kind: BlockKind,
    state: u16,
}

impl BlockId {
    pub fn instrument(self) -> Option<Instrument> {
        BLOCK_TABLE.instrument(self.kind, self.state)
    }

    pub fn with_instrument(self, instrument: Instrument) -> Option<BlockId> {
        BLOCK_TABLE
            .set_instrument(self.kind, self.state, instrument)
            .map(|state| self.with_state(state))
    }

    fn with_state(self, state: u16) -> Self {
        Self {
            kind: self.kind,
            state,
        }
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

        block = block.with_instrument(Instrument::Basedrum).unwrap();
        assert_eq!(block.instrument(), Some(Instrument::Basedrum));
    }
}
