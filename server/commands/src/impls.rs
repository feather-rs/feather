//! The implementations of various commands.

use crate::arguments::Coordinates;
use crate::{
    arguments::{EntitySelector, ItemArgument, ParsedGamemode, PositiveI32Argument, TextArgument},
    CommandCtx,
};
use feather_core::inventory::{Inventory, SlotIndex};
use feather_core::text::{Text, TextComponentBuilder, TextValue};
use feather_core::util::{Gamemode, Position};
use feather_definitions::Item;
use feather_server_types::{
    ChatEvent, ChatPosition, GamemodeUpdateEvent, InventoryUpdateEvent, MessageReceiver, Name,
    Player, ShutdownChannels, Teleported,
};
use fecs::{Entity, ResourcesProvider, World};
use lieutenant::command;
use smallvec::SmallVec;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TpError {
    #[error("No entity was found")]
    NoMatchingEntities,
    #[error("Only one entity is allowed, but the provided selector allows for more than one")]
    TooManyEntities,
}

#[command(usage = "tp|teleport <destination>")]
pub fn tp_1(ctx: &mut CommandCtx, destination: EntitySelector) -> anyhow::Result<()> {
    if let Some(first) = destination.entities.first() {
        if let Some(pos) = ctx.world.try_get::<Position>(*first).map(|r| *r) {
            teleport_entity_to_pos(&mut ctx.world, ctx.sender, pos);
        }

        Ok(Some(format!(
            "Teleported {0} to {1}",
            ctx.world.get::<Name>(ctx.sender).0.to_string(),
            ctx.world.get::<Name>(*first).0.to_string()
        )))
    } else {
        Err(TpError::NoMatchingEntities.into())
    }
}

#[command(usage = "tp|teleport <location>")]
pub fn tp_2(ctx: &mut CommandCtx, location: Coordinates) -> anyhow::Result<()> {
    teleport_entity(&mut ctx.world, ctx.sender, location);

    let position = ctx.world.get::<Position>(ctx.sender);
    Ok(Some(format!(
        "Teleported {0} to {1}, {2}, {3}",
        ctx.world.get::<Name>(ctx.sender).0,
        position.x,
        position.y,
        position.z
    )))
}

#[command(usage = "tp|teleport <targets> <location>")]
pub fn tp_3(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    location: Coordinates,
) -> anyhow::Result<()> {
    if targets.entities.is_empty() {
        Err(TpError::NoMatchingEntities.into())
    } else {
        for entity in &targets.entities {
            teleport_entity(&mut ctx.world, *entity, location);
        }

        let position = ctx
            .world
            .get::<Position>(*targets.entities.first().unwrap());
        Ok(Some(format!(
            "Teleported {0} to {1}, {2}, {3}",
            targets.entities_to_string(ctx, false),
            position.x,
            position.y,
            position.z
        )))
    }
}

#[command(usage = "tp|teleport <targets> <destination>")]
pub fn tp_4(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    destination: EntitySelector,
) -> anyhow::Result<()> {
    if destination.entities.len() > 1 {
        Err(TpError::TooManyEntities.into())
    } else if let Some(location) = destination
        .entities
        .first()
        .map(|e| ctx.world.try_get::<Position>(*e).map(|r| *r))
        .flatten()
    {
        if targets.entities.is_empty() {
            Err(TpError::NoMatchingEntities.into())
        } else {
            for entity in &targets.entities {
                teleport_entity_to_pos(&mut ctx.world, *entity, location);
            }
            Ok(Some(format!(
                "Teleported {0} to {1}",
                targets.entities_to_string(ctx, false),
                destination.entities_to_string(ctx, false)
            )))
        }
    } else {
        Err(TpError::NoMatchingEntities.into())
    }
}

fn teleport_entity(world: &mut World, entity: Entity, location: Coordinates) {
    let new_pos = world
        .try_get::<Position>(entity)
        .map(|r| *r)
        .map(|relative_to| location.into_position(relative_to));

    if let Some(new_pos) = new_pos {
        teleport_entity_to_pos(world, entity, new_pos);
    }
}

fn teleport_entity_to_pos(world: &mut World, entity: Entity, pos: Position) {
    if let Some(mut old_pos) = world.try_get_mut::<Position>(entity) {
        *old_pos = pos;
    }
    let _ = world.add(entity, Teleported);
}

