use darling::FromMeta;
use proc_macro_error::*;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs};
use syn::{spanned::Spanned, ItemFn};

#[derive(Debug, FromMeta)]
struct Args {
    name: String,
    description: Option<String>,
    author: Option<String>,
}

pub fn plugin(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = {
        let args = parse_macro_input!(args as AttributeArgs);
        Args::from_list(&args)
            .unwrap_or_else(|e| abort_call_site!("failed to parse arguments to plugin: {}", e))
    };
    let input = parse_macro_input!(input as ItemFn);

    verify_input(&input);

    let tokens = generate_output(&args, &input);
    tokens.into()
}

fn verify_input(input: &ItemFn) {
    if input.sig.ident != "init" {
        emit_error!(
            input.sig.ident.span(),
            "plugin main function must be called `init`"
        )
    }

    if let Some(generic) = input.sig.generics.params.iter().next() {
        emit_error!(
            generic.span(),
            "plugin main function may not have generic parameters"
        )
    }
}

/// Generates the output. This includes setting
/// the global allocator to delegate to the host,
/// creating a `__FAPI_INIT__` function called on initialization,
/// and finally calling the `init` function defined by the plugin.
/// This also sets the global logging instance, loads the host vtable, and initializes other
/// states.
fn generate_output(_args: &Args, input: &ItemFn) -> proc_macro2::TokenStream {
    quote! {
        #[cfg_attr(not(test), global_allocator)]
        static __FAPI_GLOBAL_ALLOCATOR__: ::fapi::alloc::Host = ::fapi::alloc::Host;

        // return value:
        // 0 => success
        // 1 => failed to deserialize vtable
        // 2 => versions not compatible
        // 3 => init() panicked
        #[no_mangle]
        pub extern "C" fn __FAPI_INIT__(state: *mut ::fapi::common::states::StartupState, vtable: ::fapi::common::util::FStr) -> u32 {
            if let Err(e) = ::fapi::vtable::init_vtable(vtable) {
                return 1;
            }

            // TODO: check versions
            let mut state = ::fapi::StartupState::from(state);

            if ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
                ::fapi::log_crate::set_logger(&::fapi::log::HOST).expect("failed to set logger");
                init(&mut state);
            })).is_err() {
                return 3;
            }

            0
        }

        #input
    }
}
