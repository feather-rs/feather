use base::inventory::{SLOT_HOTBAR_OFFSET, SLOT_INVENTORY_OFFSET};
use base::{Area, Gamemode, Inventory, Item, ItemStack, Position, Text};
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

    let player_data = game.world.load_player_data(client.uuid());
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
    let walk_speed = player_data
        .as_ref()
        .map(|data| WalkSpeed(data.abilities.walk_speed))
        .unwrap_or_default();
    let fly_speed = player_data
        .as_ref()
        .map(|data| CreativeFlyingSpeed(data.abilities.fly_speed))
        .unwrap_or_default();
    let is_flying = CreativeFlying(
        player_data
            .as_ref()
            .map(|data| data.abilities.flying)
            .unwrap_or(gamemode == Gamemode::Spectator),
    );
    let can_fly = CanCreativeFly(
        player_data
            .as_ref()
            .map(|data| data.abilities.may_fly)
            .unwrap_or(gamemode == Gamemode::Creative || gamemode == Gamemode::Spectator),
    );
    let can_build = CanBuild(
        player_data
            .as_ref()
            .map(|data| data.abilities.may_build)
            .unwrap_or(gamemode == Gamemode::Creative || gamemode == Gamemode::Survival),
    );
    let instabreak = Instabreak(
        player_data
            .as_ref()
            .map(|data| data.abilities.instabuild)
            .unwrap_or(gamemode == Gamemode::Creative),
    );
    let invulnerable = Invulnerable(
        player_data
            .as_ref()
            .map(|data| data.abilities.invulnerable)
            .unwrap_or(gamemode == Gamemode::Creative || gamemode == Gamemode::Spectator),
    );
    client.send_abilities(
        invulnerable,
        is_flying,
        can_fly,
        instabreak,
        fly_speed,
        walk_speed,
    );

    let hotbar_slot = player_data
        .as_ref()
        .map(|data| HotbarSlot::new(data.held_item as usize))
        .unwrap_or_default();
    client.set_hotbar_slot(hotbar_slot.get() as u8);

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

    let inventory = Inventory::player();
    if let Ok(data) = player_data.as_ref() {
        for slot in data.inventory.iter() {
            let slot_num = slot.slot as usize;
            if let Some(item) = Item::from_name(&slot.item) {
                let item_stack = if slot_num > SLOT_HOTBAR_OFFSET {
                    inventory.item(Area::Hotbar, slot_num - SLOT_HOTBAR_OFFSET)
                } else if slot_num > SLOT_INVENTORY_OFFSET {
                    inventory.item(Area::Storage, slot_num - SLOT_INVENTORY_OFFSET)
                } else {
                    None
                };
                if let Some(mut item_stack) = item_stack {
                    *item_stack = Some(ItemStack {
                        item,
                        count: slot.count as u32,
                        damage: slot
                            .nbt
                            .as_ref()
                            .map(|nbt| nbt.damage.map(|damage| damage as u32))
                            .unwrap_or(None),
                    });
                }
            }
        }
    }
    let window = Window::new(BackingWindow::Player {
        player: inventory.new_handle(),
    });

    client.send_window_items(&window);

    builder
        .add(client.network_id())
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
        .add(walk_speed)
        .add(fly_speed)
        .add(is_flying)
        .add(can_fly)
        .add(can_build)
        .add(instabreak)
        .add(invulnerable);

    game.spawn_entity(builder);

    broadcast_player_join(game, client.username());

    Ok(())
}

fn broadcast_player_join(game: &mut Game, username: &str) {
    let message = Text::translate_with("multiplayer.player.joined", vec![username.to_owned()]);
    game.broadcast_chat(ChatKind::System, message);
}
