use num_traits::cast::ToPrimitive;

use base::anvil::entity::{AnimalData, BaseEntityData};
use base::anvil::player::{InventorySlot, PlayerAbilities, PlayerData};
use base::{Gamemode, Inventory, Position, Text};
use common::entities::player::HotbarSlot;
use common::{chat::ChatKind, Game};
use ecs::{SysResult, SystemExecutor};
use quill_common::components::{
    CanBuild, CanCreativeFly, CreativeFlying, CreativeFlyingSpeed, Health, Instabreak,
    Invulnerable, Name, PreviousGamemode, WalkSpeed,
};

use crate::{ClientId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(remove_disconnected_clients);
}

fn remove_disconnected_clients(game: &mut Game, server: &mut Server) -> SysResult {
    let mut entities_to_remove = Vec::new();
    for (
        player,
        (
            &client_id,
            name,
            position,
            gamemode,
            previous_gamemode,
            health,
            walk_speed,
            fly_speed,
            can_fly,
            is_flying,
            can_build,
            instabreak,
            invulnerable,
            hotbar_slot,
            inventory,
        ),
    ) in game
        .ecs
        .query::<(
            &ClientId,
            &Name,
            &Position,
            &Gamemode,
            &PreviousGamemode,
            &Health,
            &WalkSpeed,
            &CreativeFlyingSpeed,
            &CanCreativeFly,
            &CreativeFlying,
            &CanBuild,
            &Instabreak,
            &Invulnerable,
            &HotbarSlot,
            &Inventory,
        )>()
        .iter()
    {
        let client = server.clients.get(client_id).unwrap();
        if client.is_disconnected() {
            entities_to_remove.push(player);
            broadcast_player_leave(game, name);
            game.world
                .save_player_data(
                    client.uuid(),
                    &create_player_data(
                        *position,
                        *gamemode,
                        *previous_gamemode,
                        *health,
                        PlayerAbilities {
                            walk_speed: *walk_speed,
                            fly_speed: *fly_speed,
                            may_fly: *can_fly,
                            is_flying: *is_flying,
                            may_build: *can_build,
                            instabreak: *instabreak,
                            invulnerable: *invulnerable,
                        },
                        *hotbar_slot,
                        inventory,
                    ),
                )
                .unwrap_or_else(|e| panic!("Couldn't save data for {}: {}", client.username(), e));
            server.remove_client(client_id);
        }
    }

    for player in entities_to_remove {
        game.remove_entity(player)?;
    }

    Ok(())
}

fn broadcast_player_leave(game: &Game, username: &Name) {
    let message = Text::translate_with("multiplayer.player.left", vec![username.to_string()]);
    game.broadcast_chat(ChatKind::System, message);
}

fn create_player_data(
    position: Position,
    gamemode: Gamemode,
    previous_gamemode: PreviousGamemode,
    health: Health,
    abilities: PlayerAbilities,
    hotbar_slot: HotbarSlot,
    inventory: &Inventory,
) -> PlayerData {
    PlayerData {
        animal: AnimalData {
            base: BaseEntityData {
                position: [position.x, position.y, position.z].into(),
                rotation: [position.yaw, position.pitch].into(),
                velocity: [0.0, 0.0, 0.0].into(),
            },
            health: *health,
        },
        gamemode: gamemode.to_i32().unwrap(),
        previous_gamemode: previous_gamemode.id() as i32,
        inventory: inventory
            .to_vec()
            .iter()
            .enumerate()
            // Here we filter out all empty slots.
            .filter_map(|(slot, item)| {
                match item {
                    libcraft_items::InventorySlot::Filled(item) => {
                        let res = InventorySlot::from_network_index(slot, item);
                        match res {
                            Some(i) => Some(i),
                            None => {
                                log::error!("Failed to convert the slot into anvil format.");
                                None
                            }
                        }
                    }
                    libcraft_items::InventorySlot::Empty => {
                        // Empty items are filtered out.
                        None
                    }
                }
            })
            .collect(),
        held_item: hotbar_slot.get() as i32,
        abilities,
    }
}
