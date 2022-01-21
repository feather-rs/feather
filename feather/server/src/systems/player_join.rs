use libcraft_items::InventorySlot;
use log::debug;

use base::anvil::player::PlayerAbilities;
use base::{Gamemode, Inventory, ItemStack, Position, Text};
use common::{
    chat::{ChatKind, ChatPreference},
    entities::player::HotbarSlot,
    view::View,
    window::BackingWindow,
    ChatBox, Game, Window,
};
use ecs::{SysResult, SystemExecutor};
use quill_common::components::{
    CanBuild, CanCreativeFly, CreativeFlying, CreativeFlyingSpeed, Health, Instabreak,
    Invulnerable, PreviousGamemode, WalkSpeed,
};
use quill_common::events::GamemodeEvent;
use quill_common::{components::Name, entity_init::EntityInit};

use crate::{ClientId, NetworkId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(poll_new_players);
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
    let client = server.clients.get_mut(client_id).unwrap();
    let player_data = game.world.load_player_data(client.uuid());
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
        EntityInit::Player,
    );
    client.set_network_id(*builder.get::<NetworkId>().unwrap());

    if player_data.is_err() {
        debug!("{} is a new player", client.username())
    }
    let gamemode = player_data
        .as_ref()
        .map(|data| Gamemode::from_id(data.gamemode as u8).expect("Unsupported gamemode"))
        .unwrap_or(server.options.default_gamemode);
    let previous_gamemode = player_data
        .as_ref()
        .map(|data| PreviousGamemode::from_id(data.previous_gamemode as i8))
        .unwrap_or(PreviousGamemode(None));

    client.send_join_game(gamemode, previous_gamemode);
    client.send_brand();

    // Abilities
    let abilities = player_abilities_or_default(
        player_data.as_ref().map(|data| data.abilities.clone()).ok(),
        gamemode,
    );
    client.send_abilities(&abilities);

    let hotbar_slot = player_data
        .as_ref()
        .map(|data| HotbarSlot::new(data.held_item as usize))
        .unwrap_or_else(|_e| HotbarSlot::new(0));
    client.set_hotbar_slot(hotbar_slot.get() as u8);

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

            // This can't fail since the earlier match filters out all incorrect indexes.
            window
                .set_item(slot, InventorySlot::Filled(ItemStack::from(inventory_slot)))
                .unwrap();
        }
    }

    client.send_window_items(&window);

    builder
        .add(client_id)
        .add(View::new(
            Position::default().chunk(),
            server.options.view_distance,
        ))
        .add(gamemode)
        .add(previous_gamemode)
        .add(Name::new(client.username()))
        .add(client.uuid())
        .add(client.profile().to_vec())
        .add(ChatBox::new(ChatPreference::All))
        .add(inventory)
        .add(window)
        .add(hotbar_slot)
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
