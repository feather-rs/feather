//! Broadcasting of block updates, i.e. when a block is changed to another.

use feather_core::network::packets::{BlockBreakAnimation, BlockChange, Effect};
use feather_server_types::{BlockUpdateCause, BlockUpdateEvent, Game, NetworkId};
use fecs::World;

/// System for broadcasting block update
/// events to all clients.
#[fecs::event_handler]
pub fn on_block_update_broadcast(event: &BlockUpdateEvent, game: &mut Game, world: &mut World) {
    // Broadcast Block Change packet.
    let packet = BlockChange {
        location: event.pos,
        block_id: event.new.vanilla_id() as i32,
    };
    game.broadcast_chunk_update(world, packet, event.pos.into(), None);
}

/// Sends an `Effect` packet with status `BlockBreak`
/// when a block is broken by a player.
#[fecs::event_handler]
pub fn on_block_break_broadcast_effect(
    event: &BlockUpdateEvent,
    game: &mut Game,
    world: &mut World,
) {
    if let BlockUpdateCause::Entity(_) = event.cause {
        let packet = Effect {
            effect_id: 2001, // TODO remove hardcoded magic number
            location: event.pos,
            data: event.old.vanilla_id() as i32,
            disable_relative_volume: false,
        };
        game.broadcast_chunk_update(world, packet, event.pos.chunk(), None);
    }
}

/// Sends `BlockBreakAnimation` when a block is broking.
#[fecs::event_handler]
pub fn on_block_break_broadcast_animation(
    event: &BlockUpdateEvent,
    game: &mut Game,
    world: &mut World,
) {
    if event.new.is_air() && !event.old.is_air() {
        if let BlockUpdateCause::Entity(entity) = event.cause {
            let packet = BlockBreakAnimation {
                entity_id: world.get::<NetworkId>(entity).0,
                location: event.pos,
                destroy_stage: 10, // removes the block
            };
            game.broadcast_chunk_update(world, packet, event.pos.chunk(), None);
        }
    }
}
