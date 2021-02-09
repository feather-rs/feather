use std::{any::TypeId, marker::PhantomData};

use quill_common::{HostComponent, QueryData};

use crate::{Entity, EntityId};

pub trait Query {
    fn add_component_types(types: &mut Vec<TypeId>);

    unsafe fn get_unchecked(
        data: &QueryData,
        component_index: &mut usize,
        entity_index: usize,
    ) -> Self;
}

impl<'a, T> Query for &'a T
where
    T: 'static,
{
    fn add_component_types(types: &mut Vec<TypeId>) {
        types.push(TypeId::of::<T>());
    }

    unsafe fn get_unchecked(
        data: &QueryData,
        component_index: &mut usize,
        entity_index: usize,
    ) -> Self {
        let component_slice = *(data.component_ptrs as *const *mut u8).add(*component_index);
        let component = &*component_slice.cast::<T>().add(entity_index);
        *component_index += 1;
        component
    }
}

macro_rules! impl_query_tuple {
    ($($query:ident),* $(,)?) => {
        impl <$($query: Query),*> Query for ($($query,)*) {
            fn add_component_types(types: &mut Vec<TypeId>) {
                $(
                    $query::add_component_types(types);
                )*
            }

            unsafe fn get_unchecked(data: &QueryData, component_index: &mut usize, entity_index: usize) -> Self {
                (
                    $(
                        $query::get_unchecked(data, component_index, entity_index)
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

pub struct QueryIter<Q> {
    data: QueryData,
    index: usize,
    _marker: PhantomData<Q>,
}

impl<Q> QueryIter<Q>
where
    Q: Query,
{
    pub(crate) fn new() -> Self {
        let mut component_types = Vec::new();
        Q::add_component_types(&mut component_types);

        let component_types: Vec<HostComponent> = component_types
            .into_iter()
            .map(|ty| HostComponent::from_type_id(ty).expect("unknown component type"))
            .collect();

        let data = unsafe {
            *quill_sys::query_begin(component_types.as_ptr(), component_types.len() as u32)
        };

        Self {
            data,
            index: 0,
            _marker: PhantomData,
        }
    }
}

impl<Q> Iterator for QueryIter<Q>
where
    Q: Query,
{
    type Item = (Entity, Q);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.num_entities as usize {
            return None;
        }

        let components = unsafe {
            let mut component_index = 0;
            Q::get_unchecked(&self.data, &mut component_index, self.index)
        };
        let entity_id = unsafe { *(self.data.entities_ptr as *const EntityId).add(self.index) };
        let entity = Entity::new(entity_id);

        self.index += 1;

        Some((entity, components))
    }
}
