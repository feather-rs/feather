//! The implementations of various commands.

use crate::arguments::Coordinates;
use crate::{
    arguments::{EntitySelector, ParsedGamemode},
    CommandCtx,
};
use feather_core::util::{Gamemode, Position};
use feather_server_types::{GamemodeUpdateEvent, Teleported};
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
