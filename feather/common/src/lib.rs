//! Gameplay functionality: entities, components, systems, game logic, ...
//!
//! This crate implements most functionality that is generic between
//! client and server, i.e., which does not involve interaction with the network.

#![allow(clippy::unnecessary_wraps)] // systems are required to return Results

pub use chat::ChatBox;
use ecs::SystemExecutor;
pub use game::Game;
pub use tick_loop::TickLoop;
pub use window::Window;
pub use world::World;

mod game;
mod tick_loop;
pub mod view;

pub mod events;
pub mod window;

pub mod chunk;
mod region_worker;

pub mod chat;
pub mod entities;
pub mod world;

pub mod interactable;
pub mod player_count;

/// Registers gameplay systems with the given `Game` and `SystemExecutor`.
pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    view::register(game, systems);
    chunk::loading::register(game, systems);
    chunk::entities::register(systems);
    interactable::register(game);

    game.add_entity_spawn_callback(entities::add_entity_components);
}
