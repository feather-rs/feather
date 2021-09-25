use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use thiserror::Error;

use libcraft_core::BlockPosition;
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
/// # use base::BlockPosition;
/// # use feather_common::ValidBlockPosition;
/// # use std::convert::TryInto;
/// // Create an unvalidated block position
/// let block_position = BlockPosition::new(727, 32, 727);
///
/// // Validate the block position and unwrap any errors
/// let valid_block_position: ValidBlockPosition = block_position.try_into().unwrap()
/// ```
///
/// Performing operations on a [`ValidBlockPosition`], then re-validating it.
/// ```
/// # use base::BlockPosition;
/// # use feather_common::ValidBlockPosition;
/// # use std::convert::TryInto;
/// # let mut valid_block_position: ValidBlockPosition = BlockPosition::new(727, 32, 727).try_into().unwrap();
/// // Convert the ValidBlockPosition into an unvalidated one to perform math
/// let block_position: BlockPosition = valid_block_position.into();
///
/// block_position.x = 821;
/// block_position.z += 32;
///
/// assert!(block_position.valid());
///
/// let valid_block_position = block_position.try_into().unwrap();
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

impl Into<BlockPosition> for ValidBlockPosition {
    fn into(self) -> BlockPosition {
        BlockPosition {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

#[derive(Error, Debug)]
pub enum BlockPositionValidationError {
    #[error("coordinate {0:?} out of range")]
    OutOfRange(BlockPosition),
}
