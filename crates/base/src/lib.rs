//! Core functionality for Feather. This crate primarily
//! implements or reexports essential data structures, such as:
//! * Inventories
//! * The block ID system
//! * The chunk data structure
//! * The chunk map (`World`)
//!
//! This crate also exposes the `Setup` and `State` types which are
//! used throughout the rest of the codebase.

use anyhow::anyhow;
use ecs::{Ecs, Entity, Stage, SysResult, SystemExecutor};
use num_derive::{FromPrimitive, ToPrimitive};
use resources::{CantGetResource, Resource, Resources};
use serde::{Deserialize, Serialize};

mod chunk;
pub mod metadata;
mod positions;
pub mod text;
mod world;

pub use blocks::*;
pub use chunk::{Chunk, ChunkSection, CHUNK_HEIGHT, CHUNK_WIDTH};
pub use generated::{Area, Biome, EntityKind, Item, ItemStack, Particle, Window};
#[doc(inline)]
pub use metadata::EntityMetadata;
pub use positions::*;
pub use text::{deserialize_text, Text};
pub use world::World;

/// A function that is called before an entity is despawned.
pub type DespawnCallback = fn(&mut State, Entity) -> SysResult;

/// Struct passed to all systems as their sole argument.
///
/// This type encapsulates all state needed during execution:
/// * The `Ecs`, which stores all entities in the current game.
/// * The `World`, which stores block data for the current world.
/// (This is _not_ the same as the "world" referred to in many Rust
/// ECS libraries; for entities see `Ecs`. We've renamed `World` to `Ecs`
/// to avoid confusion.)
/// * The `Resources`, which store additional unique data that can be used
/// throughout the codebase.
///
/// This struct can be created through `Setup::build()`.
pub struct State {
    /// Stores all entities in the current game.
    pub ecs: Ecs,
    /// Stores blocks and chunks in the world.
    pub world: World,

    resources: Resources,

    /// Functions to invoke before an entity is despawned.
    despawn_callbacks: Option<Vec<DespawnCallback>>,
}

impl State {
    /// Gets a reference to the resource with the given type.
    ///
    /// Returns an error if the resource does not exist
    /// (was not inserted with `Setup::resource()`),
    /// or if there is already a mutable borrow to that resource.
    pub fn resource<T: Resource>(&self) -> Result<resources::Ref<T>, CantGetResource> {
        self.resources.get()
    }

    /// Gets a mutable reference to the resource with the given type.
    ///
    /// Returns an error if the resource does not exist
    /// or if there is already a borrow to that resource.
    pub fn resource_mut<T: Resource>(&self) -> Result<resources::RefMut<T>, CantGetResource> {
        self.resources.get_mut()
    }

    /// Despawns an entity.
    ///
    /// Use this function instead of calling `ecs.spawn()`.
    /// This function will invoke all _despawn callbacks_,
    /// which ensures that events are properly handled.
    pub fn despawn(&mut self, entity: Entity) -> SysResult {
        let despawn_callbacks = self
            .despawn_callbacks
            .take()
            .ok_or_else(|| anyhow!("despawn callbacks cannot recursively despawn entities"))?;
        for &callback in &despawn_callbacks {
            callback(self, entity)?;
        }
        self.despawn_callbacks.replace(despawn_callbacks);

        self.ecs.inner_mut().despawn(entity).map_err(From::from)
    }
}

/// Struct passed into functions which set up the server
/// state during startup.
///
/// Currently, this struct is used to:
/// * Register systems with the system executor.
/// * Insert _resources_, data that can be accessed
/// through the method `State::resource()`.
#[derive(Default)]
pub struct Setup {
    executor: SystemExecutor<State>,
    resources: Resources,
    despawn_callbacks: Vec<DespawnCallback>,
}

impl Setup {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a system in the default `Tick` stage.
    ///
    /// The system will be invoked each tick.
    pub fn system(&mut self, system: fn(&mut State) -> SysResult) -> &mut Self {
        self.system_in_stage(system, Stage::Tick)
    }

    /// Registers a system to the specified stage.
    ///
    /// The system will be invoked each tick.
    pub fn system_in_stage(
        &mut self,
        system: fn(&mut State) -> SysResult,
        stage: Stage,
    ) -> &mut Self {
        self.executor.add_system(stage, system);
        self
    }

    /// Inserts a resource. Systems can then access
    /// this resource by calling `State::resource()`.
    ///
    /// Resources can be any type that is `Send + Sync + 'static`.
    pub fn resource(&mut self, resource: impl Resource) -> &mut Self {
        self.resources.insert(resource);
        self
    }

    /// Registers a despawn callback which is invoked before an entity
    /// is despawned.
    pub fn despawn_callback(&mut self, callback: DespawnCallback) -> &mut Self {
        self.despawn_callbacks.push(callback);
        self
    }

    /// Completes setup, returning a `Tick` and a `SystemExecutor`.
    pub fn build(self) -> (State, SystemExecutor<State>) {
        (
            State {
                resources: self.resources,
                ecs: Ecs::new(),
                world: World::new(),
                despawn_callbacks: Some(self.despawn_callbacks),
            },
            self.executor,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    FromPrimitive,
    ToPrimitive,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}