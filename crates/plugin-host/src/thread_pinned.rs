use std::{
    cell::{Ref, RefCell, RefMut},
    thread::{self, ThreadId},
};

/// Wraps a [`RefCell`] but implements `Send` and `Sync`.
///
/// The value can only be borrowed on the thread it was created
/// on; this is enforced at runtime. In other words, access to
/// the value is pinned to the main thread.
pub struct ThreadPinned<T> {
    cell: RefCell<T>,
    pinned_to: ThreadId,
}

impl<T> ThreadPinned<T> {
    pub fn new(value: T) -> Self {
        Self {
            cell: RefCell::new(value),
            pinned_to: thread::current().id(),
        }
    }

    #[allow(unused)]
    pub fn borrow(&self) -> Ref<T> {
        self.assert_thread();
        self.cell.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.assert_thread();
        self.cell.borrow_mut()
    }

    fn assert_thread(&self) {
        assert_eq!(
            thread::current().id(),
            self.pinned_to,
            "can only borrow value on the main thread"
        );
    }
}

unsafe impl<T> Send for ThreadPinned<T> {}
unsafe impl<T> Sync for ThreadPinned<T> {}
