//! The implementations of various commands.

use crate::arguments::Coordinates;
use crate::{
    arguments::{EntitySelector, ParsedGamemode, TextArgument},
    CommandCtx,
};
use feather_core::text::{Text, TextComponentBuilder};
use feather_core::util::{Gamemode, Position};
use feather_server_types::{
    ChatEvent, ChatPosition, GamemodeUpdateEvent, MessageReceiver, Name, Teleported,
};
use fecs::{Entity, World};
use lieutenant::command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TpError {
    #[error("no entities matched the target selector")]
    NoMatchingEntities,
}

#[command(usage = "tp|teleport <destination>")]
pub fn tp_1(ctx: &mut CommandCtx, destination: EntitySelector) -> anyhow::Result<()> {
    if let Some(first) = destination.entities.first() {
        if let Some(pos) = ctx.world.try_get::<Position>(*first).map(|r| *r) {
            teleport_entity_to_pos(&mut ctx.world, ctx.sender, pos);
        }

        Ok(())
    } else {
        Err(TpError::NoMatchingEntities.into())
    }
}

#[command(usage = "tp|teleport <location>")]
pub fn tp_2(ctx: &mut CommandCtx, location: Coordinates) -> anyhow::Result<()> {
    teleport_entity(&mut ctx.world, ctx.sender, location);
    Ok(())
}

#[command(usage = "tp|teleport <targets> <location>")]
pub fn tp_3(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    location: Coordinates,
) -> anyhow::Result<()> {
    for entity in &targets.entities {
        teleport_entity(&mut ctx.world, *entity, location);
    }

    Ok(())
}

#[command(usage = "tp|teleport <targets> <destination>")]
pub fn tp_4(
    ctx: &mut CommandCtx,
    targets: EntitySelector,
    destination: EntitySelector,
) -> anyhow::Result<()> {
    if let Some(location) = destination
        .entities
        .first()
        .map(|e| ctx.world.try_get::<Position>(*e).map(|r| *r))
        .flatten()
    {
        for entity in targets.entities {
            teleport_entity_to_pos(&mut ctx.world, entity, location);
        }
        Ok(())
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
    Ok(())
}

#[command(usage = "gamemode <gamemode> <target>")]
pub fn gamemode_2(
    ctx: &mut CommandCtx,
    gamemode: ParsedGamemode,
    target: EntitySelector,
) -> anyhow::Result<()> {
    for entity in target.entities {
        update_gamemode(ctx, gamemode.0, entity)
    }

    Ok(())
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

    Ok(())
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

    Ok(())
}

#[command(usage = "me <action>")]
pub fn me(ctx: &mut CommandCtx, action: TextArgument) -> anyhow::Result<()> {
    let name = ctx.world.try_get::<Name>(ctx.sender);

    let sender_name = if let Some(name) = &name { &name.0 } else { "@" };
    let command_output = Text::from(format!("* {} {}", sender_name, action.0));

    drop(name);

    ctx.game.handle(
        &mut ctx.world,
        ChatEvent {
            message: command_output.into(),
            position: ChatPosition::Chat,
        },
    );

    Ok(())
}