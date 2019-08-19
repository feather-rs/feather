//! Assorted utilities for use in Feather's codebase.
use bumpalo::Bump;
use feather_core::{ItemStack, Position};
use glm::Vec3;
use spawn::Spawner;
use thread_local::ThreadLocal;

#[macro_use]
mod macros;
mod spawn;

pub use macros::*;
pub use spawn::SpawnerSystem;

/// Converts float-based velocity in blocks per tick
/// to the obnoxious format used by the protocol.
pub fn protocol_velocity(vel: Vec3) -> (i16, i16, i16) {
    // Apparently, these are in units of 1/8000 block per tick.
    (
        (vel.x / 8000.0) as i16,
        (vel.y / 8000.0) as i16,
        (vel.z / 8000.0) as i16,
    )
}

/// General-purpose utility resource, used to
/// ergonomically perform various actions without
/// specifying ridiculous amounts of system dependencies.
///
/// This struct is thread-safe and should never be used
/// as a `Write` dependency.
///
/// # Available functions
/// * `alloc` - used to allocate temporary values
/// using a bump allocator. When allocating values
/// which will only be used inside a function, for example,
/// this function should be used rather than allocating
/// directly on the heap.
/// * `spawn_*` - functions to lazily spawn entities of
/// various types. This avoids the need to specify write
/// storage dependencies for each component the entity
/// needs. Note, however, that the entity isn't created
/// until the handling dispatcher stage. These functions simply
/// redirect to `entity::Spawner`.
#[derive(Debug, Default)]
pub struct Util {
    /// Thread-local bump allocator, reset
    /// every tick.
    ///
    /// This is used to reduce allocation frequency.
    bump: ThreadLocal<Bump>,
    /// The spawner, used to lazily spawn entities.
    spawner: Spawner,
}

impl Util {
    /// Returns the thread-local bump allocator.
    pub fn bump(&self) -> &Bump {
        self.bump.get_default()
    }

    /// Allocates a value using the bump allocator
    /// for this thread.
    ///
    /// This is equivalent to `Util::bump().alloc(value)`.
    #[allow(clippy::mut_from_ref)] // This is sound—it just redirects to `bumpalo`.
    pub fn alloc<T>(&self, value: T) -> &mut T {
        self.bump().alloc(value)
    }

    /// Queues an item to be spawned.
    ///
    /// This redirects to `Spawner::spawn`.
    pub fn spawn_item(&self, position: Position, velocity: Vec3, item: ItemStack) {
        self.spawner.spawn_item(position, velocity, item);
    }

    /// This should be called at the end of every tick.
    pub fn reset(&mut self) {
        // Reset bump allocators
        for bump in self.bump.iter_mut() {
            bump.reset();
        }
    }
}
