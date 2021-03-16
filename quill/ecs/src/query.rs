//! Dynamic query infrastructure.

use std::{any::TypeId, cell::Cell, ops::Deref};

use crate::{storage::sparse_set, Component, Components, Ecs, SparseSetRef, SparseSetStorage};

/// Drives a query by yielding the entities
/// whose components satisfy the query parameters.
pub struct QueryDriver<'w, 'q> {
    /// A sparse set for each component in the query.
    sparse_sets: &'q [SparseSetRef<'w>],

    /// The "lead" sparse set, chosen as the set with
    /// the smallest number of components.
    lead: SparseSetRef<'q>,

    /// Used as the yielded value for the iterator.
    /// (We can't own this because of the lack of GATs.)
    dense_indices: &'q [Cell<u32>],
}

impl<'w, 'q> QueryDriver<'w, 'q> {
    /// Creates a new `QueryDriver` given the sparse sets
    /// whose components are required by the query.
    ///
    /// `dense_indices` should be a zeroed slice of size `sparse_sets.len()`.
    ///
    /// # Panics
    /// Panics if `sparse_sets.len() != dense_indices.len()`.
    pub fn new(sparse_sets: &'q [SparseSetRef<'w>], dense_indices: &'q [Cell<u32>]) -> Self {
        let lead = sparse_sets
            .iter()
            .min_by_key(|set| set.len())
            .unwrap_or(SparseSetRef::empty());

        Self {
            sparse_sets,
            lead: *lead,
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

impl<'w, 'q> Iterator for QueryDriverIter<'w, 'q> {
    type Item = QueryItem<'q>;

    fn next(&mut self) -> Option<Self::Item> {
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

/*
/// A typed query element.
pub trait QueryParameter {
    type Output;
    type Component: Component;

    unsafe fn get_unchecked_by_dense_index(
        storage: &SparseSetStorage,
        dense_index: u32,
    ) -> Self::Output;
}

impl<'a, T> QueryParameter for &'a T
where
    T: Component,
{
    type Output<'b> = &'b T;
    type Component = T;

    unsafe fn get_unchecked_by_dense_index(
        storage: &SparseSetStorage,
        dense_index: u32,
    ) -> Self::Output<'_> {
        storage.get_unchecked_by_dense_index(dense_index)
    }
}

/// A tuple of query parameters.
pub trait QueryTuple {
    type Output<'s>;

    // avoiding allocations here is blocked on const generics and/or GATs
    fn sparse_sets(components: &Components) -> Option<Vec<&SparseSetStorage>>;

    fn dense_indices() -> Vec<Cell<u32>>;

    /// # Safety
    /// `dense_indices` must be valid dense indices into the
    /// sparse set at the corresponding index.
    ///
    /// `dense_indices` and `sparse_sets` must have a length equal
    /// to the length of the vectors returned by the corresponding methods
    /// of this trait.
    unsafe fn make_output<'s>(
        sparse_sets: &'s [&'s SparseSetStorage],
        dense_indices: &[Cell<u32>],
    ) -> Self::Output<'s>;
}

macro_rules! query_tuple_impl {
    ($count:literal, $(($ty:ident, $index:literal)),* $(,)?) => {
        impl <$($ty: QueryParameter),*> QueryTuple for ($($ty),*) {
            type Output<'s> = ($($ty::Output<'s>),*);

            fn sparse_sets(components: &Components) -> Option<Vec<&SparseSetStorage>> {
                Some(vec![
                    $(
                        components.storage_for::<$ty::Component>().ok()?,
                    )*
                ])
            }

            fn dense_indices() -> Vec<Cell<u32>> {
                vec![Cell::new(0); $count]
            }

            unsafe fn make_output<'s>(
                sparse_sets: &'s [&'s SparseSetStorage],
                dense_indices: &[Cell<u32>],
            ) -> Self::Output<'s> {
                (
                    $(
                        $ty::get_unchecked_by_dense_index(sparse_sets.get_unchecked($index), dense_indices.get_unchecked($index).get())
                    ),*
                )
            }
        }
    }
}

query_tuple_impl!(1, (T1, 0));
*/
