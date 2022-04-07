use std::convert::TryFrom;
use std::sync::Arc;

use log::debug;

use base::anvil::player::PlayerAbilities;
use base::biome::BiomeList;
use base::{Gamemode, Inventory, ItemStack, Position, Text};
use common::events::PlayerRespawnEvent;
use common::world::{Dimensions, WorldName, WorldPath};
use common::{
    chat::{ChatKind, ChatPreference},
    entities::player::HotbarSlot,
    view::View,
    window::BackingWindow,
    ChatBox, Game, Window,
};
use libcraft_core::EntityKind;
use libcraft_items::InventorySlot;
use quill::components;
use quill::components::{
    CanBuild, CanCreativeFly, CreativeFlying, CreativeFlyingSpeed, EntityDimension, EntityWorld,
    Health, Instabreak, Invulnerable, PreviousGamemode, WalkSpeed,
};
use quill::events::GamemodeEvent;
use vane::{SysResult, SystemExecutor};

use crate::config::Config;
use crate::{ClientId, NetworkId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(poll_new_players)
        .add_system(send_respawn_packets);
}

/// Polls for new clients and sends them the necessary packets
/// to join the game.
fn poll_new_players(game: &mut Game, server: &mut Server) -> SysResult {
    for client_id in server.accept_new_players() {
        accept_new_player(game, server, client_id)?;
    }
    Ok(())
}

fn accept_new_player(game: &mut Game, server: &mut Server, client_id: ClientId) -> SysResult {
    let config = game.resources.get::<Config>().unwrap().clone();
    let client = server.clients.get_mut(client_id).unwrap();
    let (player_data, world) = {
        let mut query = game.ecs.query::<(&WorldName, &WorldPath)>();
        let (world, (_, world_path)) = query
            .iter()
            .find(|(_, (name, _))| ***name == config.worlds.default_world)
            .unwrap();
        (
            world_path.load_player_data(client.uuid()),
            EntityWorld(world),
        )
    };
    let biomes = Arc::clone(&game.resources.get::<Arc<BiomeList>>().unwrap());

    let dimension = EntityDimension(
        player_data
            .as_ref()
            .map(|data| data.dimension.to_owned())
            .unwrap_or_else(|_| String::from("minecraft:overworld")), // TODO make it configurable
    );
    let position = player_data
        .as_ref()
        .map(|data| Position {
            x: data.animal.base.position[0],
            y: data.animal.base.position[1],
            z: data.animal.base.position[2],
            yaw: data.animal.base.rotation[0],
            pitch: data.animal.base.rotation[1],
        })
        .unwrap_or_default();

    let mut builder = game.create_entity_builder(
        player_data
            .as_ref()
            .map(|data| Position {
                x: data.animal.base.position[0],
                y: data.animal.base.position[1],
                z: data.animal.base.position[2],
                yaw: data.animal.base.rotation[0],
                pitch: data.animal.base.rotation[1],
            })
            .unwrap_or_default(),
        EntityKind::Player,
    );
    client.set_network_id(builder.get::<NetworkId>().unwrap());

    if player_data.is_err() {
        debug!("{} is a new player", client.username())
    }
    let gamemode = player_data
        .as_ref()
        .map(|data| Gamemode::from_id(data.gamemode as u8).expect("Unsupported gamemode"))
        .unwrap_or(config.server.default_gamemode);
    let previous_gamemode = player_data
        .as_ref()
        .map(|data| PreviousGamemode::from_id(data.previous_gamemode as i8))
        .unwrap_or_default();

    {
        let mut query = game.ecs.query::<(&WorldName, &Dimensions)>();
        let (_, (_, dimensions)) = query
            .iter()
            .find(|(_, (name, _))| ***name == config.worlds.default_world)
            .unwrap();
        client.send_join_game(
            gamemode,
            previous_gamemode,
            &dimensions,
            &*biomes,
            config.server.max_players as i32,
            dimension.clone(),
            world,
        );
    }
    client.send_brand();

    // Abilities
    let abilities = player_abilities_or_default(
        player_data.as_ref().map(|data| data.abilities.clone()).ok(),
        gamemode,
    );

    let hotbar_slot = player_data
        .as_ref()
        .map(|data| HotbarSlot::new(data.held_item as usize))
        .unwrap_or_else(|_e| HotbarSlot::new(0));

    let inventory = Inventory::player();
    let window = Window::new(BackingWindow::Player {
        player: inventory.new_handle(),
    });
    if let Ok(data) = player_data.as_ref() {
        for inventory_slot in data.inventory.iter() {
            let net_slot = inventory_slot.convert_index();
            let slot = match net_slot {
                Some(slot) => slot,
                None => {
                    log::error!("Failed to convert saved slot into network slot");
                    continue;
                }
            };

            window
                .set_item(
                    slot,
                    InventorySlot::Filled(
                        ItemStack::try_from(inventory_slot)
                            .expect("The player has an invalid item saved in their inventory"),
                    ),
                )
                .unwrap(); // This can't fail since the earlier match filters out all incorrect indexes.
        }
    }

    builder
        .add(client_id)
        .add(position)
        .add(View::new(
            position.chunk(),
            config.server.view_distance,
            world,
            dimension.clone(),
        ))
        .add(gamemode)
        .add(previous_gamemode)
        .add(components::Name::new(client.username()))
        .add(client.uuid())
        .add(client.profile().to_vec())
        .add(ChatBox::new(ChatPreference::All))
        .add(inventory)
        .add(window)
        .add(hotbar_slot)
        .add(dimension)
        .add(world)
        .add(Health(
            player_data
                .as_ref()
                .map(|data| data.animal.health)
                .unwrap_or(20.0),
        ))
        .add(abilities.walk_speed)
        .add(abilities.fly_speed)
        .add(abilities.is_flying)
        .add(abilities.may_fly)
        .add(abilities.may_build)
        .add(abilities.instabreak)
        .add(abilities.invulnerable);

    builder.add(GamemodeEvent(gamemode));

    game.spawn_entity(builder);

    broadcast_player_join(game, client.username());

    Ok(())
}

