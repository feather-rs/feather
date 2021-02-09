use std::marker::PhantomData;

use crate::Game;

/// Struct passed to your plugin's `enable()` function.
///
/// Allows you to register systems, etc.
pub struct Setup<Plugin> {
    _marker: PhantomData<Plugin>,
}

impl<Plugin> Setup<Plugin> {
    /// For Quill internal use only. Do not call.
    #[doc(hidden)]
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Registers a function as system to be invoked
    /// every tick.
    ///
    /// The function should take as parameters your
    /// plugin instance and an `&mut Game` and return nothing.
    pub fn add_system<T: FnMut(&mut Plugin, &mut Game)>(&mut self, system: T) -> &mut Self {
        let system: Box<dyn FnMut(&mut Plugin, &mut Game)> = Box::new(system);
        let system_data = Box::leak(Box::new(system)) as *mut Box<_> as *mut u8;

        let name = std::any::type_name::<T>();

        unsafe {
            quill_sys::register_system(system_data, name.as_ptr(), name.len() as u32);
        }

        self
    }
}
