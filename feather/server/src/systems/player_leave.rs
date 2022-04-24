use num_traits::cast::ToPrimitive;

use common::entities::player::HotbarSlot;
use common::Game;
use libcraft::anvil::entity::{AnimalData, BaseEntityData};
use libcraft::anvil::player::{InventorySlot, PlayerAbilities, PlayerData};
use libcraft::{Gamemode, Inventory, Position, Text};
use quill::chat::ChatKind;
use quill::components::{
    CanBuild, CanCreativeFly, CreativeFlying, CreativeFlyingSpeed, EntityInventory, EntityPosition,
    Health, Instabreak, Invulnerable, Name, PlayerGamemode, PreviousGamemode, WalkSpeed,
};
use quill::World;
use vane::{SysResult, SystemExecutor};

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
            client_id,
            name,
            _position,
            _gamemode,
            _previous_gamemode,
            _health,
            _walk_speed,
            _fly_speed,
            _can_fly,
            _is_flying,
            _can_build,
            _instabreak,
            _invulnerable,
            _hotbar_slot,
            _inventory,
        ),
    ) in game
        .ecs
        .query::<(
            &ClientId,
            &Name,
            &EntityPosition,
            &PlayerGamemode,
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
            &EntityInventory,
        )>()
        .iter()
    {
        let client = server.clients.get(*client_id).unwrap();
        if client.is_disconnected() {
            entities_to_remove.push(player);
            broadcast_player_leave(game, &name);
            server.remove_client(*client_id);
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

#[allow(clippy::too_many_arguments)]
#[allow(unused)]
fn create_player_data(
    position: Position,
    gamemode: Gamemode,
    previous_gamemode: PreviousGamemode,
    health: Health,
    abilities: PlayerAbilities,
    hotbar_slot: HotbarSlot,
    inventory: &Inventory,
    world: &dyn World,
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
        dimension: world.dimension_info().r#type.clone(),
        inventory: inventory
            .to_vec()
            .iter()
            .enumerate()
            // Here we filter out all empty slots.
            .filter_map(|(slot, item)| {
                match item {
                    libcraft::items::InventorySlot::Filled(item) => {
                        let res = InventorySlot::from_network_index(slot, item);
                        match res {
                            Some(i) => Some(i),
                            None => {
                                log::error!("Failed to convert the slot into anvil format.");
                                None
                            }
                        }
                    }
                    libcraft::items::InventorySlot::Empty => {
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
