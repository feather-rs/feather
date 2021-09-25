use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use thiserror::Error;

use libcraft_core::{BlockPosition, ChunkPosition, Position};
use std::convert::TryFrom;

/// Validated position of a block.
///
/// This structure is immutable.
/// All operations that change a [`ValidBlockPosition`] must be done by
/// turning it into a [`BlockPosition`], performing said operations,
/// then using [`ValidBlockPosition`]'s [`TryFrom`] impl to get a [`ValidBlockPosition`].
///
/// The definition of a valid block position is defined by [`BlockPosition::valid`].
///
/// # Examples
///
/// Converting a [`BlockPosition`] to a [`ValidBlockPosition`], unwrapping any errors that
/// occur.
/// ```
/// # use feather_base::BlockPosition;
/// # use feather_base::ValidBlockPosition;
/// # use std::convert::TryInto;
/// // Create an unvalidated block position
/// let block_position = BlockPosition::new(727, 32, 727);
///
/// // Validate the block position and unwrap any errors
/// let valid_block_position: ValidBlockPosition = block_position.try_into().unwrap();
/// ```
///
/// Performing operations on a [`ValidBlockPosition`], then re-validating it.
/// ```
/// # use feather_base::BlockPosition;
/// # use feather_base::ValidBlockPosition;
/// # use std::convert::TryInto;
/// # let mut valid_block_position: ValidBlockPosition = BlockPosition::new(727, 32, 727).try_into().unwrap();
/// // Convert the ValidBlockPosition into an unvalidated one to perform math
/// let mut block_position: BlockPosition = valid_block_position.into();
///
/// block_position.x = 821;
/// block_position.z += 32;
///
/// assert!(block_position.valid());
///
/// valid_block_position = block_position.try_into().unwrap();
/// ```
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    Zeroable,
    Pod,
)]
#[repr(C)]
pub struct ValidBlockPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl ValidBlockPosition {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn chunk(self) -> ChunkPosition {
        self.into()
    }

    pub fn position(self) -> Position {
        self.into()
    }
}

impl TryFrom<BlockPosition> for ValidBlockPosition {
    type Error = BlockPositionValidationError;

    fn try_from(value: BlockPosition) -> Result<Self, Self::Error> {
        if value.valid() {
            Ok(ValidBlockPosition {
                x: value.x,
                y: value.y,
                z: value.z,
            })
        } else {
            Err(BlockPositionValidationError::OutOfRange(value))
        }
    }
}

impl From<ValidBlockPosition> for BlockPosition {
    fn from(position: ValidBlockPosition) -> Self {
        BlockPosition {
            x: position.x,
            y: position.y,
            z: position.z,
        }
    }
}

impl From<ValidBlockPosition> for ChunkPosition {
    fn from(position: ValidBlockPosition) -> Self {
        let position: BlockPosition = position.into();
        position.into()
    }
}

impl From<ValidBlockPosition> for Position {
    fn from(position: ValidBlockPosition) -> Self {
        let position: BlockPosition = position.into();
        position.into()
    }
}

#[derive(Error, Debug)]
pub enum BlockPositionValidationError {
    #[error("coordinate {0:?} out of range")]
    OutOfRange(BlockPosition),
}

#[cfg(test)]
mod tests {

    use std::convert::TryInto;

    use libcraft_core::BlockPosition;

    use crate::ValidBlockPosition;

    #[test]
    #[should_panic]
    fn check_out_of_bounds_up() {
        let block_position = BlockPosition::new(0, 39483298, 0);

        <BlockPosition as TryInto<ValidBlockPosition>>::try_into(block_position).unwrap();
    }

    #[test]
    #[should_panic]
    fn check_out_of_bounds_down() {
        let block_position = BlockPosition::new(0, -39483298, 0);

        <BlockPosition as TryInto<ValidBlockPosition>>::try_into(block_position).unwrap();
    }
}
