use std::alloc::Layout;

/// A memory space used to store components.
///
/// The default memory space just allocates memory on the host.
pub struct MemorySpace {
    alloc: Box<dyn Fn(Layout) -> *mut u8 + Send + Sync>,
    dealloc: Box<dyn Fn(*mut u8, Layout) + Send + Sync>,
}

impl MemorySpace {
    pub fn host() -> Self {
        unsafe {
            Self::new(
                |layout| std::alloc::alloc(layout),
                |ptr, layout| std::alloc::dealloc(ptr, layout),
            )
        }
    }

    pub unsafe fn new(
        alloc: impl Fn(Layout) -> *mut u8 + Send + Sync + 'static,
        dealloc: impl Fn(*mut u8, Layout) + Send + Sync + 'static,
    ) -> Self {
        Self {
            alloc: Box::new(alloc),
            dealloc: Box::new(dealloc),
        }
    }

    pub(crate) unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (self.alloc)(layout)
    }

    pub(crate) unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        (self.dealloc)(ptr, layout)
    }
}
