//! Gameplay functionality: entities, components, systems, game logic, ...
//!
//! This crate implements most functionality that is generic between
//! client and server, i.e., which does not involve interaction with the network.

mod components;
pub use components::*;

mod game;
use ecs::SystemExecutor;
pub use game::Game;

mod tick_loop;
pub use tick_loop::TickLoop;

pub mod entity;

pub mod view;

pub mod events;

pub mod world_source;

pub mod world;
pub use world::World;

mod chunk_loading;

mod chunk_entities;

pub mod chat;
pub use chat::ChatBox;

/// Registers gameplay systems with the given `Game` and `SystemExecutor`.
pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    view::register(game, systems);
    chunk_loading::register(game, systems);
    chunk_entities::register(systems);
}
