use anyhow::bail;
use base::Gamemode;
use common::{window::BackingWindow, Window};
use ecs::{EntityRef, SysResult};
use protocol::packets::client::{ClickWindow, CreativeInventoryAction};

use crate::{ClientId, Server};

pub fn handle_creative_inventory_action(
    player: EntityRef,
    packet: CreativeInventoryAction,
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
        client.set_slot(packet.slot, window.item(packet.slot as usize)?.clone());
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
            _ => bail!("unrecgonized click"),
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

#[cfg(test)]
mod tests {
    use base::{Inventory, Item, ItemStack};
    use common::Game;

    use super::*;

    #[test]
    fn creative_inventory_action_survival_mode() {
        let mut game = Game::new();
        let entity = game.ecs.spawn((Gamemode::Survival, player_window()));
        let player = game.ecs.entity(entity).unwrap();

        let packet = CreativeInventoryAction {
            slot: 10,
            clicked_item: Some(ItemStack::new(Item::Diamond, 64)),
        };
        handle_creative_inventory_action(player, packet).unwrap_err();

        assert!(game
            .ecs
            .get::<Window>(entity)
            .unwrap()
            .item(10)
            .unwrap()
            .is_none());
    }

    #[test]
    fn creative_inventory_action_non_player_window() {
        let mut game = Game::new();
        let entity = game.ecs.spawn((
            Window::new(BackingWindow::Generic9x3 {
                player: Inventory::player(),
                block: Inventory::chest(),
            }),
            Gamemode::Creative,
        ));
        let player = game.ecs.entity(entity).unwrap();

        let packet = CreativeInventoryAction {
            slot: 5,
            clicked_item: Some(ItemStack::new(Item::Diamond, 64)),
        };
        handle_creative_inventory_action(player, packet).unwrap_err();

        assert!(game
            .ecs
            .get::<Window>(entity)
            .unwrap()
            .item(5)
            .unwrap()
            .is_none());
    }

    #[test]
    fn creative_inventory_action() {
        let mut game = Game::new();
        let entity = game.ecs.spawn((Gamemode::Creative, player_window()));
        let player = game.ecs.entity(entity).unwrap();

        let packet = CreativeInventoryAction {
            slot: 5,
            clicked_item: Some(ItemStack::new(Item::Diamond, 64)),
        };
        handle_creative_inventory_action(player, packet).unwrap();

        assert_eq!(
            game.ecs
                .get::<Window>(entity)
                .unwrap()
                .item(5)
                .unwrap()
                .clone(),
            Some(ItemStack::new(Item::Diamond, 64))
        );
    }

    fn player_window() -> Window {
        Window::new(BackingWindow::Player {
            player: Inventory::player(),
        })
    }
}
