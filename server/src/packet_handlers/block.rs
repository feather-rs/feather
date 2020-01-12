//! Broadcasting of block updates, i.e. when a block is changed to another.

use crate::block::{BlockUpdateCause, BlockUpdateEvent};
use crate::state::State;
use feather_core::network::packet::implementation::BlockChange;
use feather_core::BlockExt;

/// System for broadcasting block update
/// events to all clients.
#[event_handler]
fn broadcast_block_update(event: &BlockUpdateEvent, state: &State) {
    // Broadcast Block Change packet.
    let neq = if let BlockUpdateCause::Player(player) = event.cause {
        Some(player)
    } else {
        None
    };

    let packet = BlockChange {
        location: event.pos,
        block_id: event.new_block.native_state_id() as i32,
    };
    state.broadcast_chunk_update(event.pos.chunk_pos(), packet, neq);
}
