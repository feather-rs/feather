use num_traits::FromPrimitive;
use std::convert::TryFrom;
use thiserror::Error;

#[macro_use]
extern crate num_derive;

#[allow(warnings)]
#[allow(clippy::all)]
mod generated;

pub use generated::kind::BlockKind;

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
