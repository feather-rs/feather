#![forbid(unsafe_code)]

mod chest;
mod init;

pub use chest::{
    on_chest_break_drop_contents, on_chest_close_decrement_viewers, on_chest_open_increment_viewers,
};
use feather_core::{
    anvil::block_entity::BlockEntityBase,
    util::{BlockPosition, Position},
};
use feather_server_types::BlockEntity;
use fecs::{EntityBuilder, EntityRef};
pub use init::{on_block_entity_create_insert_to_map, on_block_update_create_block_entity};

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
