use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::PointerMut;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Zeroable, Pod, Serialize, Deserialize)]
#[repr(transparent)]
pub struct EntityId(pub u64);

/// Returned by `query_begin`. Contains pointers
/// to the data yielded by the query.
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
#[repr(C)]
pub struct QueryData {
    /// The number of (entity, component_1, ..., component_n) pairs
    /// yielded by this query.
    pub num_entities: u64,
    /// Pointer to an array of `num_entities` entities.
    pub entities_ptr: PointerMut<EntityId>,
    /// Pointer to an array of component pointers, one for
    /// each component in the call to `query_begin`. Each component
    /// pointer points to `num_entities` components of the corresponding type,
    /// serialized using the component's `to_bytes` method.
    ///
    /// Note that each component pointer is 64 bits regardless of target.
    pub component_ptrs: PointerMut<PointerMut<u8>>,
    /// Pointer to an array of `u32`s, one for each component.
    /// Each `u32` is the number of bytes in the corresponding
    /// component buffer.
    pub component_lens: PointerMut<u32>,
}
