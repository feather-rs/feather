//! Broadcasting of block updates, i.e. when a block is changed to another.

use crate::packet_handlers::Digging;
use crate::{FinishDiggingEvent, StartDiggingEvent};
use feather_core::network::packets::{BlockBreakAnimation, BlockChange, Effect};
use feather_server_types::{BlockUpdateCause, BlockUpdateEvent, BumpVec, Game, NetworkId};
use fecs::{IntoQuery, Read, World, Write};

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
    if let BlockUpdateCause::Entity(source) = event.cause {
        let packet = Effect {
            effect_id: 2001, // TODO remove hardcoded magic number
            location: event.pos,
            data: event.old.vanilla_id() as i32,
            disable_relative_volume: false,
        };
        game.broadcast_chunk_update(world, packet, event.pos.chunk(), Some(source));
    }
}

/// Component storing the most recently sent `BlockBreakAnimation`
/// destroy stage for a player.
#[derive(Copy, Clone, Debug)]
struct LastDestroyStage(i8);

/// Sends `BlockBreakAnimation` while a block is being dug.
#[fecs::system]
pub fn broadcast_block_break_animation(game: &mut Game, world: &mut World) {
    let mut broadcasts = BumpVec::new_in(game.bump());
    for (entity, (digging, entity_id, mut last_destroy_stage)) in
        <(Read<Digging>, Read<NetworkId>, Write<LastDestroyStage>)>::query()
            .iter_entities_mut(world.inner_mut())
    {
        let destroy_stage = ((digging.progress / digging.time) * 9.0).floor().min(9.0) as i8;
        if destroy_stage == last_destroy_stage.0 {
            return; // no new data to send
        }
        last_destroy_stage.0 = destroy_stage;

        let packet = BlockBreakAnimation {
            entity_id: entity_id.0,
            location: digging.pos,
            destroy_stage,
        };
        broadcasts.push((packet, entity));
    }

    for (packet, entity) in broadcasts {
        game.broadcast_entity_update(world, packet, entity, Some(entity));
    }
}

/// Removes `LastDestroyStage` and broadcasts
/// that a block animation is finished a player finishes digging.
#[fecs::event_handler]
pub fn on_finish_digging_remove_animation(
    event: &FinishDiggingEvent,
    game: &mut Game,
    world: &mut World,
) {
    let _ = world.remove::<LastDestroyStage>(event.player);

    let packet = BlockBreakAnimation {
        entity_id: world.get::<NetworkId>(event.player).0,
        location: event.digging.pos,
        destroy_stage: -1, // value outside of [0, 9] removes the animation
    };
    game.broadcast_entity_update(world, packet, event.player, None);
}

/// Inserts `LastDestroyStage` when a player starts digging.
#[fecs::event_handler]
pub fn on_start_digging_init_stage(event: &StartDiggingEvent, world: &mut World) {
    let _ = world.add(event.player, LastDestroyStage(-1));
}
