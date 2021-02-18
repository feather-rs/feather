use std::num::NonZeroU64;

use bytemuck::{Pod, Zeroable};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(transparent)]
pub struct EntityId(pub u64);

/// Returned by `query_begin`. Contains pointers
/// to the data yielded by the query.
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
#[repr(C)]
pub struct QueryData {
    /// The number of (entity, component_1, ..., component_n) pairs
    /// yielded by this query.
    pub num_entities: u32,
    /// Pointer to an array of `num_entities` entities.
    pub entities_ptr: u32,
    /// Pointer to an array of component pointers, one for
    /// each component in the call to `query_begin`. Each component
    /// pointer points to `num_entities` components of the corresponding type,
    /// serialized using the component's `to_bytes` method.
    pub component_ptrs: u32,
    /// Pointer to an array of `u32`s, one for each component.
    /// Each `u32` is the number of bytes in the corresponding
    /// component buffer.
    pub component_lens: u32,
}

/// Returned by `entity_get_component`.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct GetComponentResult(NonZeroU64);

impl GetComponentResult {
    pub fn new(ptr: u32, len: u32) -> Self {
        Self(NonZeroU64::new(((ptr as u64) << 32) | len as u64).expect("null pointer or length"))
    }

    pub fn to_bits(self) -> u64 {
        self.0.get()
    }

    pub fn ptr(self) -> u32 {
        (self.0.get() >> 32) as u32
    }

    pub fn len(self) -> u32 {
        self.0.get() as u32
    }
}
