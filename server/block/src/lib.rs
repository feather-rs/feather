#![forbid(unsafe_code)]

pub mod chest;
mod init;

pub use chest::{
    on_chest_break_drop_contents, on_chest_break_try_disconnect, on_chest_close_decrement_viewers,
    on_chest_create_try_connect, on_chest_open_increment_viewers,
};
use feather_core::{
    anvil::block_entity::BlockEntityBase,
    blocks::BlockId,
    util::{BlockPosition, Position},
};
use feather_server_types::BlockEntity;
use fecs::{EntityBuilder, EntityRef};
pub use init::{on_block_entity_create_insert_to_map, on_block_update_create_block_entity};

/// A function which determines whether a given change between
/// block states should cause a block entity to be destroyed/recreated.
///
/// First parameter is the old block; second is the new block. Return value
/// is `false` if the block entity should remain unchanged and `true`
/// if it should be replaced with a block entity for the new block.
pub struct ShouldReplace(pub fn(BlockId, BlockId) -> bool);

/// Returns the base components all block entities have.
fn base(pos: BlockPosition) -> EntityBuilder {
    EntityBuilder::new()
        .with(pos)
        .with(Position::from(pos))
        .with(BlockEntity)
}

fn serialize_base(accessor: &EntityRef) -> BlockEntityBase {
    let pos = *accessor.get::<BlockPosition>();
    BlockEntityBase {
        x: pos.x,
        y: pos.y,
        z: pos.z,
    }
}

fn load_base(data: &BlockEntityBase) -> BlockPosition {
    BlockPosition::new(data.x, data.y, data.z)
}
