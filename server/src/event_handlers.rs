//! Defines the event handlers.
use feather_server_block::*;
use feather_server_chunk::*;
use feather_server_entity::*;
use feather_server_lighting::*;
use feather_server_player::*;
use feather_server_util::*;
use feather_server_weather::*;
use fecs::EventHandlers;

macro_rules! event_handlers {
    ($($handler:path,)*) => {
        {
            let handlers = EventHandlers::new()
                $(.with($handler))*;
            handlers
        }
    }
}

pub fn build_event_handlers() -> EventHandlers {
    event_handlers! {
        on_block_update_notify_adjacent,
        on_block_break_broadcast_effect,
        on_block_update_broadcast,
        on_block_update_notify_lighting_worker,
        on_block_break_drop_loot,
        on_chest_break_drop_contents,
        on_block_update_create_block_entity,

        on_entity_despawn_remove_chunk_holder,
        on_entity_despawn_update_chunk_entities,
        on_entity_despawn_broadcast_despawn,

        on_entity_spawn_update_chunk_entities,
        on_entity_spawn_send_to_clients,

        on_entity_send_update_last_known_positions,
        on_entity_send_send_equipment,
        on_entity_send_send_metadata,

        on_entity_client_remove_update_last_known_positions,

        on_player_join_send_join_game,
        on_player_join_send_existing_entities,
        on_player_join_send_time,
        on_player_join_trigger_chunk_cross,
        on_player_join_send_weather,
        on_player_join_broadcast_join_message,

        on_player_leave_save_data,

        on_chunk_load_notify_lighting_worker,
        on_chunk_load_send_to_clients,
        on_chunk_load_queue_for_saving,

        on_chunk_holder_release_unload_chunk,

        on_chunk_cross_update_chunks,
        on_chunk_cross_update_chunk_entities,
        on_chunk_cross_update_entities,

        on_chunk_send_join_player,

        on_inventory_update_send_set_slot,
        on_inventory_update_broadcast_equipment_update,

        on_player_animation_broadcast_animation,

        on_item_drop_spawn_item_entity,

        on_item_collect_broadcast,

        on_weather_change_broadcast_weather,

        on_chat_broadcast,

        on_entity_land_remove_falling_block,

        load_chunk_request,

        release_chunk_request,

        hold_chunk_request,

        on_finish_digging_remove_animation,

        on_start_digging_init_stage,

        on_gamemode_update_update_capabilities,
        on_gamemode_update_send,

        on_health_update_send,

        on_player_death_scatter_inventory,
        on_player_death_mark_dead,
    }
}
