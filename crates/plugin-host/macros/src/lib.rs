use quote::{format_ident, quote};
use syn::{FnArg, GenericArgument, PathSegment};

/// Annotates a function so that it implements
/// the NativeHostFunction trait.
#[proc_macro_attribute]
pub fn host_function(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::ItemFn = syn::parse_macro_input!(input);

    let ident = input.sig.ident.clone();
    let gateway_ident = format_ident!("{}_gateway", input.sig.ident);
    let struct_ident = format_ident!("{}_struct", input.sig.ident);

    let args = input
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Receiver(_) => panic!("self functions are not supported"),
            FnArg::Typed(arg) => arg.clone(),
        })
        .collect::<Vec<_>>();
    let args_idents: Vec<_> = args.iter().map(|arg| arg.pat.clone()).collect();
    let args_idents_without_cx: Vec<_> = args_idents.iter().skip(1).cloned().collect();
    let args_without_cx: Vec<_> = args.iter().skip(1).cloned().collect();

    // Extract the inner return type from anyhow::Result<T>.
    let ret = match input.sig.output.clone() {
        syn::ReturnType::Default => return_type_panic(),
        syn::ReturnType::Type(_, ty) => match *ty {
            syn::Type::Path(path) => {
                let segments: Vec<PathSegment> = path.path.segments.into_iter().collect();
                if segments[0].ident != "anyhow" {
                    return_type_panic();
                }
                if segments[1].ident != "Result" {
                    return_type_panic();
                }

                let arg = segments[1].arguments.clone();
                match arg {
                    syn::PathArguments::AngleBracketed(inner) => {
                        match inner.args.first().unwrap() {
                            GenericArgument::Type(typ) => typ.clone(),
                            _ => return_type_panic(),
                        }
                    }
                    _ => return_type_panic(),
                }
            }
            _ => return_type_panic(),
        },
    };

    let result = quote! {
        #input

        extern "C" fn #gateway_ident(#(#args),*) -> #ret {
            #ident(#(#args_idents),*).expect("host function panicked")
        }

        #[allow(non_camel_case_types)]
        pub struct #struct_ident;

        impl crate::host_function::NativeHostFunction for #struct_ident {
            fn to_function_pointer(self) -> usize {
                // Safety: see the Nomicon: https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html#representation
                // For all targets that Feather compiles to, function pointers
                // have the same layout as a usize.

                unsafe {
                    std::mem::transmute(#gateway_ident as *const ())
                }
            }
        }

        impl crate::host_function::WasmHostFunction for #struct_ident {
            fn to_wasm_function(self, store: &wasmer::Store, env: crate::env::PluginEnv) -> wasmer::Function {
                wasmer::Function::new_native_with_env(store, env, |env: &crate::env::PluginEnv, #(#args_without_cx),*| {
                    let result: anyhow::Result<_> = #ident(&env.context, #(#args_idents_without_cx),*);
                    match result {
                        Ok(ret) => ret,
                        Err(e) => {
                            unsafe {
                                wasmer::raise_user_trap(e.into())
                            }
                        }
                    }
                })
            }
        }
    };
    result.into()
}

fn return_type_panic() -> ! {
    panic!("host functions must return an anyhow::Result<T>")
}
