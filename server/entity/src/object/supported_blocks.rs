//! Implements blocks that break when not supported by a full block: torches, snow, grass, etc.

use feather_core::blocks::BlockId;
use feather_server_types::{BlockUpdateCause, BumpVec, Game};
use feather_server_util::{
    is_block_supported_at, BlockNotifyBlock, BlockNotifyPosition, BlockNotifySupportedBlock,
};
use fecs::{component, IntoQuery, Read, World};

/// System to check for supporting block when a block notify
/// entity is spawned with `BlockNotifySupportedBlock`.
#[fecs::system]
pub fn break_unsupported_blocks(game: &mut Game, world: &mut World) {
    let mut actions = BumpVec::new_in(game.bump());

    actions.extend(
        <(Read<BlockNotifyBlock>, Read<BlockNotifyPosition>)>::query()
            .filter(component::<BlockNotifySupportedBlock>())
            .iter_entities(world.inner())
            .map(|(entity, (block, position))| {
                let pos = if !is_block_supported_at(block.0, game, position.0) {
                    Some(position.0) // Mark block for destruction
                } else {
                    None
                };

                (entity, pos)
            }),
    );

    for (entity, pos) in actions {
        world.despawn(entity); // Despawn BlockNotify entity

        if let Some(pos) = pos {
            // Destroy block
            game.set_block_at(world, pos, BlockId::air(), BlockUpdateCause::Unsupported);
        }
    }
}
