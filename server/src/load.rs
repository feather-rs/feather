//! Implements the loading of entities.

use ahash::AHashMap;
use feather_core::entity::{EntityData, EntityDataKind};
use fecs::EntityBuilder;

inventory::collect!(EntityLoaderRegistration);

/// Stores state for loading entities.
pub struct EntityLoader {
    /// Map from `EntityDataKind` to functions
    /// to load entities of those kinds.
    loaders: AHashMap<EntityDataKind, &'static dyn EntityLoaderFn>,
}

impl Default for EntityLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityLoader {
    /// Initializes a new entity loader state. This function allocates.
    pub fn new() -> Self {
        let loaders = inventory::iter::<EntityLoaderRegistration>
            .into_iter()
            .map(|registration| (registration.kind, registration.f))
            .collect();
        Self { loaders }
    }
}

impl EntityLoader {
    /// Converts an `EntityData` into an `EntityBuilder`
    /// ready for spawning in a `World`.
    pub fn load(&self, data: EntityData) -> Option<anyhow::Result<EntityBuilder>> {
        self.loaders
            .get(&EntityDataKind::from(&data))
            .map(|loader| loader(data))
    }
}
