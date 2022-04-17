//! Gameplay functionality: entities, components, systems, game logic, ...
//!
//! This crate implements most functionality that is generic between
//! client and server, i.e., which does not involve interaction with the network.

#![allow(clippy::unnecessary_wraps)] // systems are required to return Results

mod game;
pub use game::Game;
use quill::Game as _;
use vane::SystemExecutor;

mod tick_loop;
pub use tick_loop::TickLoop;

pub mod view;

pub mod window;
pub use window::PlayerWindow;

pub mod events;

pub mod chunk;

pub mod world;

pub mod chat;
pub use chat::ChatBox;

pub mod entities;

pub mod interactable;

pub mod world_sources;

/// Registers gameplay systems with the given `Game` and `SystemExecutor`.
pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    view::register(game, systems);
    chunk::entities::register(systems);
    interactable::register(game);
    world::systems::register(game, systems);

    game.add_entity_spawn_callback(entities::add_entity_components);

    game.register_world_source_factory(
        "worldgen",
        Box::new(world_sources::worldgen::WorldgenWorldSourceFactory),
    );
}
