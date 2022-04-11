use std::any::{type_name, TypeId};

use ahash::AHashMap;

use crate::{
    component::ComponentMeta, storage::SparseSetStorage, Component, ComponentError, Ref, RefMut,
};

/// A raw ECS that stores only components but does not track
/// entities.
///
/// Operates on entity indices but does not account for generations.
#[derive(Default)]
pub struct Components {
    storages: AHashMap<TypeId, SparseSetStorage>,
}

impl Components {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: Component>(&mut self, index: u32, component: T) {
        self.storage_or_insert_for::<T>().insert(index, component);
    }

    /// # Safety
    /// `component` must point to a valid instance of the component.
    pub unsafe fn insert_raw(
        &mut self,
        index: u32,
        component_meta: ComponentMeta,
        component: *const u8,
    ) -> *mut u8 {
        self.storage_or_insert_for_untyped(component_meta)
            .insert_raw(index, component)
    }

    pub fn remove<T: Component>(&mut self, index: u32) -> Result<(), ComponentError> {
        let was_removed = self.storage_mut_for::<T>()?.remove(index);
        if was_removed {
            Ok(())
        } else {
            Err(ComponentError::MissingComponent(type_name::<T>()))
        }
    }

    pub fn get<T: Component>(&self, index: u32) -> Result<Ref<T>, ComponentError> {
        self.storage_for::<T>()?
            .get(index)?
            .ok_or_else(|| ComponentError::MissingComponent(type_name::<T>()))
    }

    pub fn get_mut<T: Component>(&self, index: u32) -> Result<RefMut<T>, ComponentError> {
        self.storage_for::<T>()?
            .get_mut(index)?
            .ok_or_else(|| ComponentError::MissingComponent(type_name::<T>()))
    }

    pub fn storages(&self) -> impl Iterator<Item = (TypeId, &SparseSetStorage)> + '_ {
        self.storages
            .iter()
            .map(|(&type_id, storage)| (type_id, storage))
    }

    pub fn storages_mut(&mut self) -> impl Iterator<Item = (TypeId, &mut SparseSetStorage)> + '_ {
        self.storages
            .iter_mut()
            .map(|(&type_id, storage)| (type_id, storage))
    }

    pub fn storage_for<T: Component>(&self) -> Result<&SparseSetStorage, ComponentError> {
        self.storages
            .get(&TypeId::of::<T>())
            .ok_or_else(|| ComponentError::MissingComponent(type_name::<T>()))
    }

    fn storage_mut_for<T: Component>(&mut self) -> Result<&mut SparseSetStorage, ComponentError> {
        self.storages
            .get_mut(&TypeId::of::<T>())
            .ok_or_else(|| ComponentError::MissingComponent(type_name::<T>()))
    }

    fn storage_or_insert_for<T: Component>(&mut self) -> &mut SparseSetStorage {
        self.storages
            .entry(TypeId::of::<T>())
            .or_insert_with(|| SparseSetStorage::new(ComponentMeta::of::<T>()))
    }

    fn storage_or_insert_for_untyped(
        &mut self,
        component_meta: ComponentMeta,
    ) -> &mut SparseSetStorage {
        self.storages
            .entry(component_meta.type_id)
            .or_insert_with(|| SparseSetStorage::new(component_meta))
    }
}
