use std::convert::TryFrom;
use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u16)]
pub enum BlockKind {
    Air,
    Stone,
}

impl TryFrom<u16> for BlockKind {
    type Error = BlockIdFromU32Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BlockKind::Air),
            1 => Ok(BlockKind::Stone),
            x => Err(BlockIdFromU32Error::InvalidKind(x)),
        }
    }
}

impl From<BlockKind> for u16 {
    fn from(kind: BlockKind) -> Self {
        kind as u16
    }
}

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
        let kind = BlockKind::try_from(kind_id)?;

        let state = (value | ((1 << 16) - 1)) as u16;

        // todo: verify state
        Ok(BlockId { kind, state })
    }
}
