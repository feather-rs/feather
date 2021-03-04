use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro]
pub fn include_data(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: LitStr = parse_macro_input!(input as LitStr);
    let build_dir = env::var("OUT_DIR").unwrap();

    let path = PathBuf::from(&build_dir).join(input.value());
    if !path.exists() {
        panic!("Path \"{}\" does not exist.", path.display());
    }
    let (dirs_files, _) = include_dirs_files(path);

    let tokens = quote! {
        #[doc = "The path of downloaded feather-data files."]
        pub const PATH: &str = #build_dir;
        #dirs_files
    };

    tokens.into()
}

fn include_dirs_files<P: AsRef<Path>>(path: P) -> (TokenStream, Vec<syn::Path>) {
    let path = path.as_ref();
    let (files, dirs): (Vec<PathBuf>, Vec<PathBuf>) = path
        .read_dir()
        .expect("could not read dir.")
        .map(|e| e.expect("Could not read entry."))
        .map(|e| e.path())
        .partition(|p| p.is_file());
    let (files_tokens, files_idents): (Vec<_>, Vec<_>) = files.iter().map(include_file).unzip();
    let (dirs_tokens, dirs_idents): (Vec<_>, Vec<_>) = dirs.iter().map(include_dir).unzip();
    let mut idents: Vec<syn::Path> = files_idents
        .into_iter()
        .map(|ident| {
            let segments = std::iter::once(syn::PathSegment {
                ident,
                arguments: syn::PathArguments::None,
            })
            .collect();
            syn::Path {
                leading_colon: None,
                segments,
            }
        })
        .collect();
    idents.extend(dirs_idents.into_iter().flatten());
    (
        quote! {
            #(#files_tokens)*
            #(#dirs_tokens)*
            pub const ALL: &'static [&'static [u8]] = &[#(#idents,)*];
        },
        idents,
    )
}

fn include_dir<P: AsRef<Path>>(path: P) -> (TokenStream, Vec<syn::Path>) {
    let path = path.as_ref();
    let stem = path
        .file_stem()
        .and_then(OsStr::to_str)
        .expect("Could not extract file stem.");
    let name = format_ident!("{}", stem);
    let (dirs_files, idents) = include_dirs_files(path);
    let idents: Vec<syn::Path> = idents
        .into_iter()
        .map(|path| {
            let segments = std::iter::once(syn::PathSegment {
                ident: name.clone(),
                arguments: syn::PathArguments::None,
            })
            .chain(path.segments.into_iter())
            .collect();
            syn::Path {
                leading_colon: None,
                segments,
            }
        })
        .collect();
    (
        quote! {
            pub mod #name {
                #dirs_files
            }
        },
        idents,
    )
}

fn include_file<P: AsRef<Path>>(path: P) -> (TokenStream, Ident) {
    let path = path.as_ref();
    let stem = path
        .file_stem()
        .and_then(OsStr::to_str)
        .expect("Could not extract file stem.");
    let name = {
        let stem = stem.to_uppercase();
        if stem.starts_with(char::is_numeric) {
            format_ident!("_{}", stem)
        } else {
            format_ident!("{}", stem)
        }
    };
    let path = format!("{}", path.display());
    (
        quote! {
            pub const #name: &'static [u8] = include_bytes!(#path);
        },
        name,
    )
}
