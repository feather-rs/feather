use crate::{ClientId, NetworkId, Server};
use common::{entities::player::HotbarSlot, Game};
use common::events::InteractEntityEvent;
use ecs::{Entity, EntityRef, SysResult};
use libcraft_core::{InteractHand, InteractionType, Vec3f};
use protocol::packets::client::{
    BlockFace, HeldItemChange,
    InteractEntity, InteractEntityKind, PlayerBlockPlacement,
    PlayerDigging, PlayerDiggingStatus,
};

/// Handles the player block placement packet. Currently just removes the block client side for the player.
pub fn handle_player_block_placement(
    game: &Game,
    server: &mut Server,
    packet: PlayerBlockPlacement,
    player: EntityRef,
) -> SysResult {
    let position = match packet.face {
        BlockFace::Bottom => packet.position.down(),
        BlockFace::Top => packet.position.up(),
        BlockFace::North => packet.position.north(),
        BlockFace::South => packet.position.south(),
        BlockFace::West => packet.position.west(),
        BlockFace::East => packet.position.east(),
    };

    log::trace!("Got player block placement at {:?}", position);

    let client = server.clients.get(*player.get::<ClientId>()?).unwrap();
    let block_id = game.block(position).unwrap_or_default();

    client.send_block_change(position, block_id);
    Ok(())
}

/// Handles the Player Digging packet sent for the following
/// actions:
/// * Breaking blocks.
/// * Dropping items.
/// * Shooting arrows.
/// * Eating.
/// * Swapping items between the main and off hand.
pub fn handle_player_digging(game: &mut Game, packet: PlayerDigging, _player: Entity) -> SysResult {
    log::trace!("Got player digging with status {:?}", packet.status);
    match packet.status {
        PlayerDiggingStatus::StartDigging | PlayerDiggingStatus::CancelDigging => {
            game.break_block(packet.position);
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn handle_interact_entity(
    game: &mut Game,
    _server: &mut Server,
    packet: InteractEntity,
    player: Entity,
) -> SysResult {
    let target = {
        let mut found_entity = None;
        for (entity, &network_id) in game.ecs.query::<&NetworkId>().iter() {
            if network_id.0 == packet.entity_id {
                found_entity = Some(entity);
                break;
            }
        }

        match found_entity {
            None => {
                let client_id = game.ecs.get::<ClientId>(player).unwrap();

                let client = _server.clients.get(*client_id).unwrap();

                client.disconnect("Interacted with an invalid entity!");

                anyhow::bail!("Player attempted to interact with an invalid entity.")
            }
            Some(entity) => entity,
        }
    };

    let event = match packet.kind {
        InteractEntityKind::Attack => InteractEntityEvent {
            target,
            ty: InteractionType::Attack,
            target_pos: None,
            hand: None,
            sneaking: packet.sneaking,
        },
        InteractEntityKind::Interact => InteractEntityEvent {
            target,
            ty: InteractionType::Interact,
            target_pos: None,
            hand: None,
            sneaking: packet.sneaking,
        },
        InteractEntityKind::InteractAt {
            target_x,
            target_y,
            target_z,
            hand,
        } => {
            let hand = match hand {
                0 => InteractHand::Main,
                1 => InteractHand::Offhand,
                _ => unreachable!(),
            };

            InteractEntityEvent {
                target,
                ty: InteractionType::Attack,
                target_pos: Some(Vec3f::new(
                    target_x as f32,
                    target_y as f32,
                    target_z as f32,
                )),
                hand: Some(hand),
                sneaking: packet.sneaking,
            }
        }
    };

    game.ecs.insert_entity_event(player, event)?;

    Ok(())
}

pub fn handle_held_item_change(player: EntityRef, packet: HeldItemChange) -> SysResult {
    let new_id = packet.slot as usize;
    let mut slot = player.get_mut::<HotbarSlot>()?;

    log::trace!("Got player slot change from {} to {}", slot.get(), new_id);

    slot.set(new_id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use common::Game;
    use protocol::packets::client::HeldItemChange;

    use super::*;

    #[test]
    fn held_item_change() {
        let mut game = Game::new();
        let entity = game.ecs.spawn((HotbarSlot::new(0),));
        let player = game.ecs.entity(entity).unwrap();

        let packet = HeldItemChange { slot: 8 };

        handle_held_item_change(player, packet).unwrap();

        assert_eq!(
            *game.ecs.get::<HotbarSlot>(entity).unwrap(),
            HotbarSlot::new(8)
        );
    }
}
