use ahash::AHashMap;
use feather_core::anvil::{
    block_entity::{BlockEntityData, BlockEntityVariant},
    entity::{EntityData, EntityDataKind},
};
use feather_server_types::{
    BlockEntityLoaderFn, BlockEntityLoaderRegistration, EntityLoaderFn, EntityLoaderRegistration,
};
use fecs::EntityBuilder;

/// Stores state for loading entities.
pub struct EntityLoader {
    /// Map from `EntityDataKind` to functions
    /// to load entities of those kinds.
    loaders: AHashMap<EntityDataKind, &'static dyn EntityLoaderFn>,
    block_loaders: AHashMap<BlockEntityVariant, &'static dyn BlockEntityLoaderFn>,
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
        let block_loaders = inventory::iter::<BlockEntityLoaderRegistration>
            .into_iter()
            .map(|registration| (registration.kind, registration.f))
            .collect();
        Self {
            loaders,
            block_loaders,
        }
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

    /// Converts a `BlockEntityData` into an `EntityBuilder`
    /// ready for spawning in a `World`.
    pub fn load_block(&self, data: BlockEntityData) -> Option<anyhow::Result<EntityBuilder>> {
        self.block_loaders
            .get(&data.kind.variant())
            .map(|loader| loader(data))
    }
}
