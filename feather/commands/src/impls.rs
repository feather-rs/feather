//! The implementations of various commands.

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use lieutenant::command;
use smallvec::SmallVec;
use thiserror::Error;

use base::{Gamemode, Item, Position, Text, TextComponentBuilder, TextValue};
use common::{ChatBox, Window};
use ecs::{Ecs, Entity};
use quill_common::components::{ClientId, Name};
use quill_common::entities::Player;
use quill_common::events::{GamemodeUpdateEvent, InventoryUpdateEvent, TimeUpdateEvent};

use crate::arguments::find_selected_entities;
use crate::{
    arguments::{
        Coordinates, EntitySelector, ItemArgument, ParsedGamemode, PositiveI32Argument,
        TextArgument, TimeArgument, TimeQueryInformation, TimeSpec,
    },
    CommandCtx,
};

#[derive(Debug, Error)]
pub enum TpError {
    #[error("No entity was found")]
    NoMatchingEntities,
    #[error("Only one entity is allowed, but the provided selector allows for more than one")]
    TooManyEntities,
}

#[command(usage = "tp|teleport <destination>")]
pub fn tp_1(ctx: &mut CommandCtx, destination: EntitySelector) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &destination.requirements)?;
    if let Some(first) = entities.first() {
        if let Ok(pos) = ctx.ecs.get::<Position>(*first).map(|r| *r) {
            teleport_entity_to_pos(&ctx.ecs, ctx.sender, pos);
        }

        Ok(Some(format!(
            "Teleported {0} to {1}",
            *ctx.ecs.get::<Name>(ctx.sender).unwrap(),
            *ctx.ecs.get::<Name>(*first).unwrap()
        )))
    } else {
        Err(TpError::NoMatchingEntities.into())
    }
}

