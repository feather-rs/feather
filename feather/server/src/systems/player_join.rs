use std::sync::Arc;

use common::entities::player::PlayerProfile;

use common::events::PlayerRespawnEvent;
use common::{entities::player::HotbarSlot, view::View, window::BackingWindow, Game, PlayerWindow};
use libcraft::anvil::player::PlayerAbilities;
use libcraft::biome::BiomeList;
use libcraft::EntityKind;
use libcraft::{Gamemode, Inventory, Position, Text};
use quill::chat::{ChatKind, ChatPreference};
use quill::components::{self, EntityInventory, EntityPosition, EntityUuid, PlayerGamemode};
use quill::components::{
    CanBuild, CanCreativeFly, CreativeFlying, CreativeFlyingSpeed, EntityWorld, Health, Instabreak,
    Invulnerable, PreviousGamemode, WalkSpeed,
};
use quill::events::GamemodeEvent;
use quill::ChatBox;
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
    let biomes = game.resources.get::<Arc<BiomeList>>()?.clone();

    let config = game.resources.get::<Config>().unwrap().clone();
    let client = server.clients.get_mut(client_id).unwrap();

    // TODO: player data loading

    let position = Position::default();
    let mut builder = game.create_entity_builder(position, EntityKind::Player);
    client.set_network_id(builder.get::<NetworkId>().unwrap());

    let world = game.default_world();

    let gamemode = config.server.default_gamemode;
    let previous_gamemode = PreviousGamemode(Some(gamemode));

    client.send_join_game(
        gamemode,
        previous_gamemode,
        &biomes,
        config.server.max_players as i32,
        &*world,
    );
    client.send_brand();

    let abilities = player_abilities_or_default(None, gamemode);
    let hotbar_slot = HotbarSlot::new(0);

    let inventory = Inventory::player();
    let window = PlayerWindow::new(BackingWindow::Player {
        player: inventory.new_handle(),
    });

    builder
        .add(client_id)
        .add(EntityPosition(position))
        .add(View::new(
            position.chunk(),
            config.server.view_distance,
            world.id(),
        ))
        .add(PlayerGamemode(gamemode))
        .add(previous_gamemode)
        .add(components::Name::new(client.username()))
        .add(EntityUuid(client.uuid()))
        .add(PlayerProfile(client.profile().to_vec()))
        .add(ChatBox::new(ChatPreference::All))
        .add(EntityInventory::new(inventory))
        .add(window)
        .add(hotbar_slot)
        .add(EntityWorld(world.id()))
        .add(Health(20.))
        .add(WalkSpeed(abilities.walk_speed))
        .add(CreativeFlyingSpeed(abilities.fly_speed))
        .add(CreativeFlying(abilities.is_flying))
        .add(CanCreativeFly(abilities.may_fly))
        .add(CanBuild(abilities.may_build))
        .add(Instabreak(abilities.instabreak))
        .add(Invulnerable(abilities.invulnerable));

    drop(world);

    let entity = game.spawn_entity(builder);

    game.ecs
        .insert_entity_event(entity, GamemodeEvent(gamemode))?;

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
        walk_speed: WalkSpeed::default().0,
        fly_speed: CreativeFlyingSpeed::default().0,
        may_fly: matches!(gamemode, Gamemode::Creative | Gamemode::Spectator),
        is_flying: matches!(gamemode, Gamemode::Spectator),
        may_build: !matches!(gamemode, Gamemode::Adventure),
        instabreak: matches!(gamemode, Gamemode::Creative),
        invulnerable: matches!(gamemode, Gamemode::Creative | Gamemode::Spectator),
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
            &PlayerWindow,
        )>()
        .iter()
    {
        let client = server.clients.get(*client_id).unwrap();
        client.send_abilities(&PlayerAbilities {
            walk_speed: walk_speed.0,
            fly_speed: fly_speed.0,
            may_fly: may_fly.0,
            is_flying: is_flying.0,
            may_build: may_build.0,
            instabreak: instabreak.0,
            invulnerable: invulnerable.0,
        });
        client.send_hotbar_slot(hotbar_slot.get() as u8);
        client.send_window_items(&window);
    }
    Ok(())
}
