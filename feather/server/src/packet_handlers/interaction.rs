use common::block::placement::item_to_block;
use common::entities::player::HotbarSlot;
use common::events::BlockChangeEvent;
use common::interactable::InteractableRegistry;
use common::{Game, PlayerWindow};
use libcraft::anvil::inventory_consts::{SLOT_HOTBAR_OFFSET, SLOT_OFFHAND};
use libcraft::{Area, BlockFace as LibcraftBlockFace, Hand};
use libcraft::{BlockKind, BlockState};
use libcraft::{InteractionType, Vec3f};
use protocol::packets::client::{
    BlockFace, HeldItemChange, InteractEntity, InteractEntityKind, PlayerBlockPlacement,
    PlayerDigging, PlayerDiggingStatus,
};
use quill::components::{EntityInventory, EntityPosition, EntityWorld, Sneaking};
use quill::events::{
    BlockInteractEvent, BlockPlacementEvent, HeldItemChangeEvent, InteractEntityEvent,
};
use vane::{Entity, SysResult};

use crate::{ClientId, NetworkId, Server};

/// Handles the player block placement packet. Currently just removes the block client side for the player.
pub fn handle_player_block_placement(
    game: &mut Game,
    server: &mut Server,
    packet: PlayerBlockPlacement,
    player: Entity,
) -> SysResult {
    let hand = match packet.hand {
        0 => Hand::Main,
        1 => Hand::Offhand,
        _ => {
            let client_id = game.ecs.get::<ClientId>(player).unwrap();

            let client = server.clients.get_mut(*client_id).unwrap();

            client.disconnect("Malformed Packet!");

            anyhow::bail!(
                "Player sent a malformed `PlayerBlockPlacement` packet. {:?}",
                packet
            )
        }
    };

    let face = match packet.face {
        BlockFace::North => LibcraftBlockFace::North,
        BlockFace::South => LibcraftBlockFace::South,
        BlockFace::East => LibcraftBlockFace::East,
        BlockFace::West => LibcraftBlockFace::West,
        BlockFace::Top => LibcraftBlockFace::Top,
        BlockFace::Bottom => LibcraftBlockFace::Bottom,
    };

    let cursor_position = Vec3f::new(
        packet.cursor_position_x,
        packet.cursor_position_y,
        packet.cursor_position_z,
    );

    let block_kind = {
        let world_id = game.ecs.get::<EntityWorld>(player)?;
        let world = game.world(world_id.0)?;
        let result = world.block_at(packet.position.into());
        match result {
            Ok(block) => block.kind(),
            Err(_) => {
                let client_id = game.ecs.get::<ClientId>(player).unwrap();

                let client = server.clients.get_mut(*client_id).unwrap();

                client.disconnect("Attempted to interact with an unloaded block!");

                anyhow::bail!(
                    "Player attempted to interact with an unloaded block. {:?}",
                    packet
                )
            }
        }
    };

    let interactable_registry = game
        .resources
        .get::<InteractableRegistry>()
        .expect("Failed to get the interactable registry");

    if interactable_registry.is_registered(block_kind) && !game.ecs.get::<Sneaking>(player)?.0 {
        // Handle this as a block interaction
        let event = BlockInteractEvent {
            hand,
            location: packet.position.into(),
            face,
            cursor_position,
            inside_block: packet.inside_block,
        };

        game.ecs.insert_entity_event(player, event)?;
    } else {
        // Handle this as a block placement
        let item = {
            let inventory = game.ecs.get::<EntityInventory>(player)?;
            let slot = game.ecs.get::<HotbarSlot>(player)?;
            let item = inventory.item(Area::Hotbar, slot.get());
            item.and_then(|slot| slot.item_kind())
        };
        let block = match item.and_then(item_to_block) {
            Some(block) => block,
            None => return Ok(()),
        };
        let event = BlockPlacementEvent {
            hand,
            location: packet.position.into(),
            face,
            cursor_position,
            inside_block: packet.inside_block,
        };

        let mut block_change_events = Vec::new();
        {
            let mut world = game.world_mut(game.ecs.get::<EntityWorld>(player)?.0)?;
            let light_level = 15;

            common::block::placement::place_block(
                game,
                &mut *world,
                game.ecs.get::<EntityPosition>(player)?.0,
                block,
                &event,
                light_level,
                &mut block_change_events,
            );
        }

        game.ecs.insert_entity_event(player, event)?;

        for event in block_change_events {
            game.ecs.insert_event(event);
        }
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
pub fn handle_player_digging(
    game: &mut Game,
    server: &mut Server,
    packet: PlayerDigging,
    player: Entity,
) -> SysResult {
    log::trace!("Got player digging with status {:?}", packet.status);
    match packet.status {
        PlayerDiggingStatus::StartDigging | PlayerDiggingStatus::CancelDigging => {
            let world_id = game.ecs.get::<EntityWorld>(player)?.0;
            {
                let world = game.world(world_id)?;
                world.set_block_at(packet.position.into(), BlockState::new(BlockKind::Air))?;
            }

            game.ecs
                .insert_event(BlockChangeEvent::single(packet.position, world_id));
            Ok(())
        }
        PlayerDiggingStatus::SwapItemInHand => {
            let window = game.ecs.get::<PlayerWindow>(player)?;

            let hotbar_slot = game.ecs.get::<HotbarSlot>(player)?.get();

            let hotbar_index = SLOT_HOTBAR_OFFSET + hotbar_slot;
            let offhand_index = SLOT_OFFHAND;

            {
                let mut hotbar_item = window.item_mut(hotbar_index)?;
                let mut offhand_item = window.item_mut(offhand_index)?;

                std::mem::swap(&mut *hotbar_item, &mut *offhand_item);
            }

            let client_id = *game.ecs.get::<ClientId>(player)?;
            let client = server.clients.get(client_id).unwrap();

            client.send_window_items(&window);

            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn handle_interact_entity(
    game: &mut Game,
    server: &mut Server,
    packet: InteractEntity,
    player: Entity,
) -> SysResult {
    let target = {
        let mut found_entity = None;
        for (entity, network_id) in game.ecs.query::<&NetworkId>().iter() {
            if network_id.0 == packet.entity_id {
                found_entity = Some(entity);
                break;
            }
        }

        match found_entity {
            None => {
                let client_id = game.ecs.get::<ClientId>(player).unwrap();

                let client = server.clients.get_mut(*client_id).unwrap();

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
                0 => Hand::Main,
                1 => Hand::Offhand,
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

pub fn handle_held_item_change(
    game: &mut Game,
    player_id: Entity,
    packet: HeldItemChange,
) -> SysResult {
    let new_id = packet.slot as usize;
    let mut slot = game.ecs.get_mut::<HotbarSlot>(player_id)?;

    log::trace!("Got player slot change from {} to {}", slot.get(), new_id);

    slot.set(new_id)?;

    drop(slot);
    game.ecs
        .insert_entity_event(player_id, HeldItemChangeEvent)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use common::Game;
    use protocol::packets::client::HeldItemChange;

    use super::*;

    #[test]
    fn held_item_change() {
        let mut game = Game::default();
        let entity = game.ecs.spawn((HotbarSlot::new(0),));

        let packet = HeldItemChange { slot: 8 };

        handle_held_item_change(&mut game, entity, packet).unwrap();

        assert_eq!(
            *game.ecs.get::<HotbarSlot>(entity).unwrap(),
            HotbarSlot::new(8)
        );
    }
}
