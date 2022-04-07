//! Dynamic query infrastructure.

use std::{any::TypeId, borrow::Cow, cell::Cell, ops::Deref};

use crate::{
    storage::sparse_set, Component, Components, Entities, Ref, RefMut, SparseSetRef,
    SparseSetStorage,
};

/// Drives a query by yielding the entities
/// whose components satisfy the query parameters.
pub struct QueryDriver<'w, 'q> {
    /// A sparse set for each component in the query.
    sparse_sets: Cow<'q, [SparseSetRef<'w>]>,

    /// The "lead" sparse set, chosen as the set with
    /// the smallest number of components.
    lead: SparseSetRef<'w>,

    /// Used as the yielded value for the iterator.
    /// (We can't own this because of the lack of GATs.)
    dense_indices: Cow<'q, [Cell<u32>]>,
}

impl<'w, 'q> QueryDriver<'w, 'q> {
    /// Creates a new `QueryDriver` given the sparse sets
    /// whose components are required by the query.
    ///
    /// `dense_indices` should be a zeroed slice of size `sparse_sets.len()`.
    ///
    /// # Panics
    /// Panics if `sparse_sets.len() != dense_indices.len()`.
    pub fn new(
        sparse_sets: Cow<'q, [SparseSetRef<'w>]>,
        dense_indices: Cow<'q, [Cell<u32>]>,
    ) -> Self {
        let lead = sparse_sets
            .iter()
            .min_by_key(|set| set.len())
            .copied()
            .unwrap_or(*SparseSetRef::empty());

        Self {
            sparse_sets,
            lead,
            dense_indices,
        }
    }

    /// Returns the number of components in the query.
    pub fn num_components(&self) -> usize {
        self.sparse_sets.len()
    }

    /// Iterates over dense and sparse indices matching the query.
    pub fn iter(&'q mut self) -> QueryDriverIter<'w, 'q> {
        QueryDriverIter {
            lead_iter: self.lead.iter(),
            driver: self,
        }
    }
}

/// An iterator for a QueryDriver.
pub struct QueryDriverIter<'w, 'q> {
    driver: &'q mut QueryDriver<'w, 'q>,
    lead_iter: sparse_set::Iter<'q>,
}

impl<'w, 'q> QueryDriverIter<'w, 'q> {
    #[allow(clippy::never_loop)] // looks like a false positive - the loop has a `continue`
    pub fn next(&mut self) -> Option<QueryItem> {
        loop {
            let (sparse_index, lead_dense_index) = self.lead_iter.next()?;

            for (dense_index, sparse_set) in self
                .driver
                .dense_indices
                .iter()
                .zip(self.driver.sparse_sets.iter())
            {
                dense_index.set(match sparse_set.dense_index_of(sparse_index) {
                    Some(d) => d,
                    None => continue,
                });
            }

            break Some(QueryItem {
                dense_indices: &self.driver.dense_indices,
                sparse_index,
            });
        }
    }
}

/// An item yielded by a query.
pub struct QueryItem<'q> {
    /// The `dense` index into the sparse set
    /// of each component in the query.
    pub dense_indices: &'q [Cell<u32>],
    /// The sparse (entity) index of this item.
    pub sparse_index: u32,
}

// -- Static queries

/// A typed query element.
pub trait QueryParameter<'a> {
    type Output: 'a;
    type Component: Component;

    unsafe fn get_unchecked_by_dense_index(
        storage: &'a SparseSetStorage,
        dense_index: u32,
    ) -> Self::Output;
}

impl<'a, T> QueryParameter<'a> for &'a T
where
    T: Component,
{
    type Output = Ref<'a, T>;
    type Component = T;

    unsafe fn get_unchecked_by_dense_index(
        storage: &'a SparseSetStorage,
        dense_index: u32,
    ) -> Self::Output {
        let (ptr, borrow_flag) = storage.get_unchecked_by_dense_index(dense_index);
        borrow_flag.borrow().expect("query causes borrow conflicts");
        Ref::new(&*ptr.as_ptr(), borrow_flag)
    }
}

