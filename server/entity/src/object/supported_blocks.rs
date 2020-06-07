//! Implements blocks that break when not supported by a full block: torches, snow, grass, etc.

use feather_core::blocks::BlockId;
use feather_server_types::{BlockUpdateCause, Game, BumpVec};
use feather_server_util::{BlockNotifyPosition, BlockNotifySupportedBlock, BlockNotifyBlock};
use fecs::{component, IntoQuery, Read, World};

/// System to check for supporting block when a block notify
/// entity is spawned with `BlockNotifySupportedBlock`.
#[fecs::system]
pub fn break_unsupported_blocks(game: &mut Game, world: &mut World) {
    let mut actions = BumpVec::new_in(game.bump());

    actions.extend(<(Read<BlockNotifyBlock>, Read<BlockNotifyPosition>)>::query()
        .filter(component::<BlockNotifySupportedBlock>())
        .iter_entities(world.inner())
        .map(|(entity, (block, position))| {
            let support_type = block.0.needs_support().unwrap();

            let pos =
                if game
                    .block_at(position.0 + support_type.direction().offset())
                    .map(|id| support_type.is_valid_support(id.kind()))
                    != Some(true)
            {
                Some(position.0)
            } else {
                None
            };

            (entity, pos)
        })
    );

    for (entity, pos) in actions {
        world.despawn(entity);

        if let Some(pos) = pos {
            game.set_block_at(world, pos, BlockId::air(), BlockUpdateCause::Physics);
        }
    }
}
