use std::{any::TypeId, collections::VecDeque};

use ahash::AHashMap;
use hecs::{Component, DynamicBundle, Entity};

/// Tracks changes made to certain components.
#[derive(Default)]
pub struct ChangeTracker {
    registries: AHashMap<TypeId, ChangeRegistry>,
}

impl ChangeTracker {
    pub fn track_component<T: Component>(&mut self) {
        self.registries
            .insert(TypeId::of::<T>(), ChangeRegistry::default());
    }

    pub fn on_insert(&mut self, entity: Entity, components: &impl DynamicBundle) {
        components.with_ids(|typs| {
            for ty in typs {
                let registry = self.registries.get_mut(ty);
                if let Some(registry) = registry {
                    registry.mark_changed(entity);
                }
            }
        });
    }

    pub fn iter_changed<T: Component>(&self) -> impl Iterator<Item = Entity> + '_ {
        self.registries.get(&TypeId::of::<T>())
        .unwrap_or_else(|| panic!("Components of type {} are not tracked for changes. Call `Ecs::track_component` to enable change tracking.", std::any::type_name::<T>()))
        .changed_entities
        .iter()
        .copied()
    }
}

#[derive(Default)]
struct ChangeRegistry {
    changed_entities: VecDeque<Entity>,
}

impl ChangeRegistry {
    pub fn mark_changed(&mut self, entity: Entity) {
        if !self.changed_entities.contains(&entity) {
            self.changed_entities.push_back(entity);
        }
    }

    #[allow(unused)]
    pub fn pop(&mut self) {
        self.changed_entities.pop_front();
    }
}
