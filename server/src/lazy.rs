//! Extension methods for `LazyUpdate`.

use crate::entity::EntitySpawnEvent;
use shrev::EventChannel;
use specs::world::{EntitiesRes, LazyBuilder};
use specs::LazyUpdate;

pub trait LazyUpdateExt {
    /// Creates an entity and lazily inserts components.
    ///
    /// This should be used instead of `LazyUpdate::create_entity`
    /// because it automatically triggers an `EntitySpawnEvent`.
    fn spawn_entity(&self, entities: &EntitiesRes) -> LazyBuilder;
}

impl LazyUpdateExt for LazyUpdate {
    fn spawn_entity(&self, entities: &EntitiesRes) -> LazyBuilder {
        let entity = entities.create();
        // Trigger event
        self.exec(move |world| {
            world
                .fetch_mut::<EventChannel<EntitySpawnEvent>>()
                .single_write(EntitySpawnEvent { entity });
        });

        LazyBuilder { lazy: self, entity }
    }
}
