//! Gameplay functionality: entities, components, systems, game logic, ...
//!
//! This crate implements most functionality that is generic between
//! client and server, i.e., which does not involve interaction with the network.

#![allow(clippy::unnecessary_wraps)] // systems are required to return Results

mod game;
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

pub mod entities;

pub mod interactable;

pub mod block;
pub mod world_sources;

pub use game::Game;
pub use world::World;
use world_sources::worldgen::SuperflatWorldGeneratorFactory;

/// Registers gameplay systems with the given `Game` and `SystemExecutor`.
pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    view::register(game, systems);
    chunk::entities::register(systems);
    interactable::register(game);
    world::systems::register(game, systems);
    block::register(systems);

    game.add_entity_spawn_callback(entities::add_entity_components);

    game.register_world_source_factory(
        "worldgen",
        Box::new(world_sources::worldgen::WorldgenWorldSourceFactory),
    );
    game.register_world_source_factory("empty", Box::new(world_sources::EmptyWorldSourceFactory));

    game.register_world_generator_factory("flat", Box::new(SuperflatWorldGeneratorFactory));
}
