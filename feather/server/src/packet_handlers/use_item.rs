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
    speed.add_effect(PotionApplication {
        amplifier: 0,
        duration: 20 * TPS,
        particle: false,
        ambient: false,
        icon: false,
        start_tick: 0,
    });

    speed.add_effect(PotionApplication {
        amplifier: 1,
        duration: 10 * TPS,
        particle: false,
        ambient: false,
        icon: false,
        start_tick: 0,
    });

    game.ecs.insert(player, speed)?;
    Ok(())
}
