use std::{alloc::Layout, any::TypeId, sync::Arc};

use crate::space::MemorySpace;

/// A type that can be used as a component.
///
/// Components must implement this trait.
pub trait Component: Send + 'static {}

impl<T> Component for T where T: Send + 'static {}

/// Type ID of a component.
///
/// Supports both Rust types and arbitrary
/// "opaque" types identified by a `u64`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentTypeId(ComponentTypeIdInner);

impl ComponentTypeId {
    pub fn of<T: 'static>() -> Self {
        Self(ComponentTypeIdInner::Rust(TypeId::of::<T>()))
    }

    pub fn opaque(id: u64) -> Self {
        Self(ComponentTypeIdInner::Opaque(id))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ComponentTypeIdInner {
    Rust(TypeId),
    Opaque(u64),
}

/// Metadata for a component type.
#[derive(Clone)]
pub struct ComponentMeta {
    /// Memory space where the component is allocated.
    pub(crate) space: Arc<MemorySpace>,
    /// Component type ID.
    pub(crate) type_id: ComponentTypeId,
    /// Component layout.
    pub(crate) layout: Layout,
    /// Function to drop the component.
    pub(crate) drop_fn: Arc<dyn Fn(*mut u8) + Send + Sync>,
}

impl ComponentMeta {
    /// Creates a `ComponentMeta` for a native Rust component.
    pub fn of<T: 'static>() -> Self {
        Self {
            space: Arc::new(MemorySpace::host()),
            type_id: ComponentTypeId::of::<T>(),
            layout: Layout::new::<T>(),
            drop_fn: Arc::new(|data| unsafe { std::ptr::drop_in_place(data.cast::<T>()) }),
        }
    }

    /// Creates a `ComponentMeta` for an arbitrary type, maybe opaque,
    /// maybe allocated in a non-default memory space.
    pub fn custom(
        space: Arc<MemorySpace>,
        type_id: ComponentTypeId,
        layout: Layout,
        drop_fn: Arc<dyn Fn(*mut u8) + Send + Sync>,
    ) -> Self {
        Self {
            space,
            type_id,
            layout,
            drop_fn,
        }
    }
}
