use std::any::TypeId;

use bytemuck::{Pod, Zeroable};

#[doc(inline)]
pub use libcraft_core::{BlockPosition, ChunkPosition, Position};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Zeroable, Pod)]
#[repr(transparent)]
pub struct EntityId(pub u64);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct QueryData {
    /// The number of (entity, component_1, ..., component_n) pairs
    /// yielded by this query.
    pub num_entities: u32,
    /// Pointer to an array of `num_entities` entities.
    pub entities_ptr: u32,
    /// Pointer to an array of component pointers, one for
    /// each component in the call to `query_begin`. Each component
    /// pointer points to `num_entities` components of the corresponding type.
    pub component_ptrs: u32,
}

macro_rules! c_enum {
    ($name:ident {
        $(
            $variant:ident = $int:literal
        )* $(,)?
    }) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        #[repr(u32)]
        pub enum $name {
            $(
                $variant = $int,
            )*
        }

        impl $name {
            pub fn from_u32(x: u32) -> Option<Self> {
                match x {
                    $(
                        $int => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }
        }
    }
}

c_enum!(HostComponent {
    Position = 0,
});

impl HostComponent {
    pub fn from_type_id(type_id: TypeId) -> Option<Self> {
        match type_id {
            x if x == TypeId::of::<Position>() => Some(HostComponent::Position),
            _ => None,
        }
    }

    pub fn type_id(self) -> TypeId {
        match self {
            HostComponent::Position => TypeId::of::<Position>(),
        }
    }
}
