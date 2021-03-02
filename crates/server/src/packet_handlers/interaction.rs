use crate::{ClientId, NetworkId, Server};
use common::events::InteractEntityEvent;
use common::Game;
use ecs::{Entity, EntityRef, SysResult};
use libcraft_core::{InteractHand, InteractionType, Vec3f};
use protocol::packets::client::{
    BlockFace, InteractEntity, InteractEntityKind, PlayerBlockPlacement, PlayerDigging,
    PlayerDiggingStatus,
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

        found_entity.expect("Invalid entity")
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