#[command(usage = "gamemode <gamemode>")]
pub fn gamemode_1(ctx: &mut CommandCtx, gamemode: ParsedGamemode) -> anyhow::Result<()> {
    update_gamemode(ctx, gamemode.0, ctx.sender);
    Ok(Some(format!(
        "Set own gamemode to {} Mode",
        gamemode.0.to_string()
    )))
}

#[command(usage = "gamemode <gamemode> <target>")]
pub fn gamemode_2(
    ctx: &mut CommandCtx,
    gamemode: ParsedGamemode,
    target: EntitySelector,
) -> anyhow::Result<()> {
    for entity in &target.entities {
        update_gamemode(ctx, gamemode.0, *entity)
    }

    if target.entities.len() == 1 && *target.entities.first().unwrap() == ctx.sender {
        return Ok(Some(format!(
            "Set own gamemode to {} Mode",
            gamemode.0.to_string()
        )));
    }
    Ok(Some(format!(
        "Changed gamemode of {} to {} Mode",
        target.entities_to_string(ctx, false),
        gamemode.0.to_string()
    )))
}

fn update_gamemode(ctx: &mut CommandCtx, gamemode: Gamemode, entity: Entity) {
    let event = if let Some(mut old) = ctx.world.try_get_mut::<Gamemode>(ctx.sender) {
        let old_val = *old;
        *old = gamemode;

        let event = GamemodeUpdateEvent {
            player: entity,
            old: old_val,
            new: gamemode,
        };
        Some(event)
    } else {
        None
    };

    if let Some(event) = event {
        ctx.game.handle(&mut *ctx.world, event);
    }
}

#[command(usage = "tell|msg|w <target> <message>")]
pub fn whisper(
    ctx: &mut CommandCtx,
    target: EntitySelector,
    message: TextArgument,
) -> anyhow::Result<()> {
    let sender_name = if let Some(sender_name) = ctx.world.try_get::<Name>(ctx.sender) {
        sender_name.0.clone()
    } else {
        // Use a default value if the executor has no Name component
        String::from("Server")
    };

    // The message that is returned to the whisperer
    // You whisper to [player] (and [player]): [message]
    let mut response_message = String::from("You whisper to");

    // Tracks if there needs to be "and" before the next player added to the response message
    let mut needs_and = false;

    for entity in target.entities {
        if let Some(mut message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender) {
            message_receiver.send(
                Text::from(format!(
                    "{} whispers to you: {}",
                    sender_name,
                    message.0.clone()
                ))
                .gray()
                .italic(),
            );
        } else {
            // If the entity doesn't have a message receiver it is not a player and there is no need to continue
            continue;
        };

        if let Some(player_name) = ctx.world.try_get::<Name>(entity) {
            if needs_and {
                response_message += format!(" and {}", player_name.0).as_str();
            } else {
                needs_and = true;

                response_message += format!(" {}", player_name.0).as_str();
            }
        }
    }

    // Send the whisperer a confirmation message
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        response_message += format!(": {}", message.0).as_str();
        let return_text = Text::from(response_message).gray().italic();

        sender_message_receiver.send(return_text);
    }

    Ok(None)
}

#[command(usage = "say <message>")]
pub fn say(ctx: &mut CommandCtx, message: TextArgument) -> anyhow::Result<()> {
    let name = ctx.world.try_get::<Name>(ctx.sender);

    let sender_name = if let Some(name) = &name {
        &name.0
    } else {
        "Server"
    };

    let command_output = Text::from(format!("[{}] {}", sender_name, message.0));

    drop(name);

    ctx.game.handle(
        &mut ctx.world,
        ChatEvent {
            message: command_output.into(),
            position: ChatPosition::Chat,
        },
    );

    Ok(None)
}

#[command(usage = "me <action>")]
pub fn me(ctx: &mut CommandCtx, action: TextArgument) -> anyhow::Result<()> {
    let command_output = {
        let name = ctx.world.try_get::<Name>(ctx.sender);
        let sender_name = name.as_deref().map_or("@", |Name(n)| n);
        Text::from(format!("* {} {}", sender_name, action.as_ref()))
    };

    ctx.game.handle(
        &mut ctx.world,
        ChatEvent {
            message: command_output.into(),
            position: ChatPosition::Chat,
        },
    );

    Ok(None)
}

