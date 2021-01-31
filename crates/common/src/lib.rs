//! Gameplay functionality: entities, components, systems, game logic, ...
//!
//! This crate implements most functionality that is generic between
//! client and server, i.e., which does not involve interaction with the network.

mod components;
pub use components::*;

mod game;
pub use game::Game;

mod tick_loop;
pub use tick_loop::TickLoop;
