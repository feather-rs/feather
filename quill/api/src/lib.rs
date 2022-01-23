//! A WebAssembly-based plugin API for Minecraft servers.

pub mod entities;
mod entity;
mod entity_builder;
mod game;
pub mod query;
mod setup;

pub use entity::{Entity, EntityId};
pub use entity_builder::EntityBuilder;
pub use game::Game;
pub use setup::Setup;

#[doc(inline)]
pub use libcraft_blocks::{BlockKind, BlockState};
#[doc(inline)]
pub use libcraft_core::{BlockPosition, ChunkPosition, Gamemode, Position};
#[doc(inline)]
pub use libcraft_particles::{Particle, ParticleKind};
#[doc(inline)]
pub use libcraft_text::*;

#[doc(inline)]
pub use quill_common::{components, entity_init::EntityInit, events, Component};
#[doc(inline)]
pub use uuid::Uuid;

// Needed for macros
#[doc(hidden)]
pub extern crate bincode;
#[doc(hidden)]
pub extern crate quill_sys as sys;

pub use plugin_macro::plugin;

/// Implement this trait for your plugin's struct.
pub trait Plugin: Sized {
    /// Invoked when the plugin is enabled.
    ///
    /// Here, you should register systems and initialize
    /// any plugin state.
    ///
    /// # Warning
    /// This function is called when your plugin _enabled_. That
    /// is not guaranteed to coincide with the time the server starts
    /// up. Do not assume that the server has just started when
    /// this method is called.
    fn enable(game: &mut Game, setup: &mut Setup<Self>) -> Self;

    /// Invoked before the plugin is disabled.
    ///
    /// # Warning
    /// Like [`Plugin::enable`], this method is not necessarily called
    /// when the server shuts down. Users may choose to disable
    /// plugins at another time. Therefore, do not assume that
    /// the server is shutting down when this method is called.
    fn disable(self, game: &mut Game);
}
