use crate::frontend::{Data, Enum, Property, Value};
use crate::model::Type;
use anyhow::Context;
use heck::CamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use syn::export::ToTokens;

/// Given the `Data`, generates code.
pub fn generate(target_dir: &str, data: &Data) -> anyhow::Result<()> {
    let _ = std::fs::remove_dir(target_dir);
    std::fs::create_dir_all(target_dir)
        .with_context(|| format!("failed to create directory `{}`", target_dir))?;

    let mut open_files = HashMap::new();

    let mut module_names = BTreeSet::new();

    for (file_name, fdata) in &data.files {
        let path = format!("{}/{}.rs", target_dir, file_name);
        module_names.insert(file_name);

        let file = match open_files.get_mut(&path) {
            Some(file) => file,
            None => {
                let mut file = File::create(&path)
                    .with_context(|| format!("failed to create file `{}`", path))?;
                file.write_all(b"// This file is @generated\n")
                    .with_context(|| format!("failed to write to file `{}`", path))?;
                open_files.insert(path.clone(), file);
                open_files.get_mut(&path).unwrap()
            }
        };

        let tokens = fdata.enums.values().map(generate_enum).collect::<Vec<_>>();
        let tokens2 = fdata
            .properties
            .iter()
            .map(|prop| generate_property(data, prop))
            .collect::<anyhow::Result<Vec<_>>>()
            .with_context(|| format!("failed to generate properties in file `{}`", file_name))?;

        let tokens = quote! { #(#tokens)* #(#tokens2)* };

        file.write_all(tokens.to_string().as_bytes())
            .with_context(|| format!("failed to write bytes to `{}`", path))?;
    }

    // Write out mod.rs
    let lib_path = format!("{}/mod.rs", target_dir);
    let mut lib = File::create(&lib_path)?;
    lib.write_all(b"// This file is @generated\n")?;
    for module in module_names {
        lib.write_all(format!("mod {}; pub use {}::*;", module, module).as_bytes())?;
    }
    open_files.insert(lib_path, lib);

    for (path, mut file) in open_files {
        file.flush()?;

        if !Command::new("rustfmt").arg(&path).status()?.success() {
            anyhow::bail!("failed to run rustfmt on file {}", path);
        }
    }

    Ok(())
}

fn generate_enum(e: &Enum) -> TokenStream {
    let def = generate_enum_body(e);

    quote! {
        #def
    }
}

fn generate_enum_body(e: &Enum) -> TokenStream {
    let name = ident(&e.name_camel_case);
    let variants: Vec<_> = e.variants_camel_case.iter().map(ident).collect();

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ToPrimitive, FromPrimitive)]
        pub enum #name {
            #(#variants,)*
        }
    }
}

impl<'a> ToTokens for Type<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let t = match self {
            Type::Bool => quote! { bool },
            Type::Slice(inner) => quote! { &'static [#inner] },
            Type::U32 => quote! { u32 },
            Type::F64 => quote! { f64 },
            Type::String => quote! { &'static str },
            Type::Custom(name) => {
                let name = ident(name.to_camel_case());
                quote! { crate::#name }
            }
        };
        tokens.extend(t);
    }
}

impl<'a> Type<'a> {
    pub fn to_tokens_no_static_lifetime(&self) -> TokenStream {
        match self {
            Type::String => quote! { &str },
            typ => quote! { #typ },
        }
    }
}

impl Value {
    fn tokens(&self, typ: &Type) -> TokenStream {
        match self {
            Value::Bool(x) => quote! { #x },
            Value::Slice(x) => {
                let mut stream = TokenStream::new();
                let inner = if let Type::Slice(inner) = typ {
                    inner
                } else {
                    panic!()
                };

                for value in x {
                    stream.extend(value.tokens(inner.as_ref()));
                }

                stream
            }
            Value::U32(x) => quote! { #x },
            Value::F64(x) => quote! { #x },
            Value::String(x) => quote! { #x },
            Value::Custom(name) => {
                let type_name = if let Type::Custom(x) = typ {
                    ident(x.to_camel_case())
                } else {
                    panic!()
                };
                let name = ident(name.to_camel_case());
                quote! { crate::#type_name::#name }
            }
        }
    }
}

fn generate_property(data: &Data, property: &Property) -> anyhow::Result<TokenStream> {
    let name = ident(property.on.to_camel_case());

    let property_name = ident(&property.name);
    let property_type = &property.typ;

    let e = data
        .files
        .values()
        .filter_map(|models| models.enums.get(&property.on))
        .next()
        .ok_or_else(|| anyhow::anyhow!("no enum matched the name `{}`", property.on))?;

    let mut exhaustive = property.mapping.len() == e.variants.len();

    if let Type::Bool = property.typ {
        exhaustive = true;
    }

    let mut match_arms = vec![];
    for (variant, value) in &property.mapping {
        let variant = ident(variant.to_camel_case());
        let value_tokens = value.tokens(&property.typ);

        let value = if exhaustive {
            quote! { #value_tokens }
        } else {
            quote! { Some(#value_tokens) }
        };

        match_arms.push(quote! {
            crate::#name::#variant => #value,
        });
    }

    if !exhaustive {
        match_arms.push(quote! {
            _ => None,
        });
    }

    if let Type::Bool = property.typ {
        match_arms.push(quote! { _ => false, });
    }

    let ret = if exhaustive {
        quote! { #property_type }
    } else {
        quote! { Option<#property_type> }
    };

    let to_prop = quote! {
        pub fn #property_name(self) -> #ret {
            match self {
                #(#match_arms)*
            }
        }
    };

    let from_prop = if property.reverse {
        // generate from_prop as well
        let function_name = ident(format!("from_{}", property.name));
        let property_type = property_type.to_tokens_no_static_lifetime();

        let mut match_arms = vec![];

        for (variant, value) in &property.mapping {
            let variant = ident(variant.to_camel_case());
            let value_tokens = value.tokens(&property.typ);

            match_arms.push(quote! {
                #value_tokens => Some(crate::#name::#variant)
            });
        }
        match_arms.push(quote! { _ => None });

        quote! {
            pub fn #function_name(prop: #property_type) -> Option<#name> {
                match prop {
                    #(#match_arms,)*
                }
            }
        }
    } else {
        quote! {}
    };

    let tokens = quote! {
        impl crate::#name {
            #to_prop
            #from_prop
        }
    };
    Ok(tokens)
}

fn ident(s: impl AsRef<str>) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}