#[command(usage = "tp|teleport <location>")]
pub fn tp_2(ctx: &mut CommandCtx, location: Coordinates) -> anyhow::Result<Option<String>> {
    teleport_entity(&ctx.ecs, ctx.sender, location);

    let position = ctx.ecs.get::<Position>(ctx.sender).unwrap();
    Ok(Some(format!(
        "Teleported {0} to {1}, {2}, {3}",
        *ctx.ecs.get::<Name>(ctx.sender).unwrap(),
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
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &targets.requirements)?;
    if entities.is_empty() {
        Err(TpError::NoMatchingEntities.into())
    } else {
        for entity in &entities {
            teleport_entity(&ctx.ecs, *entity, location);
        }

        let position = ctx.ecs.get::<Position>(*entities.first().unwrap()).unwrap();
        Ok(Some(format!(
            "Teleported {0} to {1}, {2}, {3}",
            targets.entities_to_string(ctx, &entities.into_vec(), false),
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
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &targets.requirements)?;
    if entities.len() > 1 {
        Err(TpError::TooManyEntities.into())
    } else if let Some(Ok(location)) = entities
        .first()
        .map(|e| ctx.ecs.get::<Position>(*e).map(|r| *r))
    {
        if entities.is_empty() {
            Err(TpError::NoMatchingEntities.into())
        } else {
            for entity in &entities {
                teleport_entity_to_pos(&ctx.ecs, *entity, location);
            }
            let entities = entities.into_vec();
            Ok(Some(format!(
                "Teleported {0} to {1}",
                targets.entities_to_string(ctx, &entities, false),
                destination.entities_to_string(ctx, &entities, false)
            )))
        }
    } else {
        Err(TpError::NoMatchingEntities.into())
    }
}

fn teleport_entity(ecs: &Ecs, entity: Entity, location: Coordinates) {
    let new_pos = ecs
        .get::<Position>(entity)
        .map(|r| *r)
        .map(|relative_to| location.into_position(relative_to));

    if let Ok(new_pos) = new_pos {
        teleport_entity_to_pos(ecs, entity, new_pos);
    }
}

fn teleport_entity_to_pos(ecs: &Ecs, entity: Entity, pos: Position) {
    if let Ok(mut old_pos) = ecs.get_mut::<Position>(entity) {
        *old_pos = pos;
    }
    //let _ = game.ecs.entity(entity).add(Teleported);
}

#[command(usage = "gamemode <gamemode>")]
pub fn gamemode_1(
    ctx: &mut CommandCtx,
    gamemode: ParsedGamemode,
) -> anyhow::Result<Option<String>> {
    update_gamemode(ctx, gamemode.0, ctx.sender);
    Ok(Some(format!("Set own gamemode to {:?} Mode", gamemode.0)))
}

#[command(usage = "gamemode <gamemode> <target>")]
pub fn gamemode_2(
    ctx: &mut CommandCtx,
    gamemode: ParsedGamemode,
    target: EntitySelector,
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &target.requirements)?;
    for entity in &entities {
        update_gamemode(ctx, gamemode.0, *entity)
    }

    if entities.len() == 1 && *entities.first().unwrap() == ctx.sender {
        return Ok(Some(format!("Set own gamemode to {:?} Mode", gamemode.0)));
    }
    Ok(Some(format!(
        "Changed gamemode of {} to {:?} Mode",
        target.entities_to_string(ctx, &entities.into_vec(), false),
        gamemode.0
    )))
}

fn update_gamemode(ctx: &mut CommandCtx, gamemode: Gamemode, entity: Entity) {
    let event = if let Ok(mut old) = ctx.ecs.get_mut::<Gamemode>(ctx.sender) {
        let old_val = *old;
        *old = gamemode;

        let event = GamemodeUpdateEvent {
            old: old_val,
            new: gamemode,
        };
        Some(event)
    } else {
        None
    };

    if let Some(event) = event {
        ctx.ecs.insert_entity_event(entity, event).unwrap();
    }
}

#[command(usage = "tell|msg|w <target> <message>")]
pub fn whisper(
    ctx: &mut CommandCtx,
    target: EntitySelector,
    message: TextArgument,
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &target.requirements)?;
    let sender_name = if let Ok(sender_name) = ctx.ecs.get::<Name>(ctx.sender) {
        (**sender_name).to_owned()
    } else {
        // Use a default value if the executor has no Name component
        String::from("Server")
    };

    // The message that is returned to the whisperer
    // You whisper to [player] (and [player]): [message]
    let mut response_message = String::from("You whisper to");

    // Tracks if there needs to be "and" before the next player added to the response message
    let mut needs_and = false;

    for entity in entities {
        if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
            chat.send_system(
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

        if let Ok(player_name) = ctx.ecs.get::<Name>(entity) {
            if needs_and {
                response_message += format!(" and {}", *player_name).as_str();
            } else {
                needs_and = true;

                response_message += format!(" {}", *player_name).as_str();
            }
        }
    }

    // Send the whisperer a confirmation message
    if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
        response_message += format!(": {}", message.0).as_str();
        let return_text = Text::from(response_message).gray().italic();

        chat.send_system(return_text);
    }

    Ok(None)
}

#[command(usage = "say <message>")]
pub fn say(ctx: &mut CommandCtx, message: TextArgument) -> anyhow::Result<Option<String>> {
    let name = ctx.ecs.get::<Name>(ctx.sender);

    let sender_name = if let Ok(name) = &name {
        &***name
    } else {
        "Server"
    };

    let command_output = Text::from(format!("[{}] {}", sender_name, message.0));

    drop(name);

    ctx.ecs
        .query::<&mut ChatBox>()
        .iter()
        .for_each(|(_, chat_box)| chat_box.send_chat(command_output.clone()));

    Ok(None)
}

#[command(usage = "me <action>")]
pub fn me(ctx: &mut CommandCtx, action: TextArgument) -> anyhow::Result<Option<String>> {
    let command_output = {
        let name = ctx.ecs.get::<Name>(ctx.sender);
        let sender_name = name.as_deref().map_or("@", |n| n);
        Text::from(format!("* {} {}", sender_name, action.as_ref()))
    };

    ctx.ecs
        .query::<&mut ChatBox>()
        .iter()
        .for_each(|(_, chat_box)| chat_box.send_chat(command_output.clone()));

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
pub fn kick_1(ctx: &mut CommandCtx, targets: EntitySelector) -> anyhow::Result<Option<String>> {
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
) -> anyhow::Result<Option<String>> {
    kick_players(ctx, &targets, reason.0.into())
}

fn kick_players(
    ctx: &mut CommandCtx,
    targets: &EntitySelector,
    reason: Text,
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &targets.requirements)?;
    for entity in &entities {
        if ctx.ecs.get::<Player>(*entity).is_err() {
            return Err(KickError::NoEntities.into());
        }
    }

    for entity in &entities {
        let name = (**ctx.ecs.get::<Name>(*entity).unwrap()).to_owned();
        let _client_id = ctx.ecs.get::<ClientId>(*entity).unwrap();

        // Send confirmation message
        // TODO Server ops should also see the message
        if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
            let kick_confirm = Text::translate_with(
                "commands.kick.success",
                vec![Text::from(name), reason.clone()],
            );
            chat.send_system(kick_confirm);
        }
    }
    Ok(None)
}

#[command(usage = "stop")]
pub fn stop(ctx: &mut CommandCtx) -> anyhow::Result<Option<String>> {
    // Confirmation message
    // TODO Server ops should also see the message
    if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
        let text = Text::from(TextValue::translate("commands.stop.stopping"));
        chat.send_system(text);
    }

    //ctx.game
    //    .resources
    //    .get::<ShutdownChannels>()
    //    .tx
    //    .try_send(())?;

    Ok(None)
}

#[allow(clippy::enum_variant_names)]
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
pub fn clear_1(ctx: &mut CommandCtx) -> anyhow::Result<Option<String>> {
    if ctx.ecs.get::<Player>(ctx.sender).is_ok() {
        // Go through the player's inventory and set all the slots to no items.
        // Also, keep track of how many items we delete.
        let mut count = 0;
        clear_items(ctx, ctx.sender, None, i32::MAX, &mut count)?;
        // If count is zero, the player's inventory was empty and the command fails
        // "No items were found on player {0}."
        if count == 0 {
            let name = ctx.ecs.get::<Name>(ctx.sender).unwrap();
            return Err(ClearError::NoItems((**name).to_owned()).into());
        }
        // If the count is not zero, we return the count of items we deleted. Command succeeds.
        // "Removed {1} items from player {0}"
        Ok(Some(format!(
            "Removed {1} items from player {0}",
            *ctx.ecs.get::<Name>(ctx.sender).unwrap(),
            count
        )))
    } else {
        Err(ClearError::NotPlayer.into())
    }
}

#[command(usage = "clear <targets>")]
pub fn clear_2(ctx: &mut CommandCtx, targets: EntitySelector) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &targets.requirements)?;
    let mut players = true;
    for entity in &entities {
        players &= ctx.ecs.get::<Player>(*entity).is_ok();
    }
    if players {
        let mut count = 0;
        for entity in &entities {
            clear_items(ctx, *entity, None, i32::MAX, &mut count)?;
        }
        // If count is zero, the everybody's inventory was empty and the command fails
        // "No items were found on {0} players."
        if count == 0 {
            return Err(ClearError::NoItemsMultiplayer(targets.entities_to_string(
                ctx,
                &entities.into_vec(),
                true,
            ))
            .into());
        }
        // If the count is not zero, we return the count of items we deleted. Command succeeds.
        // "Removed {1} items from {0} players"
        Ok(Some(format!(
            "Removed {1} items from {0}",
            targets.entities_to_string(ctx, &entities.into_vec(), true),
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
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &targets.requirements)?;
    let mut players = true;
    for entity in &entities {
        players &= ctx.ecs.get::<Player>(*entity).is_ok();
    }
    if players {
        let mut count = 0;
        for entity in &entities {
            clear_items(ctx, *entity, Some(item.0), i32::MAX, &mut count)?;
        }
        // If count is zero, the everybody's inventory was empty and the command fails
        // "No items were found on {0} players."
        if count == 0 {
            return Err(ClearError::NoItemsMultiplayer(targets.entities_to_string(
                ctx,
                &entities.into_vec(),
                true,
            ))
            .into());
        }
        // If the count is not zero, we return the count of items we deleted. Command succeeds.
        // "Removed {1} items from {0} players"
        Ok(Some(format!(
            "Removed {1} items from {0}",
            targets.entities_to_string(ctx, &entities.into_vec(), true),
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
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &targets.requirements)?;
    let mut players = true;
    for entity in &entities {
        players &= ctx.ecs.get::<Player>(*entity).is_ok();
    }
    if players {
        let mut count = 0;
        for entity in &entities {
            clear_items(ctx, *entity, Some(item.0), maxcount.0, &mut count)?;
        }
        // If count is zero, the everybody's inventory was empty and the command fails
        // "No items were found on {0} players."
        if count == 0 {
            return Err(ClearError::NoItemsMultiplayer(targets.entities_to_string(
                ctx,
                &entities.into_vec(),
                true,
            ))
            .into());
        }
        // If maxcount is 0, we report not that we removed items, only that we found them.
        if maxcount.0 == 0 {
            Ok(Some(format!(
                "Found {1} matching items on {0}",
                targets.entities_to_string(ctx, &entities.into_vec(), true),
                count
            )))
        } else {
            // If the count is not zero, we return the count of items we deleted. Command succeeds.
            // "Removed {1} items from {0} players"
            Ok(Some(format!(
                "Removed {1} items from {0}",
                targets.entities_to_string(ctx, &entities.into_vec(), true),
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
) -> anyhow::Result<()> {
    let inventory = ctx.ecs.get_mut::<Window>(player).unwrap();
    let mut changed_items: SmallVec<[usize; 2]> = SmallVec::new();
    for (index, slot) in inventory.inner().to_vec().into_iter().enumerate() {
        if let Some(mut stack) = slot {
            if let Some(item_inner) = item {
                if stack.item != item_inner {
                    continue;
                }
            }
            if maxcount == 0 {
                *count += stack.count as i32;
            } else if (stack.count as i32) <= maxcount - *count {
                *count += stack.count as i32;
                inventory.set_item(index, None)?;
                changed_items.push(index);
            } else {
                stack.count -= (maxcount - *count) as u32;
                inventory.set_item(index, Some(stack))?;
                *count = maxcount;
                changed_items.push(index);
                break;
            }
        }
    }

    drop(inventory);

    if !changed_items.is_empty() {
        ctx.ecs
            .insert_entity_event(player, InventoryUpdateEvent(changed_items.into_vec()))?;
    }

    Ok(())
}

#[command(usage = "seed")]
pub fn seed(ctx: &mut CommandCtx) -> anyhow::Result<Option<String>> {
    if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
        let world_seed = "TODO";
        chat.send_system(
            Text::from("Seed: [")
                + Text::from(world_seed.to_string())
                    .green()
                    .on_click_copy_to_clipboard(world_seed.to_string())
                    .on_hover_show_text(TextValue::translate("chat.copy.click"))
                + Text::from("]"),
        );
    }
    Ok(None)
}

#[derive(Debug, Error)]
pub enum BanError {
    #[error(
        "Only players may be affected by this command, but the provided selector includes entities"
    )]
    NotPlayer,
    #[error("Already banned")]
    NoTargets,
}

#[command(usage = "ban <targets> <reason>")]
pub fn ban_withreason(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    reason: TextArgument,
) -> anyhow::Result<Option<String>> {
    ban_players(ctx, targets, reason.0, false)
}

#[command(usage = "ban <targets>")]
pub fn ban_noreason(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
) -> anyhow::Result<Option<String>> {
    ban_players(ctx, targets, "Banned by an operator.".to_owned(), false)
}

#[command(usage = "ban-ip <targets> <reason>")]
pub fn banip_withreason(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    reason: TextArgument,
) -> anyhow::Result<Option<String>> {
    ban_players(ctx, targets, reason.0, true)
}

#[command(usage = "ban-ip <targets>")]
pub fn banip_noreason(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
) -> anyhow::Result<Option<String>> {
    ban_players(ctx, targets, "Banned by an operator.".to_owned(), true)
}

#[derive(Debug, Error)]
pub enum BanIpError {
    #[error("Not a valid IP Address.")]
    InvalidIp,
}

#[command(usage = "ban-ip <ip> <reason>")]
pub fn banip_withreason_ip(
    ctx: &mut CommandCtx,
    ip: String,
    reason: TextArgument,
) -> anyhow::Result<Option<String>> {
    ban_ip(ctx, ip, reason.0)
}

#[command(usage = "ban-ip <ip>")]
pub fn banip_noreason_ip(ctx: &mut CommandCtx, ip: String) -> anyhow::Result<Option<String>> {
    ban_ip(ctx, ip, "IP Banned by an operator.".to_string())
}

pub fn ban_ip(ctx: &mut CommandCtx, ip: String, reason: String) -> anyhow::Result<Option<String>> {
    let ip = IpAddr::from_str(&ip).map_err(|_| BanIpError::InvalidIp)?;

    {
        // TODO
        //let mut bi_lock = ctx.game.resources.get_mut::<WrappedBanInfo>();
        //let mut ban_info = bi_lock.write().unwrap();

        //ban_info.ip_bans.insert(
        //    ip,
        //    Ban {
        //        reason: reason.clone(),
        //        expires_after: None,
        //    },
        //);
    }

    if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
        let ban_confirm = Text::translate_with(
            "commands.ban.success",
            vec![Text::from(ip.to_string()), Text::from(reason)],
        );
        chat.send_system(ban_confirm);
    }

    let ent = ctx
        .ecs
        .query::<&SocketAddr>()
        .iter()
        .find(|(_, x)| x.ip() == ip)
        .map(|(e, _)| e);

    if let Some(_ent) = ent {
        //ctx.game.disconnect_and_log(
        //    ent,
        //    &mut ctx.ecs,
        //    &Text::from(reason),
        //    "Banned by an operator.",
        //);
    }

    Ok(None)
}

pub fn ban_players(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    reason: String,
    _by_ip: bool,
) -> anyhow::Result<Option<String>> {
    let entities = find_selected_entities(ctx, &targets.requirements)?;
    if entities.is_empty() {
        return Err(BanError::NoTargets.into());
    }

    for entity in &entities {
        if ctx.ecs.get::<Player>(*entity).is_ok() {
            return Err(BanError::NotPlayer.into());
        }
    }

    for entity in &entities {
        {
            //let mut bi_lock = ctx.game.resources.get_mut::<WrappedBanInfo>();
            //let mut ban_info = bi_lock.write().unwrap();
            //
            //if by_ip {
            //    let ip = ctx.ecs.get::<SocketAddr>(*entity).unwrap();
            //
            //    ban_info.ip_bans.insert(
            //        ip.ip(),
            //        Ban {
            //            reason: reason.clone(),
            //            expires_after: None,
            //        },
            //    );
            //} else {
            //    let uuid = ctx.ecs.get::<Uuid>(*entity).unwrap();
            //
            //    ban_info.uuid_bans.insert(
            //        uuid.to_hyphenated_ref().to_string(),
            //        Ban {
            //            reason: reason.clone(),
            //            expires_after: None,
            //        },
            //    );
            //}
        }

        let name = (**ctx.ecs.get::<Name>(*entity).unwrap()).to_owned();
        if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
            let ban_confirm = Text::translate_with(
                "commands.ban.success",
                vec![Text::from(name), Text::from(reason.clone())],
            );
            chat.send_system(ban_confirm);
        }

        //ctx.game.disconnect_and_log(
        //    *entity,
        //    &mut ctx.ecs,
        //    &Text::from(reason.clone()),
        //    "Banned by an operator.",
        //);
    }

    Ok(None)
}

#[derive(Debug, Error)]
pub enum PardonError {
    #[error("Couldn't find that players UUID, Have they changed name?")]
    NotPlayer,
}

#[command(usage = "pardon <name>")]
pub fn pardon(ctx: &mut CommandCtx, name: TextArgument) -> anyhow::Result<Option<String>> {
    // Get UUID from name
    //let online_mode = ctx.game.shared.config.server.online_mode;
    //let uuid = if online_mode {
    //    Runtime::new()
    //        .unwrap()
    //        .block_on(name_to_uuid_online(&name.0))
    //} else {
    //    Some(name_to_uuid_offline(&name.0))
    //};

    //let uuid = match uuid {
    //    Some(uuid) => uuid,
    //    None => return Err(PardonError::NotPlayer.into()),
    //};

    //{
    //    let bi_lock = ctx.game.resources.get::<WrappedBanInfo>();
    //    let mut ban_info = bi_lock.write().unwrap();
    //    ban_info
    //        .uuid_bans
    //        .remove(&uuid.to_hyphenated_ref().to_string());
    //}

    if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
        let kick_confirm =
            Text::translate_with("commands.pardon.success", vec![Text::from(name.0)]);
        chat.send_system(kick_confirm);
    }

    Ok(None)
}

#[derive(Debug, Error)]
pub enum PardonIpError {
    #[error("Invalid IP Address")]
    NotIp,
}

#[command(usage = "pardon-ip <ip>")]
pub fn pardonip(ctx: &mut CommandCtx, ip: String) -> anyhow::Result<Option<String>> {
    // Try to parse ip
    let _addr = IpAddr::from_str(&ip).map_err(|_| PardonIpError::NotIp)?;

    //{
    //    let mut bi_lock = ctx.game.resources.get_mut::<WrappedBanInfo>();
    //    let mut ban_info = bi_lock.write().unwrap();
    //    ban_info.ip_bans.remove(&addr);
    //}

    if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
        let kick_confirm = Text::translate_with("commands.pardon.success", vec![Text::from(ip)]);
        chat.send_system(kick_confirm);
    }

    Ok(None)
}

#[command(usage = "time query <info>")]
pub fn time_query(
    ctx: &mut CommandCtx,
    info: TimeQueryInformation,
) -> anyhow::Result<Option<String>> {
    let time = match info {
        TimeQueryInformation::DayTime => ctx.world.time.time(),
        TimeQueryInformation::GameTime => ctx.world.time.world_age(),
        TimeQueryInformation::Day => ctx.world.time.days(),
    };

    if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(ctx.sender) {
        let message =
            Text::translate_with("commands.time.query", vec![Text::from(time.to_string())]);
        chat.send_system(message);
    }

    Ok(None)
}

#[command(usage = "time add <time>")]
pub fn time_add(ctx: &mut CommandCtx, time: TimeArgument) -> anyhow::Result<Option<String>> {
    let time = ctx.world.time.time() + time.0;
    set_time(ctx, time);
    Ok(None)
}

#[command(usage = "time set <time>")]
pub fn time_set_0(ctx: &mut CommandCtx, time: TimeArgument) -> anyhow::Result<Option<String>> {
    set_time(ctx, time.0);
    Ok(None)
}

#[command(usage = "time set <time_spec>")]
pub fn time_set_1(ctx: &mut CommandCtx, time_spec: TimeSpec) -> anyhow::Result<Option<String>> {
    set_time(
        ctx,
        match time_spec {
            TimeSpec::Day => 1_000,
            TimeSpec::Noon => 6_000,
            TimeSpec::Night => 13_000,
            TimeSpec::Midnight => 18_000,
        },
    );
    Ok(None)
}

fn set_time(ctx: &mut CommandCtx, time: u64) {
    ctx.ecs.insert_event(TimeUpdateEvent {
        old: ctx.world.time.time(),
        new: time,
    });
    ctx.world.time.set_time(time);
}
