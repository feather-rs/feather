//! A WebAssembly-based plugin API for Minecraft servers.

pub mod entities;
mod entity;
mod entity_builder;
mod game;
pub mod query;
mod setup;

pub use entity::{Entity, EntityId};
pub use entity_builder::EntityBuilder;
pub use game::Game;
pub use setup::Setup;

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
#[doc(inline)]
pub use uuid::Uuid;

// Needed for macros
#[doc(hidden)]
pub extern crate bincode;
#[doc(hidden)]
pub extern crate quill_sys as sys;

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

        /// Never called by Quill, but this is needed
        /// to avoid linker errors with WASI.
        #[doc(hidden)]
        fn main() {}
    };
}
