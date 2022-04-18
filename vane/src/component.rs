use std::{alloc::Layout, any::TypeId, ptr, sync::Arc};

use crate::{Entities, Entity};

/// A type that can be used as a component.
///
/// Components must implement this trait.
pub trait Component: 'static {
    /// Called when the component is inserted into the ECS.
    ///
    /// This method can be used to implement custom change detection
    /// by obtaining a `Bus`.
    ///
    /// The default implementation does nothing.
    fn on_inserted(&mut self, ecs: &Entities, owner: Entity) {
        let _ = (ecs, owner);
    }
}

// Special impl
impl Component for () {}

/// Metadata for a component type.
#[derive(Clone)]
pub struct ComponentMeta {
    /// Component type ID.
    pub(crate) type_id: TypeId,
    /// Component layout.
    pub(crate) layout: Layout,
    /// Function to drop the component.
    pub(crate) drop_fn: unsafe fn(*mut u8),
    pub(crate) on_inserted_fn: unsafe fn(*mut u8, &Entities, Entity),
    pub(crate) name: &'static str,
}

impl ComponentMeta {
    /// Creates a `ComponentMeta` for a native Rust component.
    pub fn of<T: Component>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            drop_fn: |ptr| unsafe { ptr::drop_in_place(ptr.cast::<T>()) },
            on_inserted_fn: |ptr, entities, owner| unsafe {
                let val = &mut *(ptr.cast::<T>());
                val.on_inserted(entities, owner)
            },
            name: std::any::type_name::<T>(),
        }
    }
}
