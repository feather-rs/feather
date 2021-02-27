//! Query for all entities with a certain set of components.

use std::{marker::PhantomData, mem::MaybeUninit};

use quill_common::{entity::QueryData, Component, HostComponent, PointerMut};

use crate::{Entity, EntityId};

/// A type that can be used for a query.
///
/// Implemented for tuples of `Query`s as well.
pub trait Query {
    type Item;

    fn add_component_types(types: &mut Vec<HostComponent>);

    unsafe fn get_unchecked(
        data: &QueryData,
        component_index: &mut usize,
        component_offsets: &mut [usize],
    ) -> Self::Item;
}

impl<'a, T> Query for &'a T
where
    T: Component,
    [T]: ToOwned,
{
    type Item = T;

    fn add_component_types(types: &mut Vec<HostComponent>) {
        types.push(T::host_component());
    }

    unsafe fn get_unchecked(
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

macro_rules! impl_query_tuple {
    ($($query:ident),* $(,)?) => {
        impl <$($query: Query),*> Query for ($($query,)*) {
            type Item = ($($query::Item),*);
            fn add_component_types(types: &mut Vec<HostComponent>) {
                $(
                    $query::add_component_types(types);
                )*
            }

            unsafe fn get_unchecked(data: &QueryData, component_index: &mut usize, component_offsets: &mut [usize]) -> Self::Item {
                (
                    $(
                        $query::get_unchecked(data, component_index, component_offsets)
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

/// An iterator over all entities matching a query.
pub struct QueryIter<Q> {
    data: QueryData,
    entity_index: usize,
    component_offsets: Vec<usize>,
    _marker: PhantomData<Q>,
}

impl<Q> QueryIter<Q>
where
    Q: Query,
{
    pub(crate) fn new() -> Self {
        let mut component_types = Vec::new();
        Q::add_component_types(&mut component_types);

        let mut data = MaybeUninit::uninit();
        let data = unsafe {
            quill_sys::entity_query(
                component_types.as_ptr().into(),
                component_types.len() as u32,
                PointerMut::new(&mut data),
            );
            // SAFETY: `entity_query` initializes `query_data`.
            data.assume_init()
        };

        let component_offsets = vec![0; component_types.len()];

        Self {
            data,
            entity_index: 0,
            component_offsets,
            _marker: PhantomData,
        }
    }
}

impl<Q> Iterator for QueryIter<Q>
where
    Q: Query,
{
    type Item = (Entity, Q::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.entity_index >= self.data.num_entities as usize {
            return None;
        }

        let components = unsafe {
            let mut component_index = 0;
            Q::get_unchecked(
                &self.data,
                &mut component_index,
                &mut self.component_offsets,
            )
        };
        let entity_id = unsafe { *(self.data.entities_ptr.as_mut_ptr()).add(self.entity_index) };
        let entity = Entity::new(EntityId(entity_id));

        self.entity_index += 1;

        Some((entity, components))
    }
}
