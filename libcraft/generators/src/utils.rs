use std::collections::HashMap;
use std::path::PathBuf;

pub use quote::{format_ident, quote, ToTokens};
use serde::de::DeserializeOwned;
pub use serde::Deserialize;

pub use crate::{generate_enum, generate_enum_property};

const MINECRAFT_FILES_PATH: &str = "minecraft-data/data";
const MINECRAFT_DATA_VERSION: &str = "1.18";
const LIBCRAFT_FILES_PATH: &str = "libcraft-data";
pub const GENERATED_COMMENT: &str = "// This file is @generated. Please do not edit.";

pub fn load_minecraft_json<T>(name: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    let data_paths: HashMap<String, HashMap<String, HashMap<String, String>>> =
        serde_json::from_str(
            &std::fs::read_to_string(format!("{}/dataPaths.json", MINECRAFT_FILES_PATH)).unwrap(),
        )?;
    let paths = &data_paths["pc"][MINECRAFT_DATA_VERSION];
    serde_json::from_str(
        &std::fs::read_to_string(format!(
            "{}/{}/{}",
            MINECRAFT_FILES_PATH,
            paths[name.trim_end_matches(".json")],
            name
        ))
        .unwrap(),
    )
}

pub fn load_libcraft_json<T>(name: &str) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    serde_json::from_slice(&std::fs::read(format!("{}/{}", LIBCRAFT_FILES_PATH, name)).unwrap())
}

/// Writes the contents to a file in provided path, then runs rustfmt.
///
/// Parameters:
/// path: Path to destination file, relative to feather root
/// content: Contents to be written in the file
pub fn output(file: &str, content: &str) {
    let path = std::env::current_dir().unwrap().join(file);
    if !path.parent().unwrap().exists() {
        panic!(
            "Couldn't write to file.\nPath {} does not exist",
            path.parent().unwrap().to_str().unwrap()
        );
    }
    std::fs::write(&path, format!("{}\n{}", GENERATED_COMMENT, content)).unwrap();
    println!("Generated {}", path.to_str().unwrap());

    rustfmt(path)
}

fn rustfmt(path: impl Into<PathBuf> + Clone) {
    std::process::Command::new("rustfmt")
        .arg(path.into().to_str().unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

/// Generates an enum definition with the provided variants and extra derives.
#[macro_export]
macro_rules! generate_enum {
    ($name: ident, $variants: expr $(,)?) => {
        generate_enum!($name, $variants, [])
    };
    ($name: ident, $variants: expr, [$($derive: ty),* $(,)?] $(, $(#[$prelude: meta]),*)?) => {
        {
            use proc_macro2::TokenStream;

            let prelude: TokenStream = quote! {
                $($(
                    #[$prelude]
                )*)?
            };

            let variants = $variants
                .into_iter()
                .map(|variant| format_ident!("{}", variant))
                .collect::<Vec<_>>();
            let derives = quote! { #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, $($derive),*)] };

            quote! {
                #derives
                #prelude
                pub enum $name {
                    #(#variants),*
                }

                impl $name {
                    #[inline]
                    pub fn values() -> &'static [$name] {
                        use $name::*;
                        &[
                            #(#variants),*
                        ]
                    }
                }
            }
        }
    };
}

/// Generates lookup functions for an enum.
///
/// Generates two function for an enum, one which maps the enum value to some
/// property value and one which does the reverse (returning an Option)
#[macro_export]
macro_rules! generate_enum_property {
    ($enum_name: ident, $property_name: literal, $typ: ty, $mapping: expr $(,)?) => {
        generate_enum_property!($enum_name, $property_name, $typ, $mapping, false)
    };
    ($enum_name: ident, $property_name: literal, $typ: ty, $mapping: expr, $reverse: expr $(,)?) => {
        generate_enum_property!($enum_name, $property_name, $typ, $mapping, $reverse, $typ)
    };
    ($enum_name: ident, $property_name: literal, $typ: ty, $mapping: expr, $reverse: expr, $return_type: ty $(,)?) => {
        generate_enum_property!(
            $enum_name,
            $property_name,
            $typ,
            $mapping,
            $reverse,
            $return_type,
            false
        )
    };
    ($enum_name: ident, $property_name: literal, $typ: ty, $mapping: expr, $reverse: expr, $return_type: ty, $needs_bindings: expr $(,)?) => {{
        use indexmap::IndexMap;
        use proc_macro2::TokenStream;

        let property_name: &str = $property_name;
        let mapping: IndexMap<String, TokenStream> = $mapping;
        let reverse: bool = $reverse;
        let needs_bindings: bool = $needs_bindings;

        let mut self_to_prop = Vec::new();
        let mut prop_to_self = Vec::new();

        for (enum_variant, property_value) in mapping {
            let fields = if needs_bindings {
                quote! { { .. } }
            } else {
                quote! {}
            };
            let enum_variant = format_ident!("{}", enum_variant);
            self_to_prop.push(quote! {
                $enum_name::#enum_variant #fields => { #property_value }
            });
            prop_to_self.push(quote! {
                #property_value => Some($enum_name::#enum_variant)
            });
        }

        let mut fns = Vec::new();

        let property_name_ident = format_ident!("{}", property_name);
        let doc = format!(
            "Returns the `{}` property of this `{}`.",
            property_name,
            quote! { $enum_name }.to_string()
        );
        fns.push(quote! {
            #[doc = #doc]
            #[inline]
            pub fn #property_name_ident(&self) -> $return_type {
                match self {
                    #(#self_to_prop),*
                }
            }
        });

        if reverse {
            let fn_name = format_ident!("from_{}", property_name);
            let doc = format!(
                "Gets a `{}` by its `{}`.",
                quote! { $enum_name }.to_string(),
                property_name
            );
            fns.push(quote! {
                #[doc = #doc]
                #[inline]
                pub fn #fn_name(#property_name_ident: $typ) -> Option<Self> {
                    match #property_name_ident {
                        #(#prop_to_self),*,
                        _ => None
                    }
                }
            });
        }

        quote! {
            impl $enum_name {
                #(#fns)*
            }
        }
    }};
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdAndName {
    pub id: u32,
    pub name: String,
}
