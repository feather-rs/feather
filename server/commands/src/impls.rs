//! The implementations of various commands.
#![allow(non_snake_case)]



use crate::{
    CommandCtx,
    arguments::*
};


//use thiserror::Error;
use lieutenant::command;

//use feather_core::util::{Gamemode, Position};
//use feather_core::inventory::{Inventory, SlotIndex};
//use feather_core::text::TextValue;
use feather_core::text::{Text, TextComponentBuilder};
//use feather_definitions::Item;
use feather_server_types::MessageReceiver;
/*
use feather_server_types::{
    ChatEvent, ChatPosition, GamemodeUpdateEvent, InventoryUpdateEvent, Name,
    Player, ShutdownChannels, Teleported,
};
*/
//use fecs::{Entity, ResourcesProvider, World};
//use smallvec::SmallVec;



/*
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
    _targets: EntitySelector,
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
    _targets: EntitySelector,
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
    _target: EntitySelector,
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
    _target: EntitySelector,
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
    _targets: EntitySelector,
    _item: ItemArgument,
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
    _targets: EntitySelector,
    _item: ItemArgument,
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
    _item: Option<Item>,
    maxcount: i32,
    _count: &mut i32,
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
*/

#[command(usage="advancement grant <targets> everything")]
pub fn advancement_grant_targets_everything(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement grant <targets> everything\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement grant <targets> from <advancement>")]
pub fn advancement_grant_targets_from_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement grant <targets> from <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement grant <targets> only <advancement>")]
pub fn advancement_grant_targets_only_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement grant <targets> only <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement grant <targets> only <advancement> <criterion>")]
pub fn advancement_grant_targets_only_advancement_criterion(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation,
    _criterion:StringArgumentGreedy
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement grant <targets> only <advancement> <criterion>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement grant <targets> through <advancement>")]
pub fn advancement_grant_targets_through_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement grant <targets> through <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement grant <targets> until <advancement>")]
pub fn advancement_grant_targets_until_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement grant <targets> until <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement revoke <targets> everything")]
pub fn advancement_revoke_targets_everything(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement revoke <targets> everything\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement revoke <targets> from <advancement>")]
pub fn advancement_revoke_targets_from_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement revoke <targets> from <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement revoke <targets> only <advancement>")]
pub fn advancement_revoke_targets_only_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement revoke <targets> only <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement revoke <targets> only <advancement> <criterion>")]
pub fn advancement_revoke_targets_only_advancement_criterion(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation,
    _criterion:StringArgumentGreedy
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement revoke <targets> only <advancement> <criterion>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement revoke <targets> through <advancement>")]
pub fn advancement_revoke_targets_through_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement revoke <targets> through <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="advancement revoke <targets> until <advancement>")]
pub fn advancement_revoke_targets_until_advancement(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _advancement:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"advancement revoke <targets> until <advancement>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> base get")]
pub fn attribute_target_attribute_base_get(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> base get\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> base get <scale>")]
pub fn attribute_target_attribute_base_get_scale(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _scale:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> base get <scale>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> base set <value>")]
pub fn attribute_target_attribute_base_set_value(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _value:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> base set <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> get")]
pub fn attribute_target_attribute_get(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> get\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> get <scale>")]
pub fn attribute_target_attribute_get_scale(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _scale:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> get <scale>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> modifier add <uuid> <name> <value> add")]
pub fn attribute_target_attribute_modifier_add_uuid_name_value_add(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _uuid:Uuid,
    _name:StringArgumentPhrase,
    _value:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> modifier add <uuid> <name> <value> add\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> modifier add <uuid> <name> <value> multiply")]
pub fn attribute_target_attribute_modifier_add_uuid_name_value_multiply(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _uuid:Uuid,
    _name:StringArgumentPhrase,
    _value:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> modifier add <uuid> <name> <value> multiply\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> modifier add <uuid> <name> <value> multiply_base")]
pub fn attribute_target_attribute_modifier_add_uuid_name_value_multiply_base(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _uuid:Uuid,
    _name:StringArgumentPhrase,
    _value:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> modifier add <uuid> <name> <value> multiply_base\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> modifier remove <uuid>")]
pub fn attribute_target_attribute_modifier_remove_uuid(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _uuid:Uuid
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> modifier remove <uuid>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> modifier value get <uuid>")]
pub fn attribute_target_attribute_modifier_value_get_uuid(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _uuid:Uuid
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> modifier value get <uuid>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="attribute <target> <attribute> modifier value get <uuid> <scale>")]
pub fn attribute_target_attribute_modifier_value_get_uuid_scale(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _attribute:ResourceLocation,
    _uuid:Uuid,
    _scale:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"attribute <target> <attribute> modifier value get <uuid> <scale>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="ban <targets>")]
pub fn ban_targets(
    ctx: &mut CommandCtx,
    _targets:GameProfile
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"ban <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="ban <targets> <reason>")]
pub fn ban_targets_reason(
    ctx: &mut CommandCtx,
    _targets:GameProfile,
    _reason:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"ban <targets> <reason>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="ban-ip <target>")]
pub fn ban_ip_target(
    ctx: &mut CommandCtx,
    _target:StringArgumentWord
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"ban-ip <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="ban-ip <target> <reason>")]
pub fn ban_ip_target_reason(
    ctx: &mut CommandCtx,
    _target:StringArgumentWord,
    _reason:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"ban-ip <target> <reason>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="banlist")]
pub fn banlist(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"banlist\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="banlist ips")]
pub fn banlist_ips(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"banlist ips\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="banlist players")]
pub fn banlist_players(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"banlist players\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar add <id> <name>")]
pub fn bossbar_add_id_name(
    ctx: &mut CommandCtx,
    _id:ResourceLocation,
    _name:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar add <id> <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar get <id> max")]
pub fn bossbar_get_id_max(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar get <id> max\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar get <id> players")]
pub fn bossbar_get_id_players(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar get <id> players\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar get <id> value")]
pub fn bossbar_get_id_value(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar get <id> value\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar get <id> visible")]
pub fn bossbar_get_id_visible(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar get <id> visible\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar list")]
pub fn bossbar_list(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar remove <id>")]
pub fn bossbar_remove_id(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar remove <id>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> color blue")]
pub fn bossbar_set_id_color_blue(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> color blue\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> color green")]
pub fn bossbar_set_id_color_green(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> color green\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> color pink")]
pub fn bossbar_set_id_color_pink(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> color pink\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> color purple")]
pub fn bossbar_set_id_color_purple(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> color purple\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> color red")]
pub fn bossbar_set_id_color_red(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> color red\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> color white")]
pub fn bossbar_set_id_color_white(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> color white\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> color yellow")]
pub fn bossbar_set_id_color_yellow(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> color yellow\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> max <max>")]
pub fn bossbar_set_id_max_max(
    ctx: &mut CommandCtx,
    _id:ResourceLocation,
    _max:IntegerArgumentGreaterThen1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> max <max>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> name <name>")]
pub fn bossbar_set_id_name_name(
    ctx: &mut CommandCtx,
    _id:ResourceLocation,
    _name:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> name <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> players")]
pub fn bossbar_set_id_players(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> players\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> players <targets>")]
pub fn bossbar_set_id_players_targets(
    ctx: &mut CommandCtx,
    _id:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> players <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> style notched_10")]
pub fn bossbar_set_id_style_notched_10(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> style notched_10\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> style notched_12")]
pub fn bossbar_set_id_style_notched_12(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> style notched_12\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> style notched_20")]
pub fn bossbar_set_id_style_notched_20(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> style notched_20\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> style notched_6")]
pub fn bossbar_set_id_style_notched_6(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> style notched_6\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> style progress")]
pub fn bossbar_set_id_style_progress(
    ctx: &mut CommandCtx,
    _id:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> style progress\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> value <value>")]
pub fn bossbar_set_id_value_value(
    ctx: &mut CommandCtx,
    _id:ResourceLocation,
    _value:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="bossbar set <id> visible <visible>")]
pub fn bossbar_set_id_visible_visible(
    ctx: &mut CommandCtx,
    _id:ResourceLocation,
    _visible:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"bossbar set <id> visible <visible>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clear")]
pub fn clear(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clear\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clear <targets>")]
pub fn clear_targets(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clear <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clear <targets> <item>")]
pub fn clear_targets_item(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _item:Predicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clear <targets> <item>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clear <targets> <item> <maxCount>")]
pub fn clear_targets_item_maxCount(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _item:Predicate,
    _maxCount:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clear <targets> <item> <maxCount>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination>")]
pub fn clone_begin_end_destination(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> filtered <filter>")]
pub fn clone_begin_end_destination_filtered_filter(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates,
    _filter:BlockPredicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> filtered <filter>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> filtered <filter> force")]
pub fn clone_begin_end_destination_filtered_filter_force(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates,
    _filter:BlockPredicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> filtered <filter> force\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> filtered <filter> move")]
pub fn clone_begin_end_destination_filtered_filter_move(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates,
    _filter:BlockPredicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> filtered <filter> move\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> filtered <filter> normal")]
pub fn clone_begin_end_destination_filtered_filter_normal(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates,
    _filter:BlockPredicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> filtered <filter> normal\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> masked")]
pub fn clone_begin_end_destination_masked(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> masked\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> masked force")]
pub fn clone_begin_end_destination_masked_force(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> masked force\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> masked move")]
pub fn clone_begin_end_destination_masked_move(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> masked move\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> masked normal")]
pub fn clone_begin_end_destination_masked_normal(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> masked normal\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> replace")]
pub fn clone_begin_end_destination_replace(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> replace\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> replace force")]
pub fn clone_begin_end_destination_replace_force(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> replace force\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> replace move")]
pub fn clone_begin_end_destination_replace_move(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> replace move\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="clone <begin> <end> <destination> replace normal")]
pub fn clone_begin_end_destination_replace_normal(
    ctx: &mut CommandCtx,
    _begin:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"clone <begin> <end> <destination> replace normal\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get block <targetPos>")]
pub fn data_get_block_targetPos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get block <targetPos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get block <targetPos> <path>")]
pub fn data_get_block_targetPos_path(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get block <targetPos> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get block <targetPos> <path> <scale>")]
pub fn data_get_block_targetPos_path_scale(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _path:NbtPath,
    _scale:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get block <targetPos> <path> <scale>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get entity <target>")]
pub fn data_get_entity_target(
    ctx: &mut CommandCtx,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get entity <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get entity <target> <path>")]
pub fn data_get_entity_target_path(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get entity <target> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get entity <target> <path> <scale>")]
pub fn data_get_entity_target_path_scale(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _path:NbtPath,
    _scale:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get entity <target> <path> <scale>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get storage <target>")]
pub fn data_get_storage_target(
    ctx: &mut CommandCtx,
    _target:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get storage <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get storage <target> <path>")]
pub fn data_get_storage_target_path(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get storage <target> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data get storage <target> <path> <scale>")]
pub fn data_get_storage_target_path_scale(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _path:NbtPath,
    _scale:DoubleArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data get storage <target> <path> <scale>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data merge block <targetPos> <nbt>")]
pub fn data_merge_block_targetPos_nbt(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _nbt:NbtCommandTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data merge block <targetPos> <nbt>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data merge entity <target> <nbt>")]
pub fn data_merge_entity_target_nbt(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _nbt:NbtCommandTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data merge entity <target> <nbt>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data merge storage <target> <nbt>")]
pub fn data_merge_storage_target_nbt(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _nbt:NbtCommandTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data merge storage <target> <nbt>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> append from block <sourcePos>")]
pub fn data_modify_block_targetPos_targetPath_append_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> append from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> append from block <sourcePos> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_append_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> append from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> append from entity <source>")]
pub fn data_modify_block_targetPos_targetPath_append_from_entity_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> append from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> append from entity <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_append_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> append from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> append from storage <source>")]
pub fn data_modify_block_targetPos_targetPath_append_from_storage_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> append from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> append from storage <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_append_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> append from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> append value <value>")]
pub fn data_modify_block_targetPos_targetPath_append_value_value(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> append value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> insert <index> from block <sourcePos>")]
pub fn data_modify_block_targetPos_targetPath_insert_index_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> insert <index> from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> insert <index> from block <sourcePos> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_insert_index_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> insert <index> from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> insert <index> from entity <source>")]
pub fn data_modify_block_targetPos_targetPath_insert_index_from_entity_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> insert <index> from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> insert <index> from entity <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_insert_index_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> insert <index> from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> insert <index> from storage <source>")]
pub fn data_modify_block_targetPos_targetPath_insert_index_from_storage_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> insert <index> from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> insert <index> from storage <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_insert_index_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> insert <index> from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> insert <index> value <value>")]
pub fn data_modify_block_targetPos_targetPath_insert_index_value_value(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> insert <index> value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> merge from block <sourcePos>")]
pub fn data_modify_block_targetPos_targetPath_merge_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> merge from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> merge from block <sourcePos> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_merge_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> merge from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> merge from entity <source>")]
pub fn data_modify_block_targetPos_targetPath_merge_from_entity_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> merge from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> merge from entity <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_merge_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> merge from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> merge from storage <source>")]
pub fn data_modify_block_targetPos_targetPath_merge_from_storage_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> merge from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> merge from storage <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_merge_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> merge from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> merge value <value>")]
pub fn data_modify_block_targetPos_targetPath_merge_value_value(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> merge value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> prepend from block <sourcePos>")]
pub fn data_modify_block_targetPos_targetPath_prepend_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> prepend from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> prepend from block <sourcePos> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_prepend_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> prepend from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> prepend from entity <source>")]
pub fn data_modify_block_targetPos_targetPath_prepend_from_entity_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> prepend from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> prepend from entity <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_prepend_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> prepend from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> prepend from storage <source>")]
pub fn data_modify_block_targetPos_targetPath_prepend_from_storage_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> prepend from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> prepend from storage <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_prepend_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> prepend from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> prepend value <value>")]
pub fn data_modify_block_targetPos_targetPath_prepend_value_value(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> prepend value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> set from block <sourcePos>")]
pub fn data_modify_block_targetPos_targetPath_set_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> set from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> set from block <sourcePos> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_set_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> set from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> set from entity <source>")]
pub fn data_modify_block_targetPos_targetPath_set_from_entity_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> set from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> set from entity <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_set_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> set from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> set from storage <source>")]
pub fn data_modify_block_targetPos_targetPath_set_from_storage_source(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> set from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> set from storage <source> <sourcePath>")]
pub fn data_modify_block_targetPos_targetPath_set_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> set from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify block <targetPos> <targetPath> set value <value>")]
pub fn data_modify_block_targetPos_targetPath_set_value_value(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify block <targetPos> <targetPath> set value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> append from block <sourcePos>")]
pub fn data_modify_entity_target_targetPath_append_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> append from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> append from block <sourcePos> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_append_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> append from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> append from entity <source>")]
pub fn data_modify_entity_target_targetPath_append_from_entity_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> append from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> append from entity <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_append_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> append from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> append from storage <source>")]
pub fn data_modify_entity_target_targetPath_append_from_storage_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> append from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> append from storage <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_append_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> append from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> append value <value>")]
pub fn data_modify_entity_target_targetPath_append_value_value(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> append value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> insert <index> from block <sourcePos>")]
pub fn data_modify_entity_target_targetPath_insert_index_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> insert <index> from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> insert <index> from block <sourcePos> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_insert_index_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> insert <index> from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> insert <index> from entity <source>")]
pub fn data_modify_entity_target_targetPath_insert_index_from_entity_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> insert <index> from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> insert <index> from entity <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_insert_index_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> insert <index> from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> insert <index> from storage <source>")]
pub fn data_modify_entity_target_targetPath_insert_index_from_storage_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> insert <index> from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> insert <index> from storage <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_insert_index_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> insert <index> from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> insert <index> value <value>")]
pub fn data_modify_entity_target_targetPath_insert_index_value_value(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> insert <index> value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> merge from block <sourcePos>")]
pub fn data_modify_entity_target_targetPath_merge_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> merge from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> merge from block <sourcePos> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_merge_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> merge from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> merge from entity <source>")]
pub fn data_modify_entity_target_targetPath_merge_from_entity_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> merge from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> merge from entity <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_merge_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> merge from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> merge from storage <source>")]
pub fn data_modify_entity_target_targetPath_merge_from_storage_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> merge from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> merge from storage <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_merge_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> merge from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> merge value <value>")]
pub fn data_modify_entity_target_targetPath_merge_value_value(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> merge value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> prepend from block <sourcePos>")]
pub fn data_modify_entity_target_targetPath_prepend_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> prepend from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> prepend from block <sourcePos> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_prepend_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> prepend from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> prepend from entity <source>")]
pub fn data_modify_entity_target_targetPath_prepend_from_entity_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> prepend from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> prepend from entity <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_prepend_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> prepend from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> prepend from storage <source>")]
pub fn data_modify_entity_target_targetPath_prepend_from_storage_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> prepend from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> prepend from storage <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_prepend_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> prepend from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> prepend value <value>")]
pub fn data_modify_entity_target_targetPath_prepend_value_value(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> prepend value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> set from block <sourcePos>")]
pub fn data_modify_entity_target_targetPath_set_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> set from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> set from block <sourcePos> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_set_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> set from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> set from entity <source>")]
pub fn data_modify_entity_target_targetPath_set_from_entity_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> set from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> set from entity <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_set_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> set from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> set from storage <source>")]
pub fn data_modify_entity_target_targetPath_set_from_storage_source(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> set from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> set from storage <source> <sourcePath>")]
pub fn data_modify_entity_target_targetPath_set_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> set from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify entity <target> <targetPath> set value <value>")]
pub fn data_modify_entity_target_targetPath_set_value_value(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify entity <target> <targetPath> set value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> append from block <sourcePos>")]
pub fn data_modify_storage_target_targetPath_append_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> append from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> append from block <sourcePos> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_append_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> append from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> append from entity <source>")]
pub fn data_modify_storage_target_targetPath_append_from_entity_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> append from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> append from entity <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_append_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> append from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> append from storage <source>")]
pub fn data_modify_storage_target_targetPath_append_from_storage_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> append from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> append from storage <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_append_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> append from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> append value <value>")]
pub fn data_modify_storage_target_targetPath_append_value_value(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> append value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> insert <index> from block <sourcePos>")]
pub fn data_modify_storage_target_targetPath_insert_index_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> insert <index> from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> insert <index> from block <sourcePos> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_insert_index_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> insert <index> from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> insert <index> from entity <source>")]
pub fn data_modify_storage_target_targetPath_insert_index_from_entity_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> insert <index> from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> insert <index> from entity <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_insert_index_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> insert <index> from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> insert <index> from storage <source>")]
pub fn data_modify_storage_target_targetPath_insert_index_from_storage_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> insert <index> from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> insert <index> from storage <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_insert_index_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> insert <index> from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> insert <index> value <value>")]
pub fn data_modify_storage_target_targetPath_insert_index_value_value(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _index:IntegerArgument,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> insert <index> value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> merge from block <sourcePos>")]
pub fn data_modify_storage_target_targetPath_merge_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> merge from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> merge from block <sourcePos> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_merge_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> merge from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> merge from entity <source>")]
pub fn data_modify_storage_target_targetPath_merge_from_entity_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> merge from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> merge from entity <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_merge_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> merge from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> merge from storage <source>")]
pub fn data_modify_storage_target_targetPath_merge_from_storage_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> merge from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> merge from storage <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_merge_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> merge from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> merge value <value>")]
pub fn data_modify_storage_target_targetPath_merge_value_value(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> merge value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> prepend from block <sourcePos>")]
pub fn data_modify_storage_target_targetPath_prepend_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> prepend from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> prepend from block <sourcePos> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_prepend_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> prepend from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> prepend from entity <source>")]
pub fn data_modify_storage_target_targetPath_prepend_from_entity_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> prepend from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> prepend from entity <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_prepend_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> prepend from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> prepend from storage <source>")]
pub fn data_modify_storage_target_targetPath_prepend_from_storage_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> prepend from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> prepend from storage <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_prepend_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> prepend from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> prepend value <value>")]
pub fn data_modify_storage_target_targetPath_prepend_value_value(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> prepend value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> set from block <sourcePos>")]
pub fn data_modify_storage_target_targetPath_set_from_block_sourcePos(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> set from block <sourcePos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> set from block <sourcePos> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_set_from_block_sourcePos_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _sourcePos:Coordinates,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> set from block <sourcePos> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> set from entity <source>")]
pub fn data_modify_storage_target_targetPath_set_from_entity_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> set from entity <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> set from entity <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_set_from_entity_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:SingleEntities,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> set from entity <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> set from storage <source>")]
pub fn data_modify_storage_target_targetPath_set_from_storage_source(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> set from storage <source>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> set from storage <source> <sourcePath>")]
pub fn data_modify_storage_target_targetPath_set_from_storage_source_sourcePath(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _source:ResourceLocation,
    _sourcePath:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> set from storage <source> <sourcePath>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data modify storage <target> <targetPath> set value <value>")]
pub fn data_modify_storage_target_targetPath_set_value_value(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _targetPath:NbtPath,
    _value:NbtTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data modify storage <target> <targetPath> set value <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data remove block <targetPos> <path>")]
pub fn data_remove_block_targetPos_path(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data remove block <targetPos> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data remove entity <target> <path>")]
pub fn data_remove_entity_target_path(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data remove entity <target> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="data remove storage <target> <path>")]
pub fn data_remove_storage_target_path(
    ctx: &mut CommandCtx,
    _target:ResourceLocation,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"data remove storage <target> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack disable <name>")]
pub fn datapack_disable_name(
    ctx: &mut CommandCtx,
    _name:StringArgumentPhrase
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack disable <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack enable <name>")]
pub fn datapack_enable_name(
    ctx: &mut CommandCtx,
    _name:StringArgumentPhrase
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack enable <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack enable <name> after <existing>")]
pub fn datapack_enable_name_after_existing(
    ctx: &mut CommandCtx,
    _name:StringArgumentPhrase,
    _existing:StringArgumentPhrase
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack enable <name> after <existing>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack enable <name> before <existing>")]
pub fn datapack_enable_name_before_existing(
    ctx: &mut CommandCtx,
    _name:StringArgumentPhrase,
    _existing:StringArgumentPhrase
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack enable <name> before <existing>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack enable <name> first")]
pub fn datapack_enable_name_first(
    ctx: &mut CommandCtx,
    _name:StringArgumentPhrase
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack enable <name> first\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack enable <name> last")]
pub fn datapack_enable_name_last(
    ctx: &mut CommandCtx,
    _name:StringArgumentPhrase
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack enable <name> last\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack list")]
pub fn datapack_list(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack list available")]
pub fn datapack_list_available(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack list available\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="datapack list enabled")]
pub fn datapack_list_enabled(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"datapack list enabled\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="debug report")]
pub fn debug_report(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"debug report\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="debug start")]
pub fn debug_start(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"debug start\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="debug stop")]
pub fn debug_stop(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"debug stop\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="defaultgamemode adventure")]
pub fn defaultgamemode_adventure(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"defaultgamemode adventure\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="defaultgamemode creative")]
pub fn defaultgamemode_creative(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"defaultgamemode creative\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="defaultgamemode spectator")]
pub fn defaultgamemode_spectator(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"defaultgamemode spectator\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="defaultgamemode survival")]
pub fn defaultgamemode_survival(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"defaultgamemode survival\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="deop <targets>")]
pub fn deop_targets(
    ctx: &mut CommandCtx,
    _targets:GameProfile
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"deop <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="difficulty")]
pub fn difficulty(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"difficulty\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="difficulty easy")]
pub fn difficulty_easy(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"difficulty easy\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="difficulty hard")]
pub fn difficulty_hard(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"difficulty hard\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="difficulty normal")]
pub fn difficulty_normal(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"difficulty normal\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="difficulty peaceful")]
pub fn difficulty_peaceful(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"difficulty peaceful\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="effect clear")]
pub fn effect_clear(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"effect clear\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="effect clear <targets>")]
pub fn effect_clear_targets(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"effect clear <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="effect clear <targets> <effect>")]
pub fn effect_clear_targets_effect(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _effect:MobEffect
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"effect clear <targets> <effect>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="effect give <targets> <effect>")]
pub fn effect_give_targets_effect(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _effect:MobEffect
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"effect give <targets> <effect>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="effect give <targets> <effect> <seconds>")]
pub fn effect_give_targets_effect_seconds(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _effect:MobEffect,
    _seconds:IntegerArgumentBetween1And1000000
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"effect give <targets> <effect> <seconds>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="effect give <targets> <effect> <seconds> <amplifier>")]
pub fn effect_give_targets_effect_seconds_amplifier(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _effect:MobEffect,
    _seconds:IntegerArgumentBetween1And1000000,
    _amplifier:IntegerArgumentBetween0And255
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"effect give <targets> <effect> <seconds> <amplifier>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="effect give <targets> <effect> <seconds> <amplifier> <hideParticles>")]
pub fn effect_give_targets_effect_seconds_amplifier_hideParticles(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _effect:MobEffect,
    _seconds:IntegerArgumentBetween1And1000000,
    _amplifier:IntegerArgumentBetween0And255,
    _hideParticles:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"effect give <targets> <effect> <seconds> <amplifier> <hideParticles>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="enchant <targets> <enchantment>")]
pub fn enchant_targets_enchantment(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _enchantment:Enchantment
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"enchant <targets> <enchantment>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="enchant <targets> <enchantment> <level>")]
pub fn enchant_targets_enchantment_level(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _enchantment:Enchantment,
    _level:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"enchant <targets> <enchantment> <level>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if block <pos> <block>")]
pub fn execute_if_block_pos_block(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _block:BlockPredicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if block <pos> <block>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if blocks <start> <end> <destination> all")]
pub fn execute_if_blocks_start_end_destination_all(
    ctx: &mut CommandCtx,
    _start:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if blocks <start> <end> <destination> all\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if blocks <start> <end> <destination> masked")]
pub fn execute_if_blocks_start_end_destination_masked(
    ctx: &mut CommandCtx,
    _start:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if blocks <start> <end> <destination> masked\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if data block <sourcePos> <path>")]
pub fn execute_if_data_block_sourcePos_path(
    ctx: &mut CommandCtx,
    _sourcePos:Coordinates,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if data block <sourcePos> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if data entity <source> <path>")]
pub fn execute_if_data_entity_source_path(
    ctx: &mut CommandCtx,
    _source:SingleEntities,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if data entity <source> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if data storage <source> <path>")]
pub fn execute_if_data_storage_source_path(
    ctx: &mut CommandCtx,
    _source:ResourceLocation,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if data storage <source> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if entity <entities>")]
pub fn execute_if_entity_entities(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if entity <entities>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if predicate <predicate>")]
pub fn execute_if_predicate_predicate(
    ctx: &mut CommandCtx,
    _predicate:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if predicate <predicate>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="execute if score <target> <targetObjective> < <source> <sourceObjective>")]
pub fn execute_if_score_target_targetObjective_lt_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if score <target> <targetObjective> < <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/
#[command(usage="execute if score <target> <targetObjective> <= <source> <sourceObjective>")]
pub fn execute_if_score_target_targetObjective_lteq_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if score <target> <targetObjective> <= <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="execute if score <target> <targetObjective> = <source> <sourceObjective>")]
pub fn execute_if_score_target_targetObjective_eq_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if score <target> <targetObjective> = <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/
/*
#[command(usage="execute if score <target> <targetObjective> > <source> <sourceObjective>")]
pub fn execute_if_score_target_targetObjective_gt_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if score <target> <targetObjective> > <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/

#[command(usage="execute if score <target> <targetObjective> >= <source> <sourceObjective>")]
pub fn execute_if_score_target_targetObjective_gteq_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if score <target> <targetObjective> >= <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute if score <target> <targetObjective> matches <range>")]
pub fn execute_if_score_target_targetObjective_matches_range(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _range:IntRange
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute if score <target> <targetObjective> matches <range>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless block <pos> <block>")]
pub fn execute_unless_block_pos_block(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _block:BlockPredicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless block <pos> <block>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless blocks <start> <end> <destination> all")]
pub fn execute_unless_blocks_start_end_destination_all(
    ctx: &mut CommandCtx,
    _start:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless blocks <start> <end> <destination> all\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless blocks <start> <end> <destination> masked")]
pub fn execute_unless_blocks_start_end_destination_masked(
    ctx: &mut CommandCtx,
    _start:Coordinates,
    _end:Coordinates,
    _destination:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless blocks <start> <end> <destination> masked\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless data block <sourcePos> <path>")]
pub fn execute_unless_data_block_sourcePos_path(
    ctx: &mut CommandCtx,
    _sourcePos:Coordinates,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless data block <sourcePos> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless data entity <source> <path>")]
pub fn execute_unless_data_entity_source_path(
    ctx: &mut CommandCtx,
    _source:SingleEntities,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless data entity <source> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless data storage <source> <path>")]
pub fn execute_unless_data_storage_source_path(
    ctx: &mut CommandCtx,
    _source:ResourceLocation,
    _path:NbtPath
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless data storage <source> <path>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless entity <entities>")]
pub fn execute_unless_entity_entities(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless entity <entities>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless predicate <predicate>")]
pub fn execute_unless_predicate_predicate(
    ctx: &mut CommandCtx,
    _predicate:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless predicate <predicate>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="execute unless score <target> <targetObjective> < <source> <sourceObjective>")]
pub fn execute_unless_score_target_targetObjective_lt_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless score <target> <targetObjective> < <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/

#[command(usage="execute unless score <target> <targetObjective> <= <source> <sourceObjective>")]
pub fn execute_unless_score_target_targetObjective_lteq_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless score <target> <targetObjective> <= <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="execute unless score <target> <targetObjective> = <source> <sourceObjective>")]
pub fn execute_unless_score_target_targetObjective_eq_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless score <target> <targetObjective> = <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/
/*
#[command(usage="execute unless score <target> <targetObjective> > <source> <sourceObjective>")]
pub fn execute_unless_score_target_targetObjective_gt_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless score <target> <targetObjective> > <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/
#[command(usage="execute unless score <target> <targetObjective> >= <source> <sourceObjective>")]
pub fn execute_unless_score_target_targetObjective_gteq_source_sourceObjective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _source:SingleScoreHolder,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless score <target> <targetObjective> >= <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="execute unless score <target> <targetObjective> matches <range>")]
pub fn execute_unless_score_target_targetObjective_matches_range(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _targetObjective:Objective,
    _range:IntRange
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"execute unless score <target> <targetObjective> matches <range>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience add <targets> <amount>")]
pub fn experience_add_targets_amount(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience add <targets> <amount>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience add <targets> <amount> levels")]
pub fn experience_add_targets_amount_levels(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience add <targets> <amount> levels\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience add <targets> <amount> points")]
pub fn experience_add_targets_amount_points(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience add <targets> <amount> points\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience query <targets> levels")]
pub fn experience_query_targets_levels(
    ctx: &mut CommandCtx,
    _targets:SinglePlayer
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience query <targets> levels\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience query <targets> points")]
pub fn experience_query_targets_points(
    ctx: &mut CommandCtx,
    _targets:SinglePlayer
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience query <targets> points\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience set <targets> <amount>")]
pub fn experience_set_targets_amount(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience set <targets> <amount>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience set <targets> <amount> levels")]
pub fn experience_set_targets_amount_levels(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience set <targets> <amount> levels\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="experience set <targets> <amount> points")]
pub fn experience_set_targets_amount_points(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"experience set <targets> <amount> points\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="fill <from> <to> <block>")]
pub fn fill_from_to_block(
    ctx: &mut CommandCtx,
    _from:Coordinates,
    _to:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"fill <from> <to> <block>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="fill <from> <to> <block> destroy")]
pub fn fill_from_to_block_destroy(
    ctx: &mut CommandCtx,
    _from:Coordinates,
    _to:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"fill <from> <to> <block> destroy\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="fill <from> <to> <block> hollow")]
pub fn fill_from_to_block_hollow(
    ctx: &mut CommandCtx,
    _from:Coordinates,
    _to:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"fill <from> <to> <block> hollow\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="fill <from> <to> <block> keep")]
pub fn fill_from_to_block_keep(
    ctx: &mut CommandCtx,
    _from:Coordinates,
    _to:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"fill <from> <to> <block> keep\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="fill <from> <to> <block> outline")]
pub fn fill_from_to_block_outline(
    ctx: &mut CommandCtx,
    _from:Coordinates,
    _to:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"fill <from> <to> <block> outline\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="fill <from> <to> <block> replace")]
pub fn fill_from_to_block_replace(
    ctx: &mut CommandCtx,
    _from:Coordinates,
    _to:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"fill <from> <to> <block> replace\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="fill <from> <to> <block> replace <filter>")]
pub fn fill_from_to_block_replace_filter(
    ctx: &mut CommandCtx,
    _from:Coordinates,
    _to:Coordinates,
    _block:BlockState,
    _filter:BlockPredicate
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"fill <from> <to> <block> replace <filter>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="forceload add <from>")]
pub fn forceload_add_from(
    ctx: &mut CommandCtx,
    _from:ColumnPos
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"forceload add <from>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="forceload add <from> <to>")]
pub fn forceload_add_from_to(
    ctx: &mut CommandCtx,
    _from:ColumnPos,
    _to:ColumnPos
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"forceload add <from> <to>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="forceload query")]
pub fn forceload_query(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"forceload query\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="forceload query <pos>")]
pub fn forceload_query_pos(
    ctx: &mut CommandCtx,
    _pos:ColumnPos
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"forceload query <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="forceload remove all")]
pub fn forceload_remove_all(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"forceload remove all\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="forceload remove <from>")]
pub fn forceload_remove_from(
    ctx: &mut CommandCtx,
    _from:ColumnPos
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"forceload remove <from>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="forceload remove <from> <to>")]
pub fn forceload_remove_from_to(
    ctx: &mut CommandCtx,
    _from:ColumnPos,
    _to:ColumnPos
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"forceload remove <from> <to>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="function <name>")]
pub fn function_name(
    ctx: &mut CommandCtx,
    _name:MinecraftFunction
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"function <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode adventure")]
pub fn gamemode_adventure(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode adventure\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode adventure <target>")]
pub fn gamemode_adventure_target(
    ctx: &mut CommandCtx,
    _target:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode adventure <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode creative")]
pub fn gamemode_creative(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode creative\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode creative <target>")]
pub fn gamemode_creative_target(
    ctx: &mut CommandCtx,
    _target:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode creative <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode spectator")]
pub fn gamemode_spectator(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode spectator\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode spectator <target>")]
pub fn gamemode_spectator_target(
    ctx: &mut CommandCtx,
    _target:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode spectator <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode survival")]
pub fn gamemode_survival(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode survival\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamemode survival <target>")]
pub fn gamemode_survival_target(
    ctx: &mut CommandCtx,
    _target:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamemode survival <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule announceAdvancements")]
pub fn gamerule_announceAdvancements(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule announceAdvancements\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule announceAdvancements <value>")]
pub fn gamerule_announceAdvancements_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule announceAdvancements <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule commandBlockOutput")]
pub fn gamerule_commandBlockOutput(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule commandBlockOutput\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule commandBlockOutput <value>")]
pub fn gamerule_commandBlockOutput_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule commandBlockOutput <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule disableElytraMovementCheck")]
pub fn gamerule_disableElytraMovementCheck(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule disableElytraMovementCheck\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule disableElytraMovementCheck <value>")]
pub fn gamerule_disableElytraMovementCheck_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule disableElytraMovementCheck <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule disableRaids")]
pub fn gamerule_disableRaids(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule disableRaids\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule disableRaids <value>")]
pub fn gamerule_disableRaids_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule disableRaids <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doDaylightCycle")]
pub fn gamerule_doDaylightCycle(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doDaylightCycle\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doDaylightCycle <value>")]
pub fn gamerule_doDaylightCycle_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doDaylightCycle <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doEntityDrops")]
pub fn gamerule_doEntityDrops(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doEntityDrops\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doEntityDrops <value>")]
pub fn gamerule_doEntityDrops_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doEntityDrops <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doFireTick")]
pub fn gamerule_doFireTick(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doFireTick\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doFireTick <value>")]
pub fn gamerule_doFireTick_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doFireTick <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doImmediateRespawn")]
pub fn gamerule_doImmediateRespawn(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doImmediateRespawn\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doImmediateRespawn <value>")]
pub fn gamerule_doImmediateRespawn_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doImmediateRespawn <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doInsomnia")]
pub fn gamerule_doInsomnia(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doInsomnia\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doInsomnia <value>")]
pub fn gamerule_doInsomnia_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doInsomnia <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doLimitedCrafting")]
pub fn gamerule_doLimitedCrafting(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doLimitedCrafting\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doLimitedCrafting <value>")]
pub fn gamerule_doLimitedCrafting_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doLimitedCrafting <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doMobLoot")]
pub fn gamerule_doMobLoot(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doMobLoot\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doMobLoot <value>")]
pub fn gamerule_doMobLoot_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doMobLoot <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doMobSpawning")]
pub fn gamerule_doMobSpawning(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doMobSpawning\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doMobSpawning <value>")]
pub fn gamerule_doMobSpawning_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doMobSpawning <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doPatrolSpawning")]
pub fn gamerule_doPatrolSpawning(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doPatrolSpawning\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doPatrolSpawning <value>")]
pub fn gamerule_doPatrolSpawning_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doPatrolSpawning <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doTileDrops")]
pub fn gamerule_doTileDrops(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doTileDrops\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doTileDrops <value>")]
pub fn gamerule_doTileDrops_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doTileDrops <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doTraderSpawning")]
pub fn gamerule_doTraderSpawning(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doTraderSpawning\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doTraderSpawning <value>")]
pub fn gamerule_doTraderSpawning_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doTraderSpawning <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doWeatherCycle")]
pub fn gamerule_doWeatherCycle(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doWeatherCycle\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule doWeatherCycle <value>")]
pub fn gamerule_doWeatherCycle_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule doWeatherCycle <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule drowningDamage")]
pub fn gamerule_drowningDamage(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule drowningDamage\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule drowningDamage <value>")]
pub fn gamerule_drowningDamage_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule drowningDamage <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule fallDamage")]
pub fn gamerule_fallDamage(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule fallDamage\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule fallDamage <value>")]
pub fn gamerule_fallDamage_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule fallDamage <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule fireDamage")]
pub fn gamerule_fireDamage(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule fireDamage\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule fireDamage <value>")]
pub fn gamerule_fireDamage_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule fireDamage <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule forgiveDeadPlayers")]
pub fn gamerule_forgiveDeadPlayers(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule forgiveDeadPlayers\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule forgiveDeadPlayers <value>")]
pub fn gamerule_forgiveDeadPlayers_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule forgiveDeadPlayers <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule keepInventory")]
pub fn gamerule_keepInventory(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule keepInventory\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule keepInventory <value>")]
pub fn gamerule_keepInventory_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule keepInventory <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule logAdminCommands")]
pub fn gamerule_logAdminCommands(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule logAdminCommands\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule logAdminCommands <value>")]
pub fn gamerule_logAdminCommands_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule logAdminCommands <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule maxCommandChainLength")]
pub fn gamerule_maxCommandChainLength(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule maxCommandChainLength\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule maxCommandChainLength <value>")]
pub fn gamerule_maxCommandChainLength_value(
    ctx: &mut CommandCtx,
    _value:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule maxCommandChainLength <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule maxEntityCramming")]
pub fn gamerule_maxEntityCramming(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule maxEntityCramming\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule maxEntityCramming <value>")]
pub fn gamerule_maxEntityCramming_value(
    ctx: &mut CommandCtx,
    _value:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule maxEntityCramming <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule mobGriefing")]
pub fn gamerule_mobGriefing(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule mobGriefing\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule mobGriefing <value>")]
pub fn gamerule_mobGriefing_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule mobGriefing <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule naturalRegeneration")]
pub fn gamerule_naturalRegeneration(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule naturalRegeneration\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule naturalRegeneration <value>")]
pub fn gamerule_naturalRegeneration_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule naturalRegeneration <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule randomTickSpeed")]
pub fn gamerule_randomTickSpeed(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule randomTickSpeed\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule randomTickSpeed <value>")]
pub fn gamerule_randomTickSpeed_value(
    ctx: &mut CommandCtx,
    _value:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule randomTickSpeed <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule reducedDebugInfo")]
pub fn gamerule_reducedDebugInfo(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule reducedDebugInfo\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule reducedDebugInfo <value>")]
pub fn gamerule_reducedDebugInfo_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule reducedDebugInfo <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule sendCommandFeedback")]
pub fn gamerule_sendCommandFeedback(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule sendCommandFeedback\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule sendCommandFeedback <value>")]
pub fn gamerule_sendCommandFeedback_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule sendCommandFeedback <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule showDeathMessages")]
pub fn gamerule_showDeathMessages(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule showDeathMessages\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule showDeathMessages <value>")]
pub fn gamerule_showDeathMessages_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule showDeathMessages <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule spawnRadius")]
pub fn gamerule_spawnRadius(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule spawnRadius\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule spawnRadius <value>")]
pub fn gamerule_spawnRadius_value(
    ctx: &mut CommandCtx,
    _value:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule spawnRadius <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule spectatorsGenerateChunks")]
pub fn gamerule_spectatorsGenerateChunks(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule spectatorsGenerateChunks\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule spectatorsGenerateChunks <value>")]
pub fn gamerule_spectatorsGenerateChunks_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule spectatorsGenerateChunks <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule universalAnger")]
pub fn gamerule_universalAnger(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule universalAnger\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="gamerule universalAnger <value>")]
pub fn gamerule_universalAnger_value(
    ctx: &mut CommandCtx,
    _value:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"gamerule universalAnger <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="give <targets> <item>")]
pub fn give_targets_item(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _item:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"give <targets> <item>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="give <targets> <item> <count>")]
pub fn give_targets_item_count(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _item:ItemStack,
    _count:IntegerArgumentGreaterThen1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"give <targets> <item> <count>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="help")]
pub fn help(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"help\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="help <command>")]
pub fn help_command(
    ctx: &mut CommandCtx,
    _command:StringArgumentGreedy
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"help <command>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="kick <targets>")]
pub fn kick_targets(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"kick <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="kick <targets> <reason>")]
pub fn kick_targets_reason(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _reason:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"kick <targets> <reason>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="kill")]
pub fn kill(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"kill\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="kill <targets>")]
pub fn kill_targets(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"kill <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}


#[command(usage="list")]
pub fn list(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="list uuids")]
pub fn list_uuids(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"list uuids\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate bastion_remnant")]
pub fn locate_bastion_remnant(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate bastion_remnant\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate buried_treasure")]
pub fn locate_buried_treasure(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate buried_treasure\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate desert_pyramid")]
pub fn locate_desert_pyramid(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate desert_pyramid\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate endcity")]
pub fn locate_endcity(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate endcity\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate fortress")]
pub fn locate_fortress(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate fortress\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate igloo")]
pub fn locate_igloo(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate igloo\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate jungle_pyramid")]
pub fn locate_jungle_pyramid(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate jungle_pyramid\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate mansion")]
pub fn locate_mansion(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate mansion\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate mineshaft")]
pub fn locate_mineshaft(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate mineshaft\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate monument")]
pub fn locate_monument(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate monument\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate nether_fossil")]
pub fn locate_nether_fossil(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate nether_fossil\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate ocean_ruin")]
pub fn locate_ocean_ruin(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate ocean_ruin\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate pillager_outpost")]
pub fn locate_pillager_outpost(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate pillager_outpost\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate ruined_portal")]
pub fn locate_ruined_portal(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate ruined_portal\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate shipwreck")]
pub fn locate_shipwreck(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate shipwreck\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate stronghold")]
pub fn locate_stronghold(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate stronghold\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate swamp_hut")]
pub fn locate_swamp_hut(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate swamp_hut\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locate village")]
pub fn locate_village(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locate village\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="locatebiome <biome>")]
pub fn locatebiome_biome(
    ctx: &mut CommandCtx,
    _biome:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"locatebiome <biome>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> fish <loot_table> <pos>")]
pub fn loot_give_players_fish_loot_table_pos(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> fish <loot_table> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> fish <loot_table> <pos> mainhand")]
pub fn loot_give_players_fish_loot_table_pos_mainhand(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> fish <loot_table> <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> fish <loot_table> <pos> offhand")]
pub fn loot_give_players_fish_loot_table_pos_offhand(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> fish <loot_table> <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> fish <loot_table> <pos> <tool>")]
pub fn loot_give_players_fish_loot_table_pos_tool(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _loot_table:ResourceLocation,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> fish <loot_table> <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> kill <target>")]
pub fn loot_give_players_kill_target(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> kill <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> loot <loot_table>")]
pub fn loot_give_players_loot_loot_table(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _loot_table:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> loot <loot_table>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> mine <pos>")]
pub fn loot_give_players_mine_pos(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> mine <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> mine <pos> mainhand")]
pub fn loot_give_players_mine_pos_mainhand(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> mine <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> mine <pos> offhand")]
pub fn loot_give_players_mine_pos_offhand(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> mine <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot give <players> mine <pos> <tool>")]
pub fn loot_give_players_mine_pos_tool(
    ctx: &mut CommandCtx,
    _players:MultiplePlayers,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot give <players> mine <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> fish <loot_table> <pos>")]
pub fn loot_insert_targetPos_fish_loot_table_pos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> fish <loot_table> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> fish <loot_table> <pos> mainhand")]
pub fn loot_insert_targetPos_fish_loot_table_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> fish <loot_table> <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> fish <loot_table> <pos> offhand")]
pub fn loot_insert_targetPos_fish_loot_table_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> fish <loot_table> <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> fish <loot_table> <pos> <tool>")]
pub fn loot_insert_targetPos_fish_loot_table_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _loot_table:ResourceLocation,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> fish <loot_table> <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> kill <target>")]
pub fn loot_insert_targetPos_kill_target(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> kill <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> loot <loot_table>")]
pub fn loot_insert_targetPos_loot_loot_table(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _loot_table:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> loot <loot_table>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> mine <pos>")]
pub fn loot_insert_targetPos_mine_pos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> mine <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> mine <pos> mainhand")]
pub fn loot_insert_targetPos_mine_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> mine <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> mine <pos> offhand")]
pub fn loot_insert_targetPos_mine_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> mine <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot insert <targetPos> mine <pos> <tool>")]
pub fn loot_insert_targetPos_mine_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot insert <targetPos> mine <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> fish <loot_table> <pos>")]
pub fn loot_replace_block_targetPos_slot_fish_loot_table_pos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> fish <loot_table> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> fish <loot_table> <pos> mainhand")]
pub fn loot_replace_block_targetPos_slot_fish_loot_table_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> fish <loot_table> <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> fish <loot_table> <pos> offhand")]
pub fn loot_replace_block_targetPos_slot_fish_loot_table_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> fish <loot_table> <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> fish <loot_table> <pos> <tool>")]
pub fn loot_replace_block_targetPos_slot_fish_loot_table_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> fish <loot_table> <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> kill <target>")]
pub fn loot_replace_block_targetPos_slot_kill_target(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> kill <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> loot <loot_table>")]
pub fn loot_replace_block_targetPos_slot_loot_loot_table(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _loot_table:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> loot <loot_table>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> mine <pos>")]
pub fn loot_replace_block_targetPos_slot_mine_pos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> mine <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> mine <pos> mainhand")]
pub fn loot_replace_block_targetPos_slot_mine_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> mine <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> mine <pos> offhand")]
pub fn loot_replace_block_targetPos_slot_mine_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> mine <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> mine <pos> <tool>")]
pub fn loot_replace_block_targetPos_slot_mine_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> mine <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> fish <loot_table> <pos>")]
pub fn loot_replace_block_targetPos_slot_count_fish_loot_table_pos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> fish <loot_table> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> fish <loot_table> <pos> mainhand")]
pub fn loot_replace_block_targetPos_slot_count_fish_loot_table_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> fish <loot_table> <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> fish <loot_table> <pos> offhand")]
pub fn loot_replace_block_targetPos_slot_count_fish_loot_table_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> fish <loot_table> <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> fish <loot_table> <pos> <tool>")]
pub fn loot_replace_block_targetPos_slot_count_fish_loot_table_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> fish <loot_table> <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> kill <target>")]
pub fn loot_replace_block_targetPos_slot_count_kill_target(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> kill <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> loot <loot_table>")]
pub fn loot_replace_block_targetPos_slot_count_loot_loot_table(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> loot <loot_table>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> mine <pos>")]
pub fn loot_replace_block_targetPos_slot_count_mine_pos(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> mine <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> mine <pos> mainhand")]
pub fn loot_replace_block_targetPos_slot_count_mine_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> mine <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> mine <pos> offhand")]
pub fn loot_replace_block_targetPos_slot_count_mine_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> mine <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace block <targetPos> <slot> <count> mine <pos> <tool>")]
pub fn loot_replace_block_targetPos_slot_count_mine_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Coordinates,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace block <targetPos> <slot> <count> mine <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> fish <loot_table> <pos>")]
pub fn loot_replace_entity_entities_slot_fish_loot_table_pos(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> fish <loot_table> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> fish <loot_table> <pos> mainhand")]
pub fn loot_replace_entity_entities_slot_fish_loot_table_pos_mainhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> fish <loot_table> <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> fish <loot_table> <pos> offhand")]
pub fn loot_replace_entity_entities_slot_fish_loot_table_pos_offhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> fish <loot_table> <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> fish <loot_table> <pos> <tool>")]
pub fn loot_replace_entity_entities_slot_fish_loot_table_pos_tool(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _loot_table:ResourceLocation,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> fish <loot_table> <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> kill <target>")]
pub fn loot_replace_entity_entities_slot_kill_target(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> kill <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> loot <loot_table>")]
pub fn loot_replace_entity_entities_slot_loot_loot_table(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _loot_table:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> loot <loot_table>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> mine <pos>")]
pub fn loot_replace_entity_entities_slot_mine_pos(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> mine <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> mine <pos> mainhand")]
pub fn loot_replace_entity_entities_slot_mine_pos_mainhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> mine <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> mine <pos> offhand")]
pub fn loot_replace_entity_entities_slot_mine_pos_offhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> mine <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> mine <pos> <tool>")]
pub fn loot_replace_entity_entities_slot_mine_pos_tool(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> mine <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> fish <loot_table> <pos>")]
pub fn loot_replace_entity_entities_slot_count_fish_loot_table_pos(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> fish <loot_table> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> fish <loot_table> <pos> mainhand")]
pub fn loot_replace_entity_entities_slot_count_fish_loot_table_pos_mainhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> fish <loot_table> <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> fish <loot_table> <pos> offhand")]
pub fn loot_replace_entity_entities_slot_count_fish_loot_table_pos_offhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> fish <loot_table> <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> fish <loot_table> <pos> <tool>")]
pub fn loot_replace_entity_entities_slot_count_fish_loot_table_pos_tool(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> fish <loot_table> <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> kill <target>")]
pub fn loot_replace_entity_entities_slot_count_kill_target(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> kill <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> loot <loot_table>")]
pub fn loot_replace_entity_entities_slot_count_loot_loot_table(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _loot_table:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> loot <loot_table>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> mine <pos>")]
pub fn loot_replace_entity_entities_slot_count_mine_pos(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> mine <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> mine <pos> mainhand")]
pub fn loot_replace_entity_entities_slot_count_mine_pos_mainhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> mine <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> mine <pos> offhand")]
pub fn loot_replace_entity_entities_slot_count_mine_pos_offhand(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> mine <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot replace entity <entities> <slot> <count> mine <pos> <tool>")]
pub fn loot_replace_entity_entities_slot_count_mine_pos_tool(
    ctx: &mut CommandCtx,
    _entities:MultipleEntities,
    _slot:ItemSlot,
    _count:IntegerArgumentPositive,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot replace entity <entities> <slot> <count> mine <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> fish <loot_table> <pos>")]
pub fn loot_spawn_targetPos_fish_loot_table_pos(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> fish <loot_table> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> fish <loot_table> <pos> mainhand")]
pub fn loot_spawn_targetPos_fish_loot_table_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> fish <loot_table> <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> fish <loot_table> <pos> offhand")]
pub fn loot_spawn_targetPos_fish_loot_table_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _loot_table:ResourceLocation,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> fish <loot_table> <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> fish <loot_table> <pos> <tool>")]
pub fn loot_spawn_targetPos_fish_loot_table_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _loot_table:ResourceLocation,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> fish <loot_table> <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> kill <target>")]
pub fn loot_spawn_targetPos_kill_target(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> kill <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> loot <loot_table>")]
pub fn loot_spawn_targetPos_loot_loot_table(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _loot_table:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> loot <loot_table>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> mine <pos>")]
pub fn loot_spawn_targetPos_mine_pos(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> mine <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> mine <pos> mainhand")]
pub fn loot_spawn_targetPos_mine_pos_mainhand(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> mine <pos> mainhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> mine <pos> offhand")]
pub fn loot_spawn_targetPos_mine_pos_offhand(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> mine <pos> offhand\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="loot spawn <targetPos> mine <pos> <tool>")]
pub fn loot_spawn_targetPos_mine_pos_tool(
    ctx: &mut CommandCtx,
    _targetPos:Vec3,
    _pos:Coordinates,
    _tool:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"loot spawn <targetPos> mine <pos> <tool>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="me <action>")]
pub fn me_action(
    ctx: &mut CommandCtx,
    _action:StringArgumentGreedy
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"me <action>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="msg <targets> <message>")]
pub fn msg_targets_message(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _message:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"msg <targets> <message>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="op <targets>")]
pub fn op_targets(
    ctx: &mut CommandCtx,
    _targets:GameProfile
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"op <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="pardon <targets>")]
pub fn pardon_targets(
    ctx: &mut CommandCtx,
    _targets:GameProfile
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"pardon <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="pardon-ip <target>")]
pub fn pardon_ip_target(
    ctx: &mut CommandCtx,
    _target:StringArgumentWord
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"pardon-ip <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="particle <name>")]
pub fn particle_name(
    ctx: &mut CommandCtx,
    _name:Particle
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"particle <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="particle <name> <pos>")]
pub fn particle_name_pos(
    ctx: &mut CommandCtx,
    _name:Particle,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"particle <name> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="particle <name> <pos> <delta> <speed> <count>")]
pub fn particle_name_pos_delta_speed_count(
    ctx: &mut CommandCtx,
    _name:Particle,
    _pos:Vec3,
    _delta:Vec3,
    _speed:FloatArgumentPositive,
    _count:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"particle <name> <pos> <delta> <speed> <count>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="particle <name> <pos> <delta> <speed> <count> force")]
pub fn particle_name_pos_delta_speed_count_force(
    ctx: &mut CommandCtx,
    _name:Particle,
    _pos:Vec3,
    _delta:Vec3,
    _speed:FloatArgumentPositive,
    _count:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"particle <name> <pos> <delta> <speed> <count> force\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="particle <name> <pos> <delta> <speed> <count> force <viewers>")]
pub fn particle_name_pos_delta_speed_count_force_viewers(
    ctx: &mut CommandCtx,
    _name:Particle,
    _pos:Vec3,
    _delta:Vec3,
    _speed:FloatArgumentPositive,
    _count:IntegerArgumentPositive,
    _viewers:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"particle <name> <pos> <delta> <speed> <count> force <viewers>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="particle <name> <pos> <delta> <speed> <count> normal")]
pub fn particle_name_pos_delta_speed_count_normal(
    ctx: &mut CommandCtx,
    _name:Particle,
    _pos:Vec3,
    _delta:Vec3,
    _speed:FloatArgumentPositive,
    _count:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"particle <name> <pos> <delta> <speed> <count> normal\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="particle <name> <pos> <delta> <speed> <count> normal <viewers>")]
pub fn particle_name_pos_delta_speed_count_normal_viewers(
    ctx: &mut CommandCtx,
    _name:Particle,
    _pos:Vec3,
    _delta:Vec3,
    _speed:FloatArgumentPositive,
    _count:IntegerArgumentPositive,
    _viewers:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"particle <name> <pos> <delta> <speed> <count> normal <viewers>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> ambient <targets>")]
pub fn playsound_sound_ambient_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> ambient <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> ambient <targets> <pos>")]
pub fn playsound_sound_ambient_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> ambient <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> ambient <targets> <pos> <volume>")]
pub fn playsound_sound_ambient_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> ambient <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> ambient <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_ambient_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> ambient <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> ambient <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_ambient_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> ambient <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> block <targets>")]
pub fn playsound_sound_block_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> block <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> block <targets> <pos>")]
pub fn playsound_sound_block_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> block <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> block <targets> <pos> <volume>")]
pub fn playsound_sound_block_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> block <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> block <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_block_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> block <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> block <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_block_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> block <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> hostile <targets>")]
pub fn playsound_sound_hostile_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> hostile <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> hostile <targets> <pos>")]
pub fn playsound_sound_hostile_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> hostile <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> hostile <targets> <pos> <volume>")]
pub fn playsound_sound_hostile_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> hostile <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> hostile <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_hostile_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> hostile <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> hostile <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_hostile_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> hostile <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> master <targets>")]
pub fn playsound_sound_master_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> master <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> master <targets> <pos>")]
pub fn playsound_sound_master_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> master <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> master <targets> <pos> <volume>")]
pub fn playsound_sound_master_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> master <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> master <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_master_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> master <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> master <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_master_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> master <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> music <targets>")]
pub fn playsound_sound_music_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> music <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> music <targets> <pos>")]
pub fn playsound_sound_music_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> music <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> music <targets> <pos> <volume>")]
pub fn playsound_sound_music_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> music <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> music <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_music_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> music <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> music <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_music_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> music <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> neutral <targets>")]
pub fn playsound_sound_neutral_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> neutral <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> neutral <targets> <pos>")]
pub fn playsound_sound_neutral_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> neutral <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> neutral <targets> <pos> <volume>")]
pub fn playsound_sound_neutral_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> neutral <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> neutral <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_neutral_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> neutral <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> neutral <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_neutral_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> neutral <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> player <targets>")]
pub fn playsound_sound_player_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> player <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> player <targets> <pos>")]
pub fn playsound_sound_player_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> player <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> player <targets> <pos> <volume>")]
pub fn playsound_sound_player_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> player <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> player <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_player_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> player <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> player <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_player_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> player <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> record <targets>")]
pub fn playsound_sound_record_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> record <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> record <targets> <pos>")]
pub fn playsound_sound_record_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> record <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> record <targets> <pos> <volume>")]
pub fn playsound_sound_record_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> record <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> record <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_record_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> record <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> record <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_record_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> record <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> voice <targets>")]
pub fn playsound_sound_voice_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> voice <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> voice <targets> <pos>")]
pub fn playsound_sound_voice_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> voice <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> voice <targets> <pos> <volume>")]
pub fn playsound_sound_voice_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> voice <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> voice <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_voice_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> voice <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> voice <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_voice_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> voice <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> weather <targets>")]
pub fn playsound_sound_weather_targets(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> weather <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> weather <targets> <pos>")]
pub fn playsound_sound_weather_targets_pos(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> weather <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> weather <targets> <pos> <volume>")]
pub fn playsound_sound_weather_targets_pos_volume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> weather <targets> <pos> <volume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> weather <targets> <pos> <volume> <pitch>")]
pub fn playsound_sound_weather_targets_pos_volume_pitch(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> weather <targets> <pos> <volume> <pitch>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="playsound <sound> weather <targets> <pos> <volume> <pitch> <minVolume>")]
pub fn playsound_sound_weather_targets_pos_volume_pitch_minVolume(
    ctx: &mut CommandCtx,
    _sound:ResourceLocation,
    _targets:MultiplePlayers,
    _pos:Vec3,
    _volume:FloatArgumentPositive,
    _pitch:FloatArgumentBetween0And2,
    _minVolume:FloatArgumentBetween0And1
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"playsound <sound> weather <targets> <pos> <volume> <pitch> <minVolume>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="publish")]
pub fn publish(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"publish\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="publish <port>")]
pub fn publish_port(
    ctx: &mut CommandCtx,
    _port:IntegerArgumentBetween0And65535
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"publish <port>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="recipe give <targets> *")]
pub fn recipe_give_targets_star(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"recipe give <targets> *\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/

#[command(usage="recipe give <targets> <recipe>")]
pub fn recipe_give_targets_recipe(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _recipe:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"recipe give <targets> <recipe>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="recipe take <targets> *")]
pub fn recipe_take_targets_star(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"recipe take <targets> *\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/

#[command(usage="recipe take <targets> <recipe>")]
pub fn recipe_take_targets_recipe(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _recipe:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"recipe take <targets> <recipe>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}


#[command(usage="reload")]
pub fn reload(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"reload\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="replaceitem block <pos> <slot> <item>")]
pub fn replaceitem_block_pos_slot_item(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _slot:ItemSlot,
    _item:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"replaceitem block <pos> <slot> <item>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="replaceitem block <pos> <slot> <item> <count>")]
pub fn replaceitem_block_pos_slot_item_count(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _slot:ItemSlot,
    _item:ItemStack,
    _count:IntegerArgumentBetween1And64
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"replaceitem block <pos> <slot> <item> <count>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="replaceitem entity <targets> <slot> <item>")]
pub fn replaceitem_entity_targets_slot_item(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _slot:ItemSlot,
    _item:ItemStack
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"replaceitem entity <targets> <slot> <item>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="replaceitem entity <targets> <slot> <item> <count>")]
pub fn replaceitem_entity_targets_slot_item_count(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _slot:ItemSlot,
    _item:ItemStack,
    _count:IntegerArgumentBetween1And64
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"replaceitem entity <targets> <slot> <item> <count>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="save-all")]
pub fn save_all(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"save-all\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="save-all flush")]
pub fn save_all_flush(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"save-all flush\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="save-off")]
pub fn save_off(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"save-off\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="save-on")]
pub fn save_on(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"save-on\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="say <message>")]
pub fn say_message(
    ctx: &mut CommandCtx,
    _message:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"say <message>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="schedule clear <function>")]
pub fn schedule_clear_function(
    ctx: &mut CommandCtx,
    _function:StringArgumentGreedy
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"schedule clear <function>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="schedule function <function> <time>")]
pub fn schedule_function_function_time(
    ctx: &mut CommandCtx,
    _function:MinecraftFunction,
    _time:Time
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"schedule function <function> <time>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="schedule function <function> <time> append")]
pub fn schedule_function_function_time_append(
    ctx: &mut CommandCtx,
    _function:MinecraftFunction,
    _time:Time
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"schedule function <function> <time> append\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="schedule function <function> <time> replace")]
pub fn schedule_function_function_time_replace(
    ctx: &mut CommandCtx,
    _function:MinecraftFunction,
    _time:Time
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"schedule function <function> <time> replace\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives add <objective> <criteria>")]
pub fn scoreboard_objectives_add_objective_criteria(
    ctx: &mut CommandCtx,
    _objective:StringArgumentWord,
    _criteria:ObjectiveCriteria
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives add <objective> <criteria>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives add <objective> <criteria> <displayName>")]
pub fn scoreboard_objectives_add_objective_criteria_displayName(
    ctx: &mut CommandCtx,
    _objective:StringArgumentWord,
    _criteria:ObjectiveCriteria,
    _displayName:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives add <objective> <criteria> <displayName>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives list")]
pub fn scoreboard_objectives_list(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives modify <objective> displayname <displayName>")]
pub fn scoreboard_objectives_modify_objective_displayname_displayName(
    ctx: &mut CommandCtx,
    _objective:Objective,
    _displayName:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives modify <objective> displayname <displayName>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives modify <objective> rendertype hearts")]
pub fn scoreboard_objectives_modify_objective_rendertype_hearts(
    ctx: &mut CommandCtx,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives modify <objective> rendertype hearts\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives modify <objective> rendertype integer")]
pub fn scoreboard_objectives_modify_objective_rendertype_integer(
    ctx: &mut CommandCtx,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives modify <objective> rendertype integer\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives remove <objective>")]
pub fn scoreboard_objectives_remove_objective(
    ctx: &mut CommandCtx,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives remove <objective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives setdisplay <slot>")]
pub fn scoreboard_objectives_setdisplay_slot(
    ctx: &mut CommandCtx,
    _slot:ScoreboardSlot
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives setdisplay <slot>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard objectives setdisplay <slot> <objective>")]
pub fn scoreboard_objectives_setdisplay_slot_objective(
    ctx: &mut CommandCtx,
    _slot:ScoreboardSlot,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard objectives setdisplay <slot> <objective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players add <targets> <objective> <score>")]
pub fn scoreboard_players_add_targets_objective_score(
    ctx: &mut CommandCtx,
    _targets:MultipleScoreHolders,
    _objective:Objective,
    _score:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players add <targets> <objective> <score>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players enable <targets> <objective>")]
pub fn scoreboard_players_enable_targets_objective(
    ctx: &mut CommandCtx,
    _targets:MultipleScoreHolders,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players enable <targets> <objective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players get <target> <objective>")]
pub fn scoreboard_players_get_target_objective(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players get <target> <objective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players list")]
pub fn scoreboard_players_list(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players list <target>")]
pub fn scoreboard_players_list_target(
    ctx: &mut CommandCtx,
    _target:SingleScoreHolder
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players list <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players operation <targets> <targetObjective> <operation> <source> <sourceObjective>")]
pub fn scoreboard_players_operation_targets_targetObjective_operation_source_sourceObjective(
    ctx: &mut CommandCtx,
    _targets:MultipleScoreHolders,
    _targetObjective:Objective,
    _operation:Operation,
    _source:MultipleScoreHolders,
    _sourceObjective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players operation <targets> <targetObjective> <operation> <source> <sourceObjective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players remove <targets> <objective> <score>")]
pub fn scoreboard_players_remove_targets_objective_score(
    ctx: &mut CommandCtx,
    _targets:MultipleScoreHolders,
    _objective:Objective,
    _score:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players remove <targets> <objective> <score>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players reset <targets>")]
pub fn scoreboard_players_reset_targets(
    ctx: &mut CommandCtx,
    _targets:MultipleScoreHolders
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players reset <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players reset <targets> <objective>")]
pub fn scoreboard_players_reset_targets_objective(
    ctx: &mut CommandCtx,
    _targets:MultipleScoreHolders,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players reset <targets> <objective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="scoreboard players set <targets> <objective> <score>")]
pub fn scoreboard_players_set_targets_objective_score(
    ctx: &mut CommandCtx,
    _targets:MultipleScoreHolders,
    _objective:Objective,
    _score:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"scoreboard players set <targets> <objective> <score>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="seed")]
pub fn seed(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"seed\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="setblock <pos> <block>")]
pub fn setblock_pos_block(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"setblock <pos> <block>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="setblock <pos> <block> destroy")]
pub fn setblock_pos_block_destroy(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"setblock <pos> <block> destroy\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="setblock <pos> <block> keep")]
pub fn setblock_pos_block_keep(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"setblock <pos> <block> keep\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="setblock <pos> <block> replace")]
pub fn setblock_pos_block_replace(
    ctx: &mut CommandCtx,
    _pos:Coordinates,
    _block:BlockState
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"setblock <pos> <block> replace\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="setidletimeout <minutes>")]
pub fn setidletimeout_minutes(
    ctx: &mut CommandCtx,
    _minutes:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"setidletimeout <minutes>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="setworldspawn")]
pub fn setworldspawn(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"setworldspawn\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="setworldspawn <pos>")]
pub fn setworldspawn_pos(
    ctx: &mut CommandCtx,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"setworldspawn <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spawnpoint")]
pub fn spawnpoint(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spawnpoint\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spawnpoint <targets>")]
pub fn spawnpoint_targets(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spawnpoint <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spawnpoint <targets> <pos>")]
pub fn spawnpoint_targets_pos(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _pos:Coordinates
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spawnpoint <targets> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spectate")]
pub fn spectate(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spectate\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spectate <target>")]
pub fn spectate_target(
    ctx: &mut CommandCtx,
    _target:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spectate <target>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spectate <target> <player>")]
pub fn spectate_target_player(
    ctx: &mut CommandCtx,
    _target:SingleEntities,
    _player:SinglePlayer
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spectate <target> <player>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spreadplayers <center> <spreadDistance> <maxRange> under <maxHeight> <respectTeams> <targets>")]
pub fn spreadplayers_center_spreadDistance_maxRange_under_maxHeight_respectTeams_targets(
    ctx: &mut CommandCtx,
    _center:Vec2,
    _spreadDistance:FloatArgumentPositive,
    _maxRange:FloatArgumentGreaterThen1,
    _maxHeight:IntegerArgumentPositive,
    _respectTeams:BoolArgument,
    _targets:MultipleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spreadplayers <center> <spreadDistance> <maxRange> under <maxHeight> <respectTeams> <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="spreadplayers <center> <spreadDistance> <maxRange> <respectTeams> <targets>")]
pub fn spreadplayers_center_spreadDistance_maxRange_respectTeams_targets(
    ctx: &mut CommandCtx,
    _center:Vec2,
    _spreadDistance:FloatArgumentPositive,
    _maxRange:FloatArgumentGreaterThen1,
    _respectTeams:BoolArgument,
    _targets:MultipleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"spreadplayers <center> <spreadDistance> <maxRange> <respectTeams> <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stop")]
pub fn stop(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stop\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets>")]
pub fn stopsound_targets(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="stopsound <targets> * <sound>")]
pub fn stopsound_targets_star_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> * <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/

#[command(usage="stopsound <targets> ambient")]
pub fn stopsound_targets_ambient(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> ambient\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> ambient <sound>")]
pub fn stopsound_targets_ambient_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> ambient <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> block")]
pub fn stopsound_targets_block(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> block\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> block <sound>")]
pub fn stopsound_targets_block_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> block <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> hostile")]
pub fn stopsound_targets_hostile(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> hostile\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> hostile <sound>")]
pub fn stopsound_targets_hostile_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> hostile <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> master")]
pub fn stopsound_targets_master(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> master\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> master <sound>")]
pub fn stopsound_targets_master_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> master <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> music")]
pub fn stopsound_targets_music(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> music\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> music <sound>")]
pub fn stopsound_targets_music_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> music <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> neutral")]
pub fn stopsound_targets_neutral(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> neutral\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> neutral <sound>")]
pub fn stopsound_targets_neutral_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> neutral <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> player")]
pub fn stopsound_targets_player(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> player\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> player <sound>")]
pub fn stopsound_targets_player_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> player <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> record")]
pub fn stopsound_targets_record(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> record\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> record <sound>")]
pub fn stopsound_targets_record_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> record <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> voice")]
pub fn stopsound_targets_voice(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> voice\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> voice <sound>")]
pub fn stopsound_targets_voice_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> voice <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> weather")]
pub fn stopsound_targets_weather(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> weather\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="stopsound <targets> weather <sound>")]
pub fn stopsound_targets_weather_sound(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _sound:ResourceLocation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"stopsound <targets> weather <sound>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="summon <entity>")]
pub fn summon_entity(
    ctx: &mut CommandCtx,
    _entity:EntitySummon
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"summon <entity>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="summon <entity> <pos>")]
pub fn summon_entity_pos(
    ctx: &mut CommandCtx,
    _entity:EntitySummon,
    _pos:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"summon <entity> <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="summon <entity> <pos> <nbt>")]
pub fn summon_entity_pos_nbt(
    ctx: &mut CommandCtx,
    _entity:EntitySummon,
    _pos:Vec3,
    _nbt:NbtCommandTag
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"summon <entity> <pos> <nbt>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tag <targets> add <name>")]
pub fn tag_targets_add_name(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _name:StringArgumentWord
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tag <targets> add <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tag <targets> list")]
pub fn tag_targets_list(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tag <targets> list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tag <targets> remove <name>")]
pub fn tag_targets_remove_name(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _name:StringArgumentWord
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tag <targets> remove <name>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team add <team>")]
pub fn team_add_team(
    ctx: &mut CommandCtx,
    _team:StringArgumentWord
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team add <team>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team add <team> <displayName>")]
pub fn team_add_team_displayName(
    ctx: &mut CommandCtx,
    _team:StringArgumentWord,
    _displayName:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team add <team> <displayName>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team empty <team>")]
pub fn team_empty_team(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team empty <team>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team join <team>")]
pub fn team_join_team(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team join <team>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team join <team> <members>")]
pub fn team_join_team_members(
    ctx: &mut CommandCtx,
    _team:Team,
    _members:MultipleScoreHolders
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team join <team> <members>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team leave <members>")]
pub fn team_leave_members(
    ctx: &mut CommandCtx,
    _members:MultipleScoreHolders
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team leave <members>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team list")]
pub fn team_list(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team list <team>")]
pub fn team_list_team(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team list <team>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> collisionRule always")]
pub fn team_modify_team_collisionRule_always(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> collisionRule always\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> collisionRule never")]
pub fn team_modify_team_collisionRule_never(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> collisionRule never\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> collisionRule pushOtherTeams")]
pub fn team_modify_team_collisionRule_pushOtherTeams(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> collisionRule pushOtherTeams\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> collisionRule pushOwnTeam")]
pub fn team_modify_team_collisionRule_pushOwnTeam(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> collisionRule pushOwnTeam\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> color <value>")]
pub fn team_modify_team_color_value(
    ctx: &mut CommandCtx,
    _team:Team,
    _value:Color
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> color <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> deathMessageVisibility always")]
pub fn team_modify_team_deathMessageVisibility_always(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> deathMessageVisibility always\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> deathMessageVisibility hideForOtherTeams")]
pub fn team_modify_team_deathMessageVisibility_hideForOtherTeams(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> deathMessageVisibility hideForOtherTeams\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> deathMessageVisibility hideForOwnTeam")]
pub fn team_modify_team_deathMessageVisibility_hideForOwnTeam(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> deathMessageVisibility hideForOwnTeam\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> deathMessageVisibility never")]
pub fn team_modify_team_deathMessageVisibility_never(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> deathMessageVisibility never\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> displayName <displayName>")]
pub fn team_modify_team_displayName_displayName(
    ctx: &mut CommandCtx,
    _team:Team,
    _displayName:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> displayName <displayName>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> friendlyFire <allowed>")]
pub fn team_modify_team_friendlyFire_allowed(
    ctx: &mut CommandCtx,
    _team:Team,
    _allowed:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> friendlyFire <allowed>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> nametagVisibility always")]
pub fn team_modify_team_nametagVisibility_always(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> nametagVisibility always\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> nametagVisibility hideForOtherTeams")]
pub fn team_modify_team_nametagVisibility_hideForOtherTeams(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> nametagVisibility hideForOtherTeams\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> nametagVisibility hideForOwnTeam")]
pub fn team_modify_team_nametagVisibility_hideForOwnTeam(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> nametagVisibility hideForOwnTeam\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> nametagVisibility never")]
pub fn team_modify_team_nametagVisibility_never(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> nametagVisibility never\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> prefix <prefix>")]
pub fn team_modify_team_prefix_prefix(
    ctx: &mut CommandCtx,
    _team:Team,
    _prefix:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> prefix <prefix>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> seeFriendlyInvisibles <allowed>")]
pub fn team_modify_team_seeFriendlyInvisibles_allowed(
    ctx: &mut CommandCtx,
    _team:Team,
    _allowed:BoolArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> seeFriendlyInvisibles <allowed>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team modify <team> suffix <suffix>")]
pub fn team_modify_team_suffix_suffix(
    ctx: &mut CommandCtx,
    _team:Team,
    _suffix:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team modify <team> suffix <suffix>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="team remove <team>")]
pub fn team_remove_team(
    ctx: &mut CommandCtx,
    _team:Team
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"team remove <team>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teammsg <message>")]
pub fn teammsg_message(
    ctx: &mut CommandCtx,
    _message:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teammsg <message>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <destination>")]
pub fn teleport_destination(
    ctx: &mut CommandCtx,
    _destination:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <destination>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <location>")]
pub fn teleport_location(
    ctx: &mut CommandCtx,
    _location:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <location>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <targets> <destination>")]
pub fn teleport_targets_destination(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _destination:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <targets> <destination>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <targets> <location>")]
pub fn teleport_targets_location(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <targets> <location>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <targets> <location> facing entity <facingEntity>")]
pub fn teleport_targets_location_facing_entity_facingEntity(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _facingEntity:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <targets> <location> facing entity <facingEntity>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <targets> <location> facing entity <facingEntity> <facingAnchor>")]
pub fn teleport_targets_location_facing_entity_facingEntity_facingAnchor(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _facingEntity:SingleEntities,
    _facingAnchor:EntityAnchor
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <targets> <location> facing entity <facingEntity> <facingAnchor>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <targets> <location> facing <facingLocation>")]
pub fn teleport_targets_location_facing_facingLocation(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _facingLocation:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <targets> <location> facing <facingLocation>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="teleport <targets> <location> <rotation>")]
pub fn teleport_targets_location_rotation(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _rotation:Rotation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"teleport <targets> <location> <rotation>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tell <targets> <message>")]
pub fn tell_targets_message(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _message:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tell <targets> <message>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tellraw <targets> <message>")]
pub fn tellraw_targets_message(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _message:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tellraw <targets> <message>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time add <time>")]
pub fn time_add_time(
    ctx: &mut CommandCtx,
    _time:Time
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time add <time>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time query day")]
pub fn time_query_day(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time query day\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time query daytime")]
pub fn time_query_daytime(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time query daytime\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time query gametime")]
pub fn time_query_gametime(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time query gametime\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time set day")]
pub fn time_set_day(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time set day\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time set midnight")]
pub fn time_set_midnight(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time set midnight\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time set night")]
pub fn time_set_night(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time set night\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time set noon")]
pub fn time_set_noon(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time set noon\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="time set <time>")]
pub fn time_set_time(
    ctx: &mut CommandCtx,
    _time:Time
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"time set <time>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="title <targets> actionbar <title>")]
pub fn title_targets_actionbar_title(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _title:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"title <targets> actionbar <title>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="title <targets> clear")]
pub fn title_targets_clear(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"title <targets> clear\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="title <targets> reset")]
pub fn title_targets_reset(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"title <targets> reset\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="title <targets> subtitle <title>")]
pub fn title_targets_subtitle_title(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _title:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"title <targets> subtitle <title>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="title <targets> times <fadeIn> <stay> <fadeOut>")]
pub fn title_targets_times_fadeIn_stay_fadeOut(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _fadeIn:IntegerArgumentPositive,
    _stay:IntegerArgumentPositive,
    _fadeOut:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"title <targets> times <fadeIn> <stay> <fadeOut>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="title <targets> title <title>")]
pub fn title_targets_title_title(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _title:Component
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"title <targets> title <title>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tm <message>")]
pub fn tm_message(
    ctx: &mut CommandCtx,
    _message:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tm <message>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <destination>")]
pub fn tp_destination(
    ctx: &mut CommandCtx,
    _destination:SingleEntities
) -> anyhow::Result<()> {

    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <destination>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <location>")]
pub fn tp_location(
    ctx: &mut CommandCtx,
    _location:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <location>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <targets> <destination>")]
pub fn tp_targets_destination(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _destination:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <targets> <destination>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <targets> <location>")]
pub fn tp_targets_location(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <targets> <location>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <targets> <location> facing entity <facingEntity>")]
pub fn tp_targets_location_facing_entity_facingEntity(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _facingEntity:SingleEntities
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <targets> <location> facing entity <facingEntity>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <targets> <location> facing entity <facingEntity> <facingAnchor>")]
pub fn tp_targets_location_facing_entity_facingEntity_facingAnchor(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _facingEntity:SingleEntities,
    _facingAnchor:EntityAnchor
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <targets> <location> facing entity <facingEntity> <facingAnchor>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <targets> <location> facing <facingLocation>")]
pub fn tp_targets_location_facing_facingLocation(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _facingLocation:Vec3
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <targets> <location> facing <facingLocation>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="tp <targets> <location> <rotation>")]
pub fn tp_targets_location_rotation(
    ctx: &mut CommandCtx,
    _targets:MultipleEntities,
    _location:Vec3,
    _rotation:Rotation
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"tp <targets> <location> <rotation>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="trigger <objective>")]
pub fn trigger_objective(
    ctx: &mut CommandCtx,
    _objective:Objective
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"trigger <objective>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="trigger <objective> add <value>")]
pub fn trigger_objective_add_value(
    ctx: &mut CommandCtx,
    _objective:Objective,
    _value:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"trigger <objective> add <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="trigger <objective> set <value>")]
pub fn trigger_objective_set_value(
    ctx: &mut CommandCtx,
    _objective:Objective,
    _value:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"trigger <objective> set <value>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
/*
#[command(usage="w <targets> <message>")]
pub fn w_targets_message(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _message:Message
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"w <targets> <message>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
*/

#[command(usage="weather clear")]
pub fn weather_clear(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"weather clear\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="weather clear <duration>")]
pub fn weather_clear_duration(
    ctx: &mut CommandCtx,
    _duration:IntegerArgumentBetween0And1000000
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"weather clear <duration>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="weather rain")]
pub fn weather_rain(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"weather rain\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="weather rain <duration>")]
pub fn weather_rain_duration(
    ctx: &mut CommandCtx,
    _duration:IntegerArgumentBetween0And1000000
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"weather rain <duration>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="weather thunder")]
pub fn weather_thunder(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"weather thunder\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="weather thunder <duration>")]
pub fn weather_thunder_duration(
    ctx: &mut CommandCtx,
    _duration:IntegerArgumentBetween0And1000000
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"weather thunder <duration>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="whitelist add <targets>")]
pub fn whitelist_add_targets(
    ctx: &mut CommandCtx,
    _targets:GameProfile
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"whitelist add <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="whitelist list")]
pub fn whitelist_list(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"whitelist list\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="whitelist off")]
pub fn whitelist_off(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"whitelist off\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="whitelist on")]
pub fn whitelist_on(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"whitelist on\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="whitelist reload")]
pub fn whitelist_reload(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"whitelist reload\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="whitelist remove <targets>")]
pub fn whitelist_remove_targets(
    ctx: &mut CommandCtx,
    _targets:GameProfile
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"whitelist remove <targets>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder add <distance>")]
pub fn worldborder_add_distance(
    ctx: &mut CommandCtx,
    _distance:FloatArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder add <distance>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder add <distance> <time>")]
pub fn worldborder_add_distance_time(
    ctx: &mut CommandCtx,
    _distance:FloatArgument,
    _time:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder add <distance> <time>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder center <pos>")]
pub fn worldborder_center_pos(
    ctx: &mut CommandCtx,
    _pos:Vec2
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder center <pos>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder damage amount <damagePerBlock>")]
pub fn worldborder_damage_amount_damagePerBlock(
    ctx: &mut CommandCtx,
    _damagePerBlock:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder damage amount <damagePerBlock>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder damage buffer <distance>")]
pub fn worldborder_damage_buffer_distance(
    ctx: &mut CommandCtx,
    _distance:FloatArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder damage buffer <distance>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder get")]
pub fn worldborder_get(
    ctx: &mut CommandCtx,

) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder get\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder set <distance>")]
pub fn worldborder_set_distance(
    ctx: &mut CommandCtx,
    _distance:FloatArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder set <distance>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder set <distance> <time>")]
pub fn worldborder_set_distance_time(
    ctx: &mut CommandCtx,
    _distance:FloatArgument,
    _time:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder set <distance> <time>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder warning distance <distance>")]
pub fn worldborder_warning_distance_distance(
    ctx: &mut CommandCtx,
    _distance:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder warning distance <distance>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="worldborder warning time <time>")]
pub fn worldborder_warning_time_time(
    ctx: &mut CommandCtx,
    _time:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"worldborder warning time <time>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp add <targets> <amount>")]
pub fn xp_add_targets_amount(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp add <targets> <amount>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp add <targets> <amount> levels")]
pub fn xp_add_targets_amount_levels(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp add <targets> <amount> levels\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp add <targets> <amount> points")]
pub fn xp_add_targets_amount_points(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgument
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp add <targets> <amount> points\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp query <targets> levels")]
pub fn xp_query_targets_levels(
    ctx: &mut CommandCtx,
    _targets:SinglePlayer
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp query <targets> levels\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp query <targets> points")]
pub fn xp_query_targets_points(
    ctx: &mut CommandCtx,
    _targets:SinglePlayer
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp query <targets> points\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp set <targets> <amount>")]
pub fn xp_set_targets_amount(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp set <targets> <amount>\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp set <targets> <amount> levels")]
pub fn xp_set_targets_amount_levels(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp set <targets> <amount> levels\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}

#[command(usage="xp set <targets> <amount> points")]
pub fn xp_set_targets_amount_points(
    ctx: &mut CommandCtx,
    _targets:MultiplePlayers,
    _amount:IntegerArgumentPositive
) -> anyhow::Result<()> {
    if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
    {
        let return_text = Text::from("This command \"xp set <targets> <amount> points\" is not implemented in this version of feather.").gray().italic();
        sender_message_receiver.send(return_text);
    }
    Ok(Some("".to_string()))
}
