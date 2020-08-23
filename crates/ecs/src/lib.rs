//! An ECS library tailored to Feather's use cases.
//!
//! This is implemented as a wrapper around Legion ECS, but we've made some interface
//! changes:
//! * Systems are just functions executed sequentially. This simplifies the mental
//! model of how systems are executed. (By contrast, Legion includes a parallel system
//! executor which may provide performance improvements at the cost of worse ergonomics.
//! We have no performance measurements that demonstrate this tradeoff is worthwhile.)
//! * Queries support real, fine-grained change detection, as opposed to Legion's "maybe changed" filter
//! which doesn't suit many of our needs. This again comes at a performance cost, which we may
//! work on optimizing if profiling demonstrates change tracking becomes a bottleneck.
//! * Components are runtime borrow-checked at the per-entity level. This means you can get mutable references
//! to the positions of two different entities at once without unsafe code—something often required
//! in our codebase. (In comparison, Legion only offers static borrow checking, which is inflexible and often
//! difficult to work with.)
//! * `World` is renamed to `Ecs`, since the term "world" conflicts with Minecraft's concept of worlds.

use legion::{storage::Component, world::EntryRef, EntityStore};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicU16, Ordering},
};
use thiserror::Error;

pub use legion::Entity;

mod builder;
pub use builder::EntityBuilder;

/// An error returned when an `Entity` handle is invalid
/// (because the entity was despawned).
#[derive(Debug, Error)]
#[error("entity {0:?} was despawned, so its components are no longer available")]
pub struct InvalidEntity(Entity);

/// An error returned when a component cannnot be accessed.
#[derive(Debug, Error)]
pub enum ComponentError {
    #[error(transparent)]
    InvalidEntity(InvalidEntity),
    #[error("this entity does not have a component of type `{0}`")]
    ComponentNotFound(&'static str),
    #[error("component of type `{0}` is already mutably borrowed")]
    MutablyBorrowed(&'static str),
    #[error("component of type `{0}` is already borrowed mutably or immutably")]
    Borrowed(&'static str),
}

/// A container for entities and their components.
///
/// Akin to the `World` type found in most ECS libraries (`legion`, `specs`, etc.)
/// or `entt::registry` in EnTT.
///
/// # Implementation
/// This is a wrapper around `legion::World` with some interface changes.
#[derive(Default)]
pub struct Ecs {
    inner: legion::World,
}

impl Ecs {
    /// Creates a new `Ecs` with no entities.
    pub fn new() -> Self {
        Self {
            inner: legion::World::default(),
        }
    }

    /// Creates an entity with a set of components. Returns
    /// its `Entity` handle, which can be used to access
    /// its components in the future.
    ///
    /// This is equivalent to calling `EntityBuilder::spawn_into()`.
    pub fn spawn(&mut self, components: EntityBuilder) -> Entity {
        components.spawn_into(self)
    }

    /// Deletes an entity, dropping all its components and invalidating
    /// its `Entity` handle.
    pub fn despawn(&mut self, entity: Entity) -> Result<(), InvalidEntity> {
        if self.inner.remove(entity) {
            Ok(())
        } else {
            Err(InvalidEntity(entity))
        }
    }
}

/// Component added to an entity for each user-facing
/// component is has. Used for fine-grained, per-entity
/// borrow checking of components.
#[derive(Default)]
struct BorrowFlag<T> {
    // The most significant bit stores whether the component
    // is mutably borrowed. The remaining 15 bits store
    // the number of shared references to the component.
    //
    // This allows for a maximum of 32767 simultaneous shared references to the same
    // component, which is reasonable enough.
    flag: AtomicU16,
    _marker: PhantomData<T>,
}

const MUTABLY_BORROWED_BIT: u16 = 1 << 15;

impl<T> BorrowFlag<T> {
    fn new() -> Self {
        Self {
            flag: AtomicU16::new(0),
            _marker: PhantomData,
        }
    }

    /// Attempts to mark the component as mutably borrowed. Returns
    /// whether whether this was successful.
    #[must_use = "indicates whether borrowing was successful"]
    fn mark_unique(&self) -> bool {
        // 0 means no shared or unique references are present, which is
        // the only valid state in which acquiring a unique reference is legal.
        self.flag
            .compare_exchange(0, MUTABLY_BORROWED_BIT, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    /// Attempts to increment the number of shared references to the component.
    ///
    /// Returns `false` if there already exists a unique reference.
    #[must_use = "indicates whether borrowing was successful"]
    fn mark_shared(&self) -> bool {
        // CAS loop
        loop {
            let current = self.flag.load(Ordering::Acquire);

            if current & MUTABLY_BORROWED_BIT != 0 {
                return false;
            }

            let new = current.checked_add(1).unwrap();
            if new & MUTABLY_BORROWED_BIT != 0 {
                // we overflowed the max number of shared references
                panic!(
                    "too many shared references to component of type `{}`",
                    std::any::type_name::<T>()
                );
            }

            match self
                .flag
                .compare_exchange(current, new, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => return true,
                Err(_) => continue, // flag changed by another thread: try again
            }
        }
    }

    /// Marks the flag as _not_ mutably borrowed anymore.
    ///
    /// # Safety
    /// Must only be called once after a call to `mark_unique()`.
    fn unmark_unique(&self) {
        let old = self.flag.swap(0, Ordering::Release);

        assert_eq!(
            old, MUTABLY_BORROWED_BIT,
            "called unmark_unique not paired to a call to mark_unique"
        );
    }

    /// Decrements the number of shared references to the component.
    ///
    /// # Safety
    /// Must only be called once after a call to `mark_shared()`.
    fn unmark_shared(&self) {
        self.flag.fetch_sub(1, Ordering::Release);
    }
}

/// A shared reference to a component for an entity.
pub struct ComponentRef<'a, T> {
    component: &'a T,
    flag: &'a BorrowFlag<T>,
}

impl<'a, T> Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component
    }
}

impl<'a, T> Drop for ComponentRef<'a, T> {
    fn drop(&mut self) {
        self.flag.unmark_shared();
    }
}

/// A unique reference to a component for an entity.
pub struct ComponentRefMut<'a, T> {
    component: &'a mut T,
    flag: &'a BorrowFlag<T>,
}

impl<'a, T> Deref for ComponentRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component
    }
}

