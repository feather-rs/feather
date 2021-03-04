use std::{
    any::{type_name, Any, TypeId},
    cell::{Ref, RefCell, RefMut},
};

use ahash::AHashMap;

#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("resource of type '{0}' does not exist")]
    Missing(&'static str),
    #[error(
        "resource of type '{0}' borrowed invalidly (mutably and immutable borrow at the same time)"
    )]
    Borrow(&'static str),
}

/// Structure storing _resources_, where each
/// resource is identified by its Rust type. At most
/// one resource of each type can exist.
///
/// Resources are borrow-checked at runtime using `RefCell`.
#[derive(Default)]
pub struct Resources {
    resources: AHashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a new resource into this container.
    ///
    /// Returns the old resource of the same type, if it existed.
    pub fn insert<T: 'static>(&mut self, resource: T) -> Option<T> {
        self.resources
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(resource)))
            .map(|resource| *resource.into_inner().downcast::<T>().unwrap())
    }

    /// Removes a resource from the container, returning it.
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.resources
            .remove(&TypeId::of::<T>())
            .map(|resource| *resource.into_inner().downcast::<T>().unwrap())
    }

    /// Gets the resource of type `T`.
    pub fn get<T: 'static>(&self) -> Result<Ref<T>, ResourceError> {
        let resource = self
            .resources
            .get(&TypeId::of::<T>())
            .ok_or_else(|| ResourceError::Missing(type_name::<T>()))?;

        resource
            .try_borrow()
            .map_err(|_| ResourceError::Borrow(type_name::<T>()))
            .map(|b| Ref::map(b, |b| b.downcast_ref::<T>().unwrap()))
    }

    /// Mutably gets the resource of type `T`.
    pub fn get_mut<T: 'static>(&self) -> Result<RefMut<T>, ResourceError> {
        let resource = self
            .resources
            .get(&TypeId::of::<T>())
            .ok_or_else(|| ResourceError::Missing(type_name::<T>()))?;

        resource
            .try_borrow_mut()
            .map_err(|_| ResourceError::Borrow(type_name::<T>()))
            .map(|b| RefMut::map(b, |b| b.downcast_mut::<T>().unwrap()))
    }
}
