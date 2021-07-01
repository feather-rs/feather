use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::{EntityAction, EntityActionKind};
use quill_common::{
    components::{Sneaking, Sprinting},
    events::{SneakEvent, SprintEvent},
};

///  From [wiki](https://wiki.vg/Protocol#Entity_Action)
///  Sent by the client to indicate that it has performed certain actions:
///  *) sneaking (crouching),
///  *) sprinting,
///  *) exiting a bed,
///  *) jumping with a horse,
///  *) opening a horse's inventory while riding it.
///
pub fn handle_entity_action(game: &mut Game, player: Entity, packet: EntityAction) -> SysResult {
    match packet.action_id {
        EntityActionKind::StartSneaking => {
            let is_sneaking = game.ecs.get_mut::<Sneaking>(player)?.0;
            if !is_sneaking {
                game.ecs
                    .insert_entity_event(player, SneakEvent::new(true))?;
                game.ecs.get_mut::<Sneaking>(player)?.0 = true;
            }
        }
        EntityActionKind::StopSneaking => {
            let is_sneaking = game.ecs.get_mut::<Sneaking>(player)?.0;
            if is_sneaking {
                game.ecs
                    .insert_entity_event(player, SneakEvent::new(false))?;
                game.ecs.get_mut::<Sneaking>(player)?.0 = false;
            }
        }
        EntityActionKind::LeaveBed => {
            //TODO issue #423
            // Note that the leave bed packet is not sent if the server changes night to day
            // and all players are kicked out of the bed. We have to seperatly send out
            // a notice that bed state might have changed.
        }
        EntityActionKind::StartSprinting | EntityActionKind::StopSprinting => {
            let start_sprinting = matches!(packet.action_id, EntityActionKind::StartSprinting);
            let is_sprinting = game.ecs.get_mut::<Sprinting>(player)?.0;
            if is_sprinting != start_sprinting {
                game.ecs
                    .insert_entity_event(player, SprintEvent::new(start_sprinting))?;
                game.ecs.get_mut::<Sprinting>(player)?.0 = start_sprinting;
            }
        }
        EntityActionKind::StartHorseJump => {
            //TODO issue #423
        }
        EntityActionKind::StopJorseJump => {
            //TODO issue #423
        }
        EntityActionKind::OpenHorseInventory => {
            //TODO issue #423
        }
        EntityActionKind::StartElytraFlight => {
            //TODO issue #423
        }
    }

    Ok(())
}