#[derive(Debug, Error)]
pub enum KickError {
    #[error(
        "Only players may be affected by this command, but the provided selector includes entities"
    )]
    NoEntities,
}

#[command(usage = "kick <targets>")]
pub fn kick_1(ctx: &mut CommandCtx, targets: EntitySelector) -> anyhow::Result<()> {
    kick_players(
        ctx,
        &targets,
        TextValue::translate("multiplayer.disconnect.kicked").into(),
    )
}

#[command(usage = "kick <targets> <reason>")]
pub fn kick_2(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    reason: TextArgument,
) -> anyhow::Result<()> {
    kick_players(ctx, &targets, reason.0.into())
}

fn kick_players(
    ctx: &mut CommandCtx,
    targets: &EntitySelector,
    reason: Text,
) -> anyhow::Result<Option<String>> {
    for entity in &targets.entities {
        if ctx.world.try_get::<Player>(*entity).is_none() {
            return Err(KickError::NoEntities.into());
        }
    }

    for entity in &targets.entities {
        let name = ctx.world.get::<Name>(*entity).0.clone();
        ctx.game
            .disconnect_and_log(*entity, &mut ctx.world, &reason, "player kicked");

        // Send confirmation message
        // TODO Server ops should also see the message
        if let Some(mut sender_message_receiver) =
            ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
        {
            let kick_confirm = Text::from(TextValue::translate_with(
                "commands.kick.success",
                vec![Text::from(name), reason.clone()],
            ));
            sender_message_receiver.send(kick_confirm);
        }
    }
    Ok(None)
}

#[command(usage = "stop")]
pub fn stop(ctx: &mut CommandCtx) -> anyhow::Result<()> {
    // Confirmation message
    // TODO Server ops should also see the message
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let text = Text::from(TextValue::translate("commands.stop.stopping"));
        sender_message_receiver.send(text);
    }

    ctx.game
        .resources
        .get::<ShutdownChannels>()
        .tx
        .try_send(())?;

    Ok(None)
}

#[derive(Debug, Error)]
pub enum ClearError {
    #[error("command has to be run from a player")]
    NotPlayer,
    #[error("No items were found on player {0}")]
    NoItems(String),
    #[error("No items were found on {0}")]
    NoItemsMultiplayer(String),
    #[error(
        "Only players may be affected by this command, but the provided selector includes entities"
    )]
    NoEntities,
}

#[command(usage = "clear")]
pub fn clear_1(ctx: &mut CommandCtx) -> anyhow::Result<()> {
    if ctx.world.try_get::<Player>(ctx.sender).is_some() {
        // Go through the player's inventory and set all the slots to no items.
        // Also, keep track of how many items we delete.
        let mut count = 0;
        clear_items(ctx, ctx.sender, None, i32::MAX, &mut count);
        // If count is zero, the player's inventory was empty and the command fails
        // "No items were found on player {0}."
        if count == 0 {
            let name = ctx.world.get::<Name>(ctx.sender);
            return Err(ClearError::NoItems(name.0.clone()).into());
        }
        // If the count is not zero, we return the count of items we deleted. Command succeeds.
        // "Removed {1} items from player {0}"
        Ok(Some(format!(
            "Removed {1} items from player {0}",
            ctx.world.get::<Name>(ctx.sender).0,
            count
        )))
    } else {
        Err(ClearError::NotPlayer.into())
    }
}

#[command(usage = "clear <targets>")]
pub fn clear_2(ctx: &mut CommandCtx, targets: EntitySelector) -> anyhow::Result<()> {
    let mut players = true;
    for entity in &targets.entities {
        players &= ctx.world.try_get::<Player>(*entity).is_some();
    }
    if players {
        let mut count = 0;
        for entity in &targets.entities {
            clear_items(ctx, *entity, None, i32::MAX, &mut count);
        }
        // If count is zero, the everybody's inventory was empty and the command fails
        // "No items were found on {0} players."
        if count == 0 {
            return Err(
                ClearError::NoItemsMultiplayer(targets.entities_to_string(ctx, true)).into(),
            );
        }
        // If the count is not zero, we return the count of items we deleted. Command succeeds.
        // "Removed {1} items from {0} players"
        Ok(Some(format!(
            "Removed {1} items from {0}",
            targets.entities_to_string(ctx, true),
            count
        )))
    } else {
        Err(ClearError::NoEntities.into())
    }
}

