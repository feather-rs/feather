use anyhow::{anyhow, bail};
use base::{Area, Gamemode, Inventory, Position};
use common::{entities::player::HotbarSlot, window::BackingWindow, Window};
use ecs::{EntityRef, SysResult};
use protocol::packets::client::{ClickWindow, CreativeInventoryAction};

use crate::{ClientId, NetworkId, Server};

pub fn handle_creative_inventory_action(
    player: EntityRef,
    packet: CreativeInventoryAction,
    server: &mut Server,
) -> SysResult {
    if *player.get::<Gamemode>()? != Gamemode::Creative {
        bail!("cannot use Creative Inventory Action outside of creative mode");
    }

    if packet.slot != -1 {
        let window = player.get::<Window>()?;
        if !matches!(window.inner(), BackingWindow::Player { .. }) {
            bail!("cannot use Creative Inventory Action in external inventories");
        }

        window
            .inner()
            .set_item(packet.slot as usize, packet.clicked_item)?;

        // Sends the client updates about window changes.
        // Is required to make delete inventory button reflect in-game.
        let client_id = *player.get::<ClientId>()?;
        let client = server.clients.get(client_id).unwrap();
        client.send_window_items(&window);

        let visible_change =
            if let Some((_, area, slot)) = window.inner().index_to_slot(packet.slot as usize) {
                match area {
                    Area::Hotbar => {
                        let hotbar_slot = *player.get::<HotbarSlot>()?;
                        hotbar_slot.get() == slot
                    }
                    Area::Helmet => true,
                    Area::Chestplate => true,
                    Area::Leggings => true,
                    Area::Boots => true,
                    Area::Offhand => true,
                    _ => false,
                }
            } else {
                false
            };

        if visible_change {
            // Send an entity equipment packet to nearby players
            let position = *player.get::<Position>()?;
            let network_id = *player.get::<NetworkId>()?;
            let inventory = player.get::<Inventory>()?;
            let hotbar_slot = player.get::<HotbarSlot>()?;
            server.broadcast_nearby_with(position, |client| {
                client.send_entity_equipment(network_id, &inventory, &hotbar_slot);
            });
        }
    }

    Ok(())
}

pub fn handle_click_window(
    server: &mut Server,
    player: EntityRef,
    packet: ClickWindow,
) -> SysResult {
    let result = _handle_click_window(&player, &packet);

    let client = server.clients.get(*player.get::<ClientId>()?).unwrap();
    client.confirm_window_action(
        packet.window_id,
        packet.action_number as i16,
        result.is_ok(),
    );

    let window = player.get::<Window>()?;

    if packet.slot >= 0 {
        client.set_slot(packet.slot, &*window.item(packet.slot as usize)?);
    }
    client.set_cursor_slot(window.cursor_item());

    client.send_window_items(&*window);

    result
}

fn _handle_click_window(player: &EntityRef, packet: &ClickWindow) -> SysResult {
    let mut window = player.get_mut::<Window>()?;
    match packet.mode {
        0 => match packet.button {
            0 => window.left_click(packet.slot as usize)?,
            1 => window.right_click(packet.slot as usize)?,
            _ => bail!("unrecognized click"),
        },
        1 => window.shift_click(packet.slot as usize)?,
        5 => match packet.button {
            0 => window.begin_left_mouse_paint(),
            4 => window.begin_right_mouse_paint(),
            1 | 5 => window.add_paint_slot(packet.slot as usize)?,
            2 | 6 => window.end_paint()?,
            _ => bail!("unrecognized paint operation"),
        },
        _ => bail!("unsupported window click mode"),
    };

    Ok(())
}
