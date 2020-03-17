//! Defines all systems and the order in which they are executed.

use super::*;
use fecs::Executor;

pub fn build_executor() -> Executor {
    Executor::new()
        .with(network::poll_player_disconnect)
        .with(network::poll_new_clients)
        .with(packet_handlers::handle_movement_packets)
        .with(packet_handlers::handle_creative_inventory_action)
        .with(packet_handlers::handle_held_item_change)
        .with(packet_handlers::handle_animation)
        .with(packet_handlers::handle_player_block_placement)
        .with(packet_handlers::handle_player_digging)
        .with(chunk_logic::chunk_load)
        .with(chunk_logic::chunk_unload)
        .with(chunk_logic::chunk_optimize)
        .with(view::check_crossed_chunks)
        .with(broadcasters::broadcast_keepalive)
        .with(broadcasters::broadcast_entity_movement)
        .with(game::reset_bump_allocators)
        .with(game::increment_tick_count)
        .with(entity::position_reset) // should be at end
}
