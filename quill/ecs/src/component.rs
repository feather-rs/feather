use std::{alloc::Layout, any::TypeId, ptr, sync::Arc};

/// A type that can be used as a component.
///
/// Components must implement this trait.
pub trait Component: Send + 'static {}

impl<T> Component for T where T: Send + 'static {}

/// Metadata for a component type.
#[derive(Clone)]
pub struct ComponentMeta {
    /// Component type ID.
    pub(crate) type_id: TypeId,
    /// Component layout.
    pub(crate) layout: Layout,
    /// Function to drop the component.
    pub(crate) drop_fn: unsafe fn(*mut u8),
}

impl ComponentMeta {
    /// Creates a `ComponentMeta` for a native Rust component.
    pub fn of<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            drop_fn: |ptr| unsafe { ptr::drop_in_place(ptr.cast::<T>()) },
        }
    }
}
