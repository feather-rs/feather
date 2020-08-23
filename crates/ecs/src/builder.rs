use crate::{BorrowFlag, Ecs};
use bumpalo::Bump;
use legion::{
    storage::{
        ArchetypeSource, Component, ComponentSource, ComponentTypeId, EntityLayout,
        IntoComponentSource,
    },
    Entity,
};

/// A builder for an entity.
///
/// To spawn an entity, you add the components for that entity
/// with calls to `with()`, then call `spawn_into(ecs)` to spawn
/// the entity with all those components.
#[derive(Default)]
pub struct EntityBuilder {
    /// Bump allocator into which we allocate component data.
    bump: Bump,
    /// The set of components in this entity builder.
    components: Vec<Meta>,
    /// The archetype layout for the resulting entity.
    layout: EntityLayout,
}

struct Meta {
    /// Pointer to the component data in the `EntityBuilder`'s bump allocator.
    ptr: *const u8,
    /// The component's type ID.
    type_id: ComponentTypeId,
}

impl EntityBuilder {
    /// Creates a new, empty `EntityBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a component to the entity that will be built.
    pub fn with<T: Component>(mut self, component: T) -> Self {
        self.add_internal(BorrowFlag::<T>::new());
        self.add_internal(component);
        self
    }

    fn add_internal<T: Component>(&mut self, component: T) {
        let ptr = self.bump.alloc(component) as *mut T as *mut u8;
        let meta = Meta {
            ptr,
            type_id: ComponentTypeId::of::<T>(),
        };
        self.components.push(meta);
        self.layout.register_component::<T>();
    }

    /// Spawns this entity into an `Ecs`. Adds all components
    /// that were added with calls to `with()`.
    ///
    /// Returns an `Entity` handle to the entity that was created.
    ///
    /// Equivalent to calling `Ecs::spawn` with this `EntityBuilder`.
    pub fn spawn_into(self, ecs: &mut Ecs) -> Entity {
        ecs.inner.extend(self)[0]
    }
}

impl IntoComponentSource for EntityBuilder {
    type Source = Self;

    fn into(self) -> Self::Source {
        self
    }
}

impl ArchetypeSource for EntityBuilder {
    type Filter = EntityLayout;

    fn filter(&self) -> Self::Filter {
        self.layout.clone()
    }

    fn layout(&mut self) -> legion::storage::EntityLayout {
        self.layout.clone()
    }
}

impl ComponentSource for EntityBuilder {
    fn push_components<'a>(
        &mut self,
        writer: &mut legion::storage::ArchetypeWriter<'a>,
        mut entities: impl Iterator<Item = Entity>,
    ) {
        // We're only adding a single entity.
        writer.push(entities.next().expect("expected at least one entity"));

        for component in &self.components {
            unsafe {
                writer
                    .claim_components_unknown(component.type_id)
                    .extend_memcopy_raw(component.ptr, 1)
            }
        }
    }
}
