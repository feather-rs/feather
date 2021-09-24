use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use commands::dispatcher::CommandDispatcher;

use quill_common::PointerMut;

use crate::command::CommandContext;
use crate::Game;

/// Struct passed to your plugin's `enable()` function.
///
/// Allows you to register systems, etc.
pub struct Setup<Plugin: crate::Plugin> {
    _marker: PhantomData<Plugin>,
}

impl<Plugin: crate::Plugin> Setup<Plugin> {
    /// For Quill internal use only. Do not call.
    #[doc(hidden)]
    #[allow(clippy::new_without_default)]
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
            quill_sys::register_system(system_data.into(), name.as_ptr().into(), name.len() as u32);
        }

        self
    }

    /// Perform various actions on command dispatcher: Register commands, tab-completions, etc.
    pub fn with_dispatcher(
        &mut self,
        f: impl FnOnce(&mut CommandDispatcher<CommandContext<Plugin>>),
    ) -> &mut Self {
        let mut dispatcher = CommandDispatcher::new();

        f(&mut dispatcher);

        let (nodes, executors, tab_completers) = dispatcher.to_parts();
        let nodes: Vec<_> = nodes.into_iter().map(|(_, a)| a).collect();
        let executors: Vec<_> = executors.into_iter().map(|(_, a)| a).collect();
        let tab_completers: Vec<_> = tab_completers.into_iter().collect();

        let (nodes, executors, tab_completers) = (
            ManuallyDrop::new(nodes),
            ManuallyDrop::new(executors),
            ManuallyDrop::new(tab_completers),
        );
        // SAFETY: passing raw vec data to recreate the vec on host
        unsafe {
            quill_sys::modify_command_executor(
                PointerMut::new(nodes.as_ptr() as *const _ as *mut _),
                nodes.len() as u32,
                nodes.capacity() as u32,
                PointerMut::new(executors.as_ptr() as *const _ as *mut _),
                executors.len() as u32,
                executors.capacity() as u32,
                PointerMut::new(tab_completers.as_ptr() as *const _ as *mut _),
                tab_completers.len() as u32,
                tab_completers.capacity() as u32,
            );
        }
        self
    }
}
