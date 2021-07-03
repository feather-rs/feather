use std::iter::once;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use crate::{Component, HostComponent, PointerMut};

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

/// A type that can be used for a query.
///
/// Implemented for tuples of `FromQuery`s as well.
///
/// Implemented for `&T` where `T: Component`.
///
/// Implemented for entity wrappers like `Player`.
pub trait FromQuery {
    type Item;

    /// Adds the requested component types to the query.
    fn add_component_types(types: &mut impl Extend<HostComponent>);

    /// # Safety
    /// `component_index` must be the index of the first compoonent
    /// added by `add_component_types`.
    ///
    /// `component_offsets` must contain the proper byte offset
    /// of the current component index.
    unsafe fn get_unchecked(
        entity: EntityId,
        data: &QueryData,
        component_index: &mut usize,
        component_offsets: &mut [usize],
    ) -> Self::Item;
}

macro_rules! impl_query_tuple {
    ($($query:ident),* $(,)?) => {
        impl <$($query: FromQuery),*> FromQuery for ($($query,)*) {
            type Item = ($($query::Item),*);
            fn add_component_types(types: &mut impl Extend<HostComponent>) {
                $(
                    $query::add_component_types(types);
                )*
            }

            unsafe fn get_unchecked(entity: EntityId, data: &QueryData, component_index: &mut usize, component_offsets: &mut [usize]) -> Self::Item {
                (
                    $(
                        $query::get_unchecked(entity, data, component_index, component_offsets)
                    ),*
                )
            }
        }
    }
}

impl_query_tuple!(A, B);
impl_query_tuple!(A, B, C);
impl_query_tuple!(A, B, C, D);
impl_query_tuple!(A, B, C, D, E);
impl_query_tuple!(A, B, C, D, E, F);
impl_query_tuple!(A, B, C, D, E, F, G);
impl_query_tuple!(A, B, C, D, E, F, G, H);
impl_query_tuple!(A, B, C, D, E, F, G, H, I);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

impl<'a, T> FromQuery for &'a T
where
    T: Component,
{
    type Item = T;

    fn add_component_types(types: &mut impl Extend<HostComponent>) {
        types.extend(once(T::host_component()));
    }

    unsafe fn get_unchecked(
        _entity: EntityId,
        data: &QueryData,
        component_index: &mut usize,
        component_offsets: &mut [usize],
    ) -> T {
        let component_len = *((data.component_lens.as_mut_ptr()).add(*component_index)) as usize;
        let component_ptr =
            (*(data.component_ptrs.as_mut_ptr().add(*component_index))).as_mut_ptr();

        let offset = component_offsets[*component_index];
        let component_ptr = component_ptr.add(offset);
        let component_len = component_len - offset;

        let component_bytes = std::slice::from_raw_parts(component_ptr, component_len);
        let (value, advance) = T::from_bytes_unchecked(component_bytes);

        component_offsets[*component_index] += advance;

        *component_index += 1;

        value
    }
}
