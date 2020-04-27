//! Broadcasting of block updates, i.e. when a block is changed to another.

use feather_core::network::packets::BlockChange;
use feather_server_types::{BlockUpdateEvent, Game};
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