fn broadcast_player_join(game: &mut Game, username: &str) {
    let message = Text::translate_with("multiplayer.player.joined", vec![username.to_owned()]);
    game.broadcast_chat(ChatKind::System, message);
}

fn player_abilities_or_default(
    data: Option<PlayerAbilities>,
    gamemode: Gamemode,
) -> PlayerAbilities {
    data.unwrap_or(PlayerAbilities {
        walk_speed: WalkSpeed::default(),
        fly_speed: CreativeFlyingSpeed::default(),
        may_fly: CanCreativeFly(matches!(gamemode, Gamemode::Creative | Gamemode::Spectator)),
        is_flying: CreativeFlying(matches!(gamemode, Gamemode::Spectator)),
        may_build: CanBuild(!matches!(gamemode, Gamemode::Adventure)),
        instabreak: Instabreak(matches!(gamemode, Gamemode::Creative)),
        invulnerable: Invulnerable(matches!(gamemode, Gamemode::Creative | Gamemode::Spectator)),
    })
}

fn send_respawn_packets(game: &mut Game, server: &mut Server) -> SysResult {
    for (
        _,
        (
            _,
            client_id,
            walk_speed,
            fly_speed,
            may_fly,
            is_flying,
            may_build,
            instabreak,
            invulnerable,
            hotbar_slot,
            window,
        ),
    ) in game
        .ecs
        .query::<(
            &PlayerRespawnEvent,
            &ClientId,
            &WalkSpeed,
            &CreativeFlyingSpeed,
            &CanCreativeFly,
            &CreativeFlying,
            &CanBuild,
            &Instabreak,
            &Invulnerable,
            &HotbarSlot,
            &Window,
        )>()
        .iter()
    {
        let client = server.clients.get(*client_id).unwrap();
        client.send_abilities(&PlayerAbilities {
            walk_speed: *walk_speed,
            fly_speed: *fly_speed,
            may_fly: *may_fly,
            is_flying: *is_flying,
            may_build: *may_build,
            instabreak: *instabreak,
            invulnerable: *invulnerable,
        });
        client.send_hotbar_slot(hotbar_slot.get() as u8);
        client.send_window_items(&window);
    }
    Ok(())
}
