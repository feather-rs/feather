use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
};

/// Used to dynamically borrow-check component accesses.
///
/// Supports either one mutable reference or up to 254 shared references.
/// Exceeding either limit results in `BorrowError`.
#[derive(Default)]
pub(crate) struct BorrowFlag {
    flag: Cell<u8>,
}

const MUTABLE_SENTINEL: u8 = u8::MAX;

#[derive(Debug, thiserror::Error)]
#[error("borrow conflict or too many borrows (more than 127 Ref<T>s)")]
pub struct BorrowError;

impl BorrowFlag {
    pub fn borrow(&self) -> Result<(), BorrowError> {
        let flag = self.flag.get();

        // The checked arithmetic will fail if the current borrow count
        // is 254, which is the greatest possible number of shared borrows, or
        // if it's 255, which means the flag is mutably borrowed.
        let new_flag_plus_one = flag.checked_add(2).ok_or(BorrowError)?;

        self.flag.set(new_flag_plus_one - 1);
        Ok(())
    }

    pub fn borrow_mut(&self) -> Result<(), BorrowError> {
        let flag = self.flag.get();
        if flag != 0 {
            return Err(BorrowError);
        }

        self.flag.set(MUTABLE_SENTINEL);
        Ok(())
    }

    pub fn unborrow(&self) {
        let flag = self.flag.get();
        debug_assert!(flag > 0);
        self.flag.set(flag - 1);
    }

    pub fn unborrow_mut(&self) {
        debug_assert_eq!(self.flag.get(), MUTABLE_SENTINEL);
        self.flag.set(0);
    }
}

/// A reference to a component.
///
/// This is an RAII guard that dynamically tracks
/// borrow checking, akin to `std::cell::RefCell`.
pub struct Ref<'a, T> {
    component: &'a T,
    flag: &'a BorrowFlag,
}

impl<'a, T> Ref<'a, T> {
    pub(crate) fn new(component: &'a T, flag: &'a BorrowFlag) -> Self {
        Self { component, flag }
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component
    }
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        self.flag.unborrow();
    }
}

/// A mutable reference to a component.
///
/// This is an RAII guard that dynamically tracks
/// borrow checking, akin to `std::cell::RefCell`.
pub struct RefMut<'a, T> {
    component: &'a mut T,
    flag: &'a BorrowFlag,
}

impl<'a, T> RefMut<'a, T> {
    pub(crate) fn new(component: &'a mut T, flag: &'a BorrowFlag) -> Self {
        Self { component, flag }
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.component
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.component
    }
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        self.flag.unborrow_mut();
    }
}
