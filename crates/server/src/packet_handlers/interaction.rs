use crate::{ClientId, NetworkId, Server};
use common::Game;
use common::{
    events::{BlockInteractEvent, BlockPlacementEvent, InteractEntityEvent},
    interactable::InteractableRegistry,
};
use ecs::{Entity, SysResult};
use libcraft_core::{InteractBlockFace, InteractHand, InteractionType, Vec3f};
use protocol::packets::client::{
    BlockFace, InteractEntity, InteractEntityKind, PlayerBlockPlacement, PlayerDigging,
    PlayerDiggingStatus,
};

/// Handles the player block placement packet. Currently just removes the block client side for the player.
pub fn handle_player_block_placement(
    game: &mut Game,
    _server: &mut Server,
    packet: PlayerBlockPlacement,
    player: Entity,
) -> SysResult {
    let hand = match packet.hand {
        0 => InteractHand::Main,
        1 => InteractHand::Offhand,
        _ => unreachable!(),
    };

    let face = match packet.face {
        BlockFace::North => InteractBlockFace::North,
        BlockFace::South => InteractBlockFace::South,
        BlockFace::East => InteractBlockFace::East,
        BlockFace::West => InteractBlockFace::West,
        BlockFace::Top => InteractBlockFace::Top,
        BlockFace::Bottom => InteractBlockFace::Bottom,
    };

    let cursor_position = Vec3f::new(
        packet.cursor_position_x,
        packet.cursor_position_y,
        packet.cursor_position_z,
    );

    let block_kind = { game.block(packet.position).unwrap().kind() };

    let interactable_registry = game
        .resources
        .get::<InteractableRegistry>()
        .expect("Failed to get the interactable registry");

    if interactable_registry.is_registered(block_kind) {
        // Handle this as a block interaction
        let event = BlockInteractEvent {
            hand,
            location: packet.position,
            face,
            cursor_position,
            inside_block: packet.inside_block,
        };

        game.ecs.insert_entity_event(player, event)?;
    } else {
        // Handle this as a block placement
        let event = BlockPlacementEvent {
            hand,
            location: packet.position,
            face,
            cursor_position,
            inside_block: packet.inside_block,
        };

        game.ecs.insert_entity_event(player, event)?;
    }

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
