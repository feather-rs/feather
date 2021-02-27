use quote::quote;
use syn::ForeignItem;

/// Macro to redefine host functions depending
/// on whether we are compiling to the WebAssembly
/// or the native target.
///
/// See the `quill-sys` crate-level documentation for more info.
#[proc_macro_attribute]
pub fn host_functions(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::ItemForeignMod = syn::parse_macro_input!(input as syn::ItemForeignMod);

    let mut functions = Vec::new();
    for item in &input.items {
        let item = match item {
            ForeignItem::Fn(f) => f,
            _ => panic!("only functions may be defined within the host calls module"),
        };
        functions.push(item.clone());
    }

    let mut vtable_entries = Vec::new();
    for function in &functions {
        let ident = &function.sig.ident;
        let args: Vec<_> = function
            .sig
            .inputs
            .iter()
            .map(|arg| match arg {
                syn::FnArg::Receiver(_) => panic!("self argument"),
                syn::FnArg::Typed(arg) => arg.ty.clone(),
            })
            .collect();
        let ret = &function.sig.output;
        let ret = match ret {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(quote! { -> #ty }),
        };
        vtable_entries.push(quote! {
            #ident: unsafe extern "C" fn(*const (), #(#args),*) #ret
        });
    }

    let vtable = quote! {
        struct HostVTable {
            #(#vtable_entries,)*
        }
    };

    let mut vtable_init_bindings = Vec::new();
    let mut vtable_init = Vec::new();
    for function in &functions {
        let ident = &function.sig.ident;
        let ident_string = ident.to_string();
        let missing_error = format!("missing vtable entry {}", ident_string);

        vtable_init_bindings.push(quote! {
            let #ident = *vtable.get(#ident_string).ok_or_else(|| #missing_error)?;
            // Safety: Transmute from a usize to a function pointer.
            // This is valid on all targeted native platforms.
            let #ident = std::mem::transmute::<usize, _>(#ident);
        });

        vtable_init.push(ident.clone());
    }

    let vtable_init = quote! {
        #[doc = "Initializes the host vtable."]
        #[doc = "Safety: the host vtable must not already be initialized."]
        pub unsafe fn init_host_vtable(vtable: &std::collections::HashMap<&str, usize>) -> Result<(), &'static str> {
            #(#vtable_init_bindings)*
            HOST_VTABLE = Some(HostVTable {
                #(#vtable_init,)*
            });
            Ok(())
        }
    };

    let through_vtable_functions: Vec<_> = functions
        .iter()
        .map(|function| {
            let ident = &function.sig.ident;
            let args = &function.sig.inputs;
            let ret = match &function.sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_, ty) => Some(quote! { -> #ty }),
            };
            let value_args: Vec<_> = args
                .iter()
                .map(|arg| match arg {
                    syn::FnArg::Receiver(_) => panic!("host functions cannot take self"),
                    syn::FnArg::Typed(arg) => arg.pat.clone(),
                })
                .collect();
            let attrs = &function.attrs;
            quote! {
                #(#attrs)*
                pub unsafe fn #ident(#args) #ret {
                    let vtable = HOST_VTABLE.as_ref().expect("vtable not initialized");
                    let context = HOST_CONTEXT.expect("context not initialized");
                    (vtable.#ident)(context, #(#value_args),*)
                }
            }
        })
        .collect();

    let attrs = &input.attrs;

    let result = quote! {
        #[cfg(target_arch = "wasm32")]
        #(#attrs)*
        extern "C" {
            #(#functions)*
        }

        #[cfg(not(target_arch = "wasm32"))]
        mod host_functions {
            use super::*;

            static mut HOST_VTABLE: Option<HostVTable> = None;
            static mut HOST_CONTEXT: Option<*const ()> = None;

            #vtable

            #vtable_init

            #[doc = "Sets the host context."]
            #[doc = "Safety: can only be called once."]
            pub unsafe fn init_host_context(context: *const ()) {
                HOST_CONTEXT = Some(context);
            }

            #(#through_vtable_functions)*
        }
        #[cfg(not(target_arch = "wasm32"))]
        pub use host_functions::*;
    };
    result.into()
}
