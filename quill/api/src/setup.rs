use std::{convert::TryInto, marker::PhantomData};

use lieutenant::command::command::Command;

use crate::{command::CommandContext, Game};

/// Struct passed to your plugin's `enable()` function.
///
/// Allows you to register systems, etc.
pub struct Setup<Plugin> {
    _marker: PhantomData<Plugin>,
}

impl<Plugin: 'static> Setup<Plugin> {
    /// For Quill internal use only. Do not call.
    #[doc(hidden)]
    #[allow(clippy::clippy::new_without_default)]
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

    /// Registers a command to be triggerd when
    /// some chat msg roughly looks like the regex.
    /// The server is very lazy when it comes to
    /// command dispatching, so dont expect it to have
    /// perfectly/compleatly matched the regex before
    /// invoking the callback.
    ///
    /// You should not register a command with too borad
    /// of a regex. So a regex "." or ".\s*" is bad.
    /// It can prevent the server from optimising its
    /// DFA, making all command dispatching take more
    /// memmory and beeing slower. If you are using
    /// lieutenant with its vanilla parsers/arguments
    /// then you should be fine.
    ///
    /// If you need to listen in on all chat msg's then
    /// you should ... @TODO give alternative.
    //pub fn add_command<P, T: FnMut(&mut CommandContext<P>, &str) -> u32>(
    pub fn add_command<
        'a,
        C: Command<GameState = (&'a mut Plugin, &'a mut CommandContext), CommandResult = i64>,
    >(
        &mut self,
        command: C,
    ) -> &mut Self {
        let command: Box<
            dyn Command<GameState = (&'a mut Plugin, &'a mut CommandContext), CommandResult = i64>,
        > = Box::new(command);
        let regex = command.regex();
        let data = Box::leak(Box::new(command)) as *mut Box<_> as *mut u8;

        let was_sucsess = unsafe {
            quill_sys::register_command(
                regex.as_str().as_ptr().into(),
                regex
                    .len()
                    .try_into()
                    .expect("Wow, the regex for the command was too big! (over 4gb, impressive)"),
                data.into(),
            )
        };

        if !was_sucsess {
            // Figure out what to do if a command was not able to be added.
            // TODO
        }

        self
    }
}
