//! Data sourced from: <https://minecraft.gamepedia.com/Game_rule>

use serde::{Deserialize, Serialize};

/// All game rules.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameRules {
    announce_advancements: bool,
    command_block_output: bool,
    disable_elytra_movement_check: bool,
    disable_raids: bool,
    do_daylight_cycle: bool,
    do_entity_drops: bool,
    do_fire_tick: bool,
    do_insomnia: bool,
    do_immediate_respawn: bool,
    do_limited_crafting: bool,
    do_mob_loot: bool,
    do_mob_spawning: bool,
    do_patrol_spawning: bool,
    do_tile_drops: bool,
    do_trader_spawning: bool,
    do_weather_cycle: bool,
    drowning_damage: bool,
    fall_damage: bool,
    fire_damage: bool,
    forgive_dead_players: bool,
    keep_inventory: bool,
    log_admin_commands: bool,
    max_command_chain_length: u32,
    max_entity_cramming: u32,
    mob_griefing: bool,
    natural_regeneration: bool,
    random_tick_speed: u32,
    reduced_debug_info: bool,
    send_command_feedback: bool,
    show_death_messages: bool,
    spawn_radius: u32,
    spectators_generate_chunks: bool,
    universal_anger: bool,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            announce_advancements: true,
            command_block_output: true,
            disable_elytra_movement_check: false,
            disable_raids: false,
            do_daylight_cycle: true,
            do_entity_drops: true,
            do_fire_tick: true,
            do_insomnia: true,
            do_immediate_respawn: false,
            do_limited_crafting: false,
            do_mob_loot: true,
            do_mob_spawning: true,
            do_patrol_spawning: true,
            do_tile_drops: true,
            do_trader_spawning: true,
            do_weather_cycle: true,
            drowning_damage: true,
            fall_damage: true,
            fire_damage: true,
            forgive_dead_players: true,
            keep_inventory: false,
            log_admin_commands: true,
            max_command_chain_length: 65536,
            max_entity_cramming: 24,
            mob_griefing: true,
            natural_regeneration: true,
            random_tick_speed: 3,
            reduced_debug_info: false,
            send_command_feedback: true,
            show_death_messages: true,
            spawn_radius: 10,
            spectators_generate_chunks: true,
            universal_anger: false,
        }
    }
}
