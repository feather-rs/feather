use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

/// Invoke this macro in your plugin's main.rs.
///
///  Give it the name of your struct implementing `Plugin`.
///
/// # Example
/// ```ignore
/// // main.rs
/// use quill::{Plugin, Setup, Game};
///
/// #[quill::plugin]
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
#[proc_macro_attribute]
pub fn plugin(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let cloned_item = item.clone();
    let input = parse_macro_input!(cloned_item as Item);

    let name = match input {
        Item::Enum(itm_enum) => itm_enum.ident,
        Item::Struct(itm_str) => itm_str.ident,
        _ => panic!("Only structs or enums can be #[quill::plugin]!"),
    };
    let res = quote! {
        // `static mut` can be used without synchronization because the host
        // guarantees it will not invoke plugin systems outside of the main thread.
        static mut PLUGIN: Option<#name> = None;

        // Exports to the host required for all plugins
        #[no_mangle]
        #[doc(hidden)]
        #[cfg(target_arch = "wasm32")]
        pub unsafe extern "C" fn quill_setup() {
            let plugin: #name =
                quill::Plugin::enable(&mut ::quill::Game::new(), &mut ::quill::Setup::new());
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
                ::quill::bincode::deserialize(vtable_bytes).expect("invalid vtable");

            ::quill::sys::init_host_context(context);
            ::quill::sys::init_host_vtable(&vtable)
                .expect("invalid vtable (check that the plugin and host are up to date)");

            let plugin: #name =
                quill::Plugin::enable(&mut ::quill::Game::new(), &mut ::quill::Setup::new());
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
            let system = &mut *data.cast::<Box<dyn FnMut(&mut #name, &mut ::quill::Game)>>();
            let plugin = PLUGIN.as_mut().expect("quill_setup never called");
            system(plugin, &mut ::quill::Game::new());
        }

        /// Never called by Quill, but this is needed
        /// to avoid linker errors with WASI.
        #[doc(hidden)]
        fn main() {}
    };
    item.extend(TokenStream::from(res));

    item
}
