//! Defines all systems and the order in which they are executed.

use fecs::Executor;

use feather_server_chunk as chunk_logic;
use feather_server_entity as entity;
use feather_server_physics as physics;
use feather_server_player as player;
use feather_server_types as game;
use feather_server_util as util;
use feather_server_weather as weather;

pub fn build_executor() -> Executor {
    Executor::new()
        .with(player::poll_player_disconnect)
        .with(player::poll_new_clients)
        .with(physics::entity_physics)
        .with(player::handle_movement_packets)
        .with(player::handle_close_window)
        .with(player::handle_creative_inventory_action)
        .with(player::handle_click_windows)
        .with(player::handle_held_item_change)
        .with(player::handle_animation)
        .with(player::handle_player_block_placement)
        .with(player::handle_player_use_item)
        .with(player::handle_player_digging)
        .with(player::advance_dig_progress)
        .with(player::broadcast_block_break_animation)
        .with(player::handle_client_status)
        .with(player::handle_chat)
        .with(player::flush_player_message_receiver)
        .with(game::task::run_sync_tasks)
        .with(player::send_teleported)
        .with(weather::update_weather)
        .with(entity::item::item_collect)
        .with(chunk_logic::handle_chunk_worker_replies)
        .with(chunk_logic::chunk_unload)
        .with(chunk_logic::chunk_optimize)
        .with(player::check_crossed_chunks)
        .with(player::broadcast_keepalive)
        .with(entity::broadcast_movement)
        .with(entity::update_blocks_fallen)
        .with(entity::broadcast_velocity)
        .with(entity::falling_block::spawn_falling_blocks)
        .with(entity::supported_blocks::break_unsupported_blocks)
        .with(chunk_logic::chunk_save)
        .with(game::reset_bump_allocators)
        .with(game::increment_tick_count)
        .with(util::increment_time)
        .with(entity::previous_position_velocity_reset) // should be at end
}
