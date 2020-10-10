macro_rules! gamerules {
    {$($name:ident: $value:ty = $default:literal),*$(,)*} => {
        #[derive(Debug)]
        pub struct GameRules {
            $(
                pub $name: $value
            ),*
        }

        impl Default for GameRules {
            fn default() -> Self {
                Self {
                    $(
                        $name: $default
                    ),*
                }
            }
        }
    };
}

gamerules! {
    announce_advancements: bool = true,
    command_block_output: bool = true,
    disable_elytra_movement_check: bool = false,
    disable_raids: bool = false,
    do_daylight_cycle: bool = true,
    do_entity_drops: bool = true,
    do_fire_tick: bool = true,
    do_insomnia: bool = true,
    do_immediate_respawn: bool = false,
    do_limited_crafting: bool = false,
    do_mob_loot: bool = true,
    do_mob_spawning: bool = true,
    do_patrol_spawning: bool = true,
    do_tile_drops: bool = true,
    do_trader_spawning: bool = true,
    do_weather_cycle: bool = true,
    drowning_damage: bool = true,
    fall_damage: bool = true,
    fire_damage: bool = true,
    keep_inventory: bool = false,
    log_admin_commands: bool = true,
    max_command_chain_length: u32 = 65536,
    max_entity_cramming: u32 = 24,
    mob_griefing: bool = true,
    natural_regeneration: bool = true,
    random_tick_speed: u32 = 3,
    reduced_debug_info: bool = false,
    send_command_feedback: bool = true,
    show_death_messages: bool = true,
    spawn_radius: u32 = 10,
    spectators_generate_chunks: bool = true,
}
