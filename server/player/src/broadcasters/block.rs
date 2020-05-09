//! Broadcasting of block updates, i.e. when a block is changed to another.

use crate::packet_handlers::Digging;
use feather_core::network::packets::{BlockBreakAnimation, BlockChange, Effect};
use feather_server_types::{BlockUpdateCause, BlockUpdateEvent, Game, NetworkId};
use fecs::{IntoQuery, Read, World};

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

/// Sends `BlockBreakAnimation` while a block is being dug.
#[fecs::system]
pub fn broadcast_block_break_animation(game: &mut Game, world: &mut World) {
    <(Read<Digging>, Read<NetworkId>)>::query().par_entities_for_each(
        world.inner(),
        |(entity, (digging, entity_id))| {
            let destroy_stage = ((digging.progress / digging.time) * 9.0).round().min(9.0) as i8;
            let packet = BlockBreakAnimation {
                entity_id: entity_id.0,
                location: digging.pos,
                destroy_stage,
            };
            game.broadcast_entity_update(world, packet, entity, None);
        },
    );
}
