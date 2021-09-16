//! Query for all entities with a certain set of components.

use std::{marker::PhantomData, mem::MaybeUninit};

use quill_common::{
    entity::{FromQuery, QueryData},
    PointerMut,
};

use crate::{Entity, EntityId};

/// An iterator over all entities matching a query.
pub struct QueryIter<Q> {
    data: QueryData,
    entity_index: usize,
    component_offsets: Vec<usize>,
    _marker: PhantomData<Q>,
}

impl<Q> QueryIter<Q>
where
    Q: FromQuery,
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
    Q: FromQuery,
{
    type Item = (Entity, Q::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.entity_index >= self.data.num_entities as usize {
            return None;
        }

        let entity_id =
            EntityId(unsafe { *(self.data.entities_ptr.as_mut_ptr()).add(self.entity_index) });
        let entity = Entity::new(entity_id);

        let components = unsafe {
            let mut component_index = 0;
            Q::get_unchecked(
                entity_id.0,
                &self.data,
                &mut component_index,
                &mut self.component_offsets,
            )
        };

        self.entity_index += 1;

        Some((entity, components))
    }
}