#[command(usage = "clear <targets> <item>")]
pub fn clear_3(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    item: ItemArgument,
) -> anyhow::Result<()> {
    let mut players = true;
    for entity in &targets.entities {
        players &= ctx.world.try_get::<Player>(*entity).is_some();
    }
    if players {
        let mut count = 0;
        for entity in &targets.entities {
            clear_items(ctx, *entity, Some(item.0), i32::MAX, &mut count);
        }
        // If count is zero, the everybody's inventory was empty and the command fails
        // "No items were found on {0} players."
        if count == 0 {
            return Err(
                ClearError::NoItemsMultiplayer(targets.entities_to_string(ctx, true)).into(),
            );
        }
        // If the count is not zero, we return the count of items we deleted. Command succeeds.
        // "Removed {1} items from {0} players"
        Ok(Some(format!(
            "Removed {1} items from {0}",
            targets.entities_to_string(ctx, true),
            count
        )))
    } else {
        Err(ClearError::NoEntities.into())
    }
}

#[command(usage = "clear <targets> <item> <maxcount>")]
pub fn clear_4(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    item: ItemArgument,
    maxcount: PositiveI32Argument,
) -> anyhow::Result<()> {
    let mut players = true;
    for entity in &targets.entities {
        players &= ctx.world.try_get::<Player>(*entity).is_some();
    }
    if players {
        let mut count = 0;
        for entity in &targets.entities {
            clear_items(ctx, *entity, Some(item.0), maxcount.0, &mut count);
        }
        // If count is zero, the everybody's inventory was empty and the command fails
        // "No items were found on {0} players."
        if count == 0 {
            return Err(
                ClearError::NoItemsMultiplayer(targets.entities_to_string(ctx, true)).into(),
            );
        }
        // If maxcount is 0, we report not that we removed items, only that we found them.
        if maxcount.0 == 0 {
            Ok(Some(format!(
                "Found {1} matching items on {0}",
                targets.entities_to_string(ctx, true),
                count
            )))
        } else {
            // If the count is not zero, we return the count of items we deleted. Command succeeds.
            // "Removed {1} items from {0} players"
            Ok(Some(format!(
                "Removed {1} items from {0}",
                targets.entities_to_string(ctx, true),
                count
            )))
        }
    } else {
        Err(ClearError::NoEntities.into())
    }
}

/// Go through a player's inventory and set all the slots that match "item" to empty, up to maxcount items removed.
/// Also, keep track of how many items we delete total in the variable count.
/// Will panic if entity does not have an inventory
fn clear_items(
    ctx: &mut CommandCtx,
    player: Entity,
    item: Option<Item>,
    maxcount: i32,
    count: &mut i32,
) {
    let inventory = ctx.world.get_mut::<Inventory>(player);
    let mut changed_items: SmallVec<[SlotIndex; 2]> = SmallVec::new();
    for (index, slot) in inventory.enumerate() {
        if let Some(mut stack) = slot {
            if let Some(item_inner) = item {
                if stack.ty != item_inner {
                    continue;
                }
            }
            if maxcount == 0 {
                *count += stack.amount as i32;
            } else if (stack.amount as i32) <= maxcount - *count {
                *count += stack.amount as i32;
                inventory.remove_item_at(index.area, index.slot).unwrap();
                changed_items.push(index);
            } else {
                stack.amount -= (maxcount - *count) as u8;
                inventory
                    .set_item_at(index.area, index.slot, stack)
                    .unwrap();
                *count = maxcount;
                changed_items.push(index);
                break;
            }
        }
    }

    drop(inventory);

    if !changed_items.is_empty() {
        ctx.game.handle(
            &mut *ctx.world,
            InventoryUpdateEvent {
                entity: player,
                slots: changed_items,
            },
        );
    }
}

#[command(usage = "seed")]
pub fn seed(ctx: &mut CommandCtx) -> anyhow::Result<()> {
    Ok(Some(format!("Seed: [{}]", ctx.game.level.seed.to_string())))
}