impl<'a, T> DerefMut for ComponentRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.component
    }
}

impl<'a, T> Drop for ComponentRefMut<'a, T> {
    fn drop(&mut self) {
        self.flag.unmark_unique();
    }
}

impl Ecs {
    /// Gets the component of type `T` for an entity.
    ///
    /// Returns an error if any of the following apply:
    /// * The entity has been despawned.
    /// * The entity does not have a component of type `T`.
    /// * This component is already mutably borrowed elsewhere.
    pub fn get<T: Component>(&self, entity: Entity) -> Result<ComponentRef<T>, ComponentError> {
        let component = self
            .entry_ref(entity)?
            .into_component::<T>()
            .map_err(|_| ComponentError::ComponentNotFound(std::any::type_name::<T>()))?;
        let flag = self
            .entry_ref(entity)?
            .into_component::<BorrowFlag<T>>()
            .expect("missing borrow flag component");

        if !flag.mark_shared() {
            return Err(ComponentError::MutablyBorrowed(std::any::type_name::<T>()));
        }

        Ok(ComponentRef { component, flag })
    }

    /// Gets a mutable reference to the component of type `T` for an entity.
    ///
    /// Returns an error if any of the following apply:
    /// * The entity has been despawned.
    /// * The entity does not have a component of type `T`.
    /// * This component is already mutably or immutably borrowed elsewhere.
    pub fn get_mut<T: Component>(
        &self,
        entity: Entity,
    ) -> Result<ComponentRefMut<T>, ComponentError> {
        // To ensure we don't violate Rust aliasing rules, we have to get
        // the mutable reference to the component _after_ checking that the borrow
        // is valid.
        let flag = self
            .entry_ref(entity)?
            .into_component::<BorrowFlag<T>>()
            .map_err(|_| ComponentError::ComponentNotFound(std::any::type_name::<T>()))?;

        if !flag.mark_unique() {
            return Err(ComponentError::Borrowed(std::any::type_name::<T>()));
        }

        // Safety: the BorrowFlag tracks references to the component,
        // and we know by now that no references (mutable or immutable)
        // to this component exist.
        let component = unsafe { self.entry_ref(entity)?.into_component_unchecked::<T>() }
            .unwrap_or_else(|_| {
                panic!(
                    "BorrowFlag<{}> was present, but associated component {} was not",
                    std::any::type_name::<T>(),
                    std::any::type_name::<T>()
                )
            });

        Ok(ComponentRefMut { flag, component })
    }

    fn entry_ref(&self, entity: Entity) -> Result<EntryRef, ComponentError> {
        self.inner
            .entry_ref(entity)
            .ok_or_else(|| ComponentError::InvalidEntity(InvalidEntity(entity)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::Arc, thread};

    #[test]
    fn borrow_flag_single_threaded() {
        let flag = BorrowFlag::<()>::default();

        assert!(flag.mark_unique());
        assert!(!flag.mark_unique());
        assert!(!flag.mark_shared());
        flag.unmark_unique();

        for _ in 0..1000 {
            assert!(flag.mark_shared());
            assert!(!flag.mark_unique());
        }

        for _ in 0..1000 {
            assert!(!flag.mark_unique());
            flag.unmark_shared();
        }

        assert!(flag.mark_unique());
    }

    #[test]
    #[allow(clippy::same_item_push)] // lint is inaccurate and decreases code clarity
    fn borrow_flag_multi_threaded() {
        let flag = Arc::new(BorrowFlag::<()>::default());

        let mut handles = Vec::new();
        for _ in 0..8 {
            let flag = Arc::clone(&flag);
            handles.push(thread::spawn(move || {
                for _ in 0..1500 {
                    assert!(flag.mark_shared());
                    assert!(!flag.mark_unique());
                }

                for _ in 0..1500 {
                    flag.unmark_shared();
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert!(flag.mark_unique());
    }

    #[test]
    #[should_panic]
    fn borrow_flag_overflow() {
        let flag = BorrowFlag::<()>::default();

        for _ in 0..MUTABLY_BORROWED_BIT {
            let _ = flag.mark_shared();
        }
    }
}
