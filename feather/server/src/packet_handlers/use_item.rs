use crate::Server;
use base::TPS;
use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::UseItem;
use quill_common::components_effects::*;

pub fn handle_use_item(
    game: &mut Game,
    _server: &mut Server,
    _packet: UseItem,
    player: Entity,
) -> SysResult {
    // example for effect system
    let mut speed = SpeedEffect::default();
    speed.add_effect(EffectApplication {
        amplifier: 0,
        duration: 20 * TPS,
        flags: EffectFlags {
            particle: true,
            ambient: false,
            icon: true,
        },
        start_tick: 0,
    });

    speed.add_effect(EffectApplication {
        amplifier: 1,
        duration: 10 * TPS,
        flags: EffectFlags {
            particle: true,
            ambient: false,
            icon: true,
        },
        start_tick: 0,
    });

    game.ecs.insert(player, speed)?;
    Ok(())
}