impl<'a, T> QueryParameter<'a> for &'a mut T
where
    T: Component,
{
    type Output = RefMut<'a, T>;
    type Component = T;

    unsafe fn get_unchecked_by_dense_index(
        storage: &'a SparseSetStorage,
        dense_index: u32,
    ) -> Self::Output {
        let (ptr, borrow_flag) = storage.get_unchecked_by_dense_index(dense_index);
        borrow_flag
            .borrow_mut()
            .expect("query causes borrow conflicts");
        RefMut::new(&mut *ptr.as_ptr(), borrow_flag)
    }
}

/// A tuple of query parameters.
pub trait QueryTuple<'a> {
    type Output: 'a;

    // avoiding allocations here is blocked on const generics and/or GATs
    fn sparse_sets(components: &Components) -> Vec<&SparseSetStorage>;

    fn dense_indices() -> Vec<Cell<u32>>;

    /// # Safety
    /// `dense_indices` must be valid dense indices into the
    /// sparse set at the corresponding index.
    ///
    /// `dense_indices` and `sparse_sets` must have a length equal
    /// to the length of the vectors returned by the corresponding methods
    /// of this trait.
    unsafe fn make_output(
        sparse_sets: &[&'a SparseSetStorage],
        dense_indices: &[Cell<u32>],
    ) -> Self::Output;
}

macro_rules! query_tuple_impl {
    ($count:literal, $(($ty:ident, $index:literal)),* $(,)?) => {
        impl <'a, $($ty: QueryParameter<'a>),*> QueryTuple<'a> for ($($ty),*) {
            type Output = ($($ty::Output),*);

            fn sparse_sets(components: &Components) -> Vec<&SparseSetStorage> {
                vec![
                    $(
                        components.storage_for::<$ty::Component>().unwrap_or_else(|_| SparseSetStorage::empty()),
                    )*
                ]
            }

            fn dense_indices() -> Vec<Cell<u32>> {
                vec![Cell::new(0); $count]
            }

            unsafe fn make_output(
                sparse_sets: &[&'a SparseSetStorage],
                dense_indices: &[Cell<u32>],
            ) -> Self::Output {
                (
                    $(
                        $ty::get_unchecked_by_dense_index(sparse_sets.get_unchecked($index), dense_indices.get_unchecked($index).get())
                    ),*
                )
            }
        }
    }
}

query_tuple_impl!(1, (T0, 0));
query_tuple_impl!(2, (T0, 0), (T1, 1));
query_tuple_impl!(3, (T0, 0), (T1, 1), (T2, 2));
query_tuple_impl!(4, (T0, 0), (T1, 1), (T2, 2), (T3, 3));
query_tuple_impl!(5, (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4));
query_tuple_impl!(6, (T0, 0), (T1, 1), (T2, 2), (T3, 3), (T4, 4), (T5, 5));
query_tuple_impl!(
    7,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6)
);
query_tuple_impl!(
    8,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7)
);
query_tuple_impl!(
    9,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8)
);
query_tuple_impl!(
    10,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9)
);
query_tuple_impl!(
    11,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10)
);
query_tuple_impl!(
    12,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11)
);
query_tuple_impl!(
    13,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12)
);
query_tuple_impl!(
    14,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13)
);
query_tuple_impl!(
    15,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13),
    (T14, 14)
);
query_tuple_impl!(
    16,
    (T0, 0),
    (T1, 1),
    (T2, 2),
    (T3, 3),
    (T4, 4),
    (T5, 5),
    (T6, 6),
    (T7, 7),
    (T8, 8),
    (T9, 9),
    (T10, 10),
    (T11, 11),
    (T12, 12),
    (T13, 13),
    (T14, 14),
    (T15, 15)
);
