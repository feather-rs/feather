use base::{Area, Gamemode, Inventory, Item, ItemStack, Position, Text};
use common::{
    chat::{ChatKind, ChatPreference},
    view::View,
    window::BackingWindow,
    ChatBox, Game, Window,
};
use ecs::{ SysResult, SystemExecutor};
use quill_common::{components::Name, entity_init::EntityInit};

use crate::{ClientId, Server};

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
    let client = server.clients.get(client_id).unwrap();
    client.send_join_game(Gamemode::Creative);
    client.send_brand();

    let mut builder = game.create_entity_builder(Position::default(), EntityInit::Player);

    let inventory = Inventory::player();
    let window = Window::new(BackingWindow::Player {
        player: inventory.new_handle(),
    });

    *inventory.item(Area::Hotbar, 0).unwrap() = Some(ItemStack::new(Item::Diamond, 64));
    *inventory.item(Area::Hotbar, 1).unwrap() = Some(ItemStack::new(Item::OakWood, 64));
    *inventory.item(Area::Hotbar, 2).unwrap() = Some(ItemStack::new(Item::GoldenSword, 1));

    client.send_window_items(&window);

    builder
        .add(client.network_id())
        .add(client_id)
        .add(View::new(
            Position::default().chunk(),
            server.options.view_distance,
        ))
        .add(Gamemode::Creative)
        .add(Name::new(client.username()))
        .add(client.uuid())
        .add(client.profile().to_vec())
        .add(ChatBox::new(ChatPreference::All))
        .add(inventory)
        .add(window);

    game.spawn_entity(builder);

    broadcast_player_join(game, client.username());

    Ok(())
}

fn broadcast_player_join(game: &mut Game, username: &str) {
    let message = Text::translate_with("multiplayer.player.joined", vec![username.to_owned()]);
    game.broadcast_chat(ChatKind::System, message);
}