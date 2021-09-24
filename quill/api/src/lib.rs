//! A WebAssembly-based plugin API for Minecraft servers.

// Needed for macros
#[doc(hidden)]
pub extern crate bincode;
#[doc(hidden)]
pub extern crate quill_sys as sys;

#[doc(hidden)]
pub use commands;
#[doc(inline)]
pub use uuid::Uuid;

pub use command::*;
pub use entity::{Entity, EntityId};
pub use entity_builder::EntityBuilder;
pub use game::Game;
#[doc(inline)]
pub use libcraft_blocks::{BlockKind, BlockState};
#[doc(inline)]
pub use libcraft_core::{BlockPosition, ChunkPosition, Gamemode, Position};
#[doc(inline)]
pub use libcraft_particles::{Particle, ParticleKind};
#[doc(inline)]
pub use libcraft_text::*;
#[doc(inline)]
pub use quill_common::{components, entity_init::EntityInit, events, Component};
pub use setup::Setup;

pub mod command;
pub mod entities;
mod entity;
mod entity_builder;
mod game;
pub mod query;
mod setup;

/// Implement this trait for your plugin's struct.
pub trait Plugin: Sized {
    /// Invoked when the plugin is enabled.
    ///
    /// Here, you should register systems and initialize
    /// any plugin state.
    ///
    /// # Warning
    /// This function is called when your plugin _enabled_. That
    /// is not guaranteed to coincide with the time the server starts
    /// up. Do not assume that the server has just started when
    /// this method is called.
    fn enable(game: &mut Game, setup: &mut Setup<Self>) -> Self;

    /// Invoked before the plugin is disabled.
    ///
    /// # Warning
    /// Like [`Plugin::enable`], this method is not necessarily called
    /// when the server shuts down. Users may choose to disable
    /// plugins at another time. Therefore, do not assume that
    /// the server is shutting down when this method is called.
    fn disable(self, game: &mut Game);
}

/// Invoke this macro in your plugin's main.rs.
///
///  Give it the name of your struct implementing `Plugin`.
///
/// # Example
/// ```no_run
/// // main.rs
/// use quill::{Plugin, Setup, Game};
///
/// quill::plugin!(MyPlugin);
///
/// pub struct MyPlugin {
///    // plugin state goes here
/// }
///
/// impl Plugin for MyPlugin {
///     fn enable(game: &mut Game, setup: &mut Setup<Self>) -> Self {
///         // Initialize plugin state...
///         Self {}
///     }
///
///     fn disable(self, game: &mut Game) {
///         // Clean up...
///     }
/// }
/// ```
#[macro_export]
macro_rules! plugin {
    ($plugin:ident) => {
        // `static mut` can be used without synchronization because the host
        // guarantees it will not invoke plugin systems outside of the main thread.
        static mut PLUGIN: Option<$plugin> = None;

        type CommandContext<'a> = $crate::CommandContext<'a, $plugin>;

        // Exports to the host required for all plugins
        #[no_mangle]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        pub unsafe extern "C" fn quill_setup() {
            let plugin: $plugin =
                quill::Plugin::enable(&mut $crate::Game::new(), &mut $crate::Setup::new());
            PLUGIN = Some(plugin);
        }

        #[no_mangle]
        #[doc(hidden)]
        #[cfg(not(target_arch = "wasm32"))]
        pub unsafe extern "C" fn quill_setup(
            context: *const (),
            vtable_ptr: *const u8,
            vtable_len: usize,
        ) {
            // Set up vtable and host context for quill_sys.
            let vtable_bytes = ::std::slice::from_raw_parts(vtable_ptr, vtable_len);
            let vtable: ::std::collections::HashMap<&str, usize> =
                $crate::bincode::deserialize(vtable_bytes).expect("invalid vtable");

            $crate::sys::init_host_context(context);
            $crate::sys::init_host_vtable(&vtable)
                .expect("invalid vtable (check that the plugin and host are up to date)");

            let plugin: $plugin =
                quill::Plugin::enable(&mut $crate::Game::new(), &mut $crate::Setup::new());
            PLUGIN = Some(plugin);
        }

        #[no_mangle]
        #[doc(hidden)]
        pub unsafe extern "C" fn quill_allocate(size: usize, align: usize) -> *mut u8 {
            std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(size, align))
        }

        #[no_mangle]
        #[doc(hidden)]
        pub unsafe extern "C" fn quill_deallocate(ptr: *mut u8, size: usize, align: usize) {
            std::alloc::dealloc(
                ptr,
                std::alloc::Layout::from_size_align_unchecked(size, align),
            )
        }

        #[no_mangle]
        #[doc(hidden)]
        pub unsafe extern "C" fn quill_run_system(data: *mut u8) {
            let system = &mut *data.cast::<Box<dyn FnMut(&mut $plugin, &mut $crate::Game)>>();
            let plugin = PLUGIN.as_mut().expect("quill_setup never called");
            system(plugin, &mut $crate::Game::new());
        }

        #[no_mangle]
        #[doc(hidden)]
        pub unsafe extern "C" fn quill_run_command(
            data: *mut u8,
            args: *mut u8,
            args_len: u32,
            ctx: *mut u8,
        ) -> u32 {
            let executor = &*data.cast::<Box<
                dyn Fn($crate::commands::dispatcher::Args, $crate::CommandContext<$plugin>) -> bool,
            >>();
            let ctx = &*ctx.cast::<$crate::CommandContext<()>>();
            let ctx = $crate::CommandContext {
                game: $crate::Game::new(),
                caller: ctx.caller.clone(),
                plugin: PLUGIN.as_mut().expect("quill_setup never called"),
            };
            let args = Vec::from_raw_parts(
                args as *mut Box<dyn std::any::Any>,
                args_len as usize,
                args_len as usize,
            );
            executor(args, ctx) as u32
        }

        #[no_mangle]
        #[doc(hidden)]
        pub unsafe extern "C" fn quill_run_command_completer(
            data: *mut u8,
            text: *mut u8,
            text_len: u32,
            ctx: *mut u8,
        ) -> (u32, u32, u32) {
            let complete =
                &*data.cast::<Box<
                    dyn Fn(&str, $crate::CommandContext<$plugin>) -> Vec<(String, Option<String>)>,
                >>();
            let ctx = &*ctx.cast::<$crate::CommandContext<()>>();
            let ctx = $crate::CommandContext {
                game: Game::new(),
                caller: ctx.caller.clone(),
                plugin: PLUGIN.as_mut().expect("quill_setup never called"),
            };
            let text = String::from_raw_parts(text, text_len as usize, text_len as usize);
            let completions = complete(&text, ctx)
                .into_iter()
                .map(|(c, t)| {
                    (
                        (c.as_ptr(), c.len(), c.capacity()),
                        t.is_some(),
                        if t.is_none() {
                            (0, 0, 0)
                        } else {
                            (
                                t.as_ref().unwrap().as_ptr() as usize as u32,
                                t.as_ref().unwrap().len() as u32,
                                t.as_ref().unwrap().capacity() as u32,
                            )
                        },
                    )
                })
                .collect::<Vec<_>>();
            (
                completions.as_ptr() as usize as u32,
                completions.len() as u32,
                completions.capacity() as u32,
            )
        }

        /// Never called by Quill, but this is needed
        /// to avoid linker errors with WASI.
        #[doc(hidden)]
        fn main() {}
    };
}
