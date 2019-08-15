use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use syn::braced;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;
use syn::Lit;
use syn::Token;

#[derive(Clone)]
struct EntityMetadata {
    ident: Ident,
    variants: HashMap<Ident, Variant>,
}

impl Parse for EntityMetadata {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let ident = input.parse()?;

        input.parse::<Token![,]>()?;

        let mut variants = HashMap::new();
        while let Ok(variant) = input.parse::<Variant>() {
            variants.insert(variant.ident.clone(), variant);

            input.parse::<Token![,]>()?;
        }

        Ok(Self { ident, variants })
    }
}

#[derive(Clone)]
struct Variant {
    ident: Ident,
    extends: Option<Ident>,
    entries: Vec<Entry>,
}

impl Parse for Variant {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let ident = input.parse()?;

        let extends = if input.parse::<Token![:]>().is_ok() {
            let ident = input.parse()?;
            Some(ident)
        } else {
            None
        };

        let content;
        braced!(content in input);

        let mut entries = vec![];

        while let Ok(entry) = content.parse::<Entry>() {
            entries.push(entry);
        }

        Ok(Self {
            ident,
            extends,
            entries,
        })
    }
}

#[derive(Clone)]
struct Entry {
    ty: EntryType,
    name: Ident,
    index: u8,
}

impl Parse for Entry {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let name = input.parse()?;
        let _ = input.parse::<Token![:]>()?;
        let ty = input.parse()?;

        let _ = input.parse::<Token![=]>()?;

        let index = match input.parse::<Lit>()? {
            Lit::Int(val) => val.base10_parse()?,
            _ => panic!("Index not a `u8`"),
        };

        let _ = input.parse::<Token![,]>()?;

        Ok(Self { ty, name, index })
    }
}

#[derive(PartialEq, Debug, Display, EnumString, Copy, Clone)]
enum EntryType {
    Byte,
    VarInt,
    Float,
    String,
    Slot,
    Boolean,
}

impl Parse for EntryType {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let ty = input.parse::<proc_macro2::Ident>()?;

        Ok(EntryType::from_rust_type(&ty.to_string()))
    }
}

impl EntryType {
    fn rust_type(self) -> &'static str {
        match self {
            EntryType::Byte => "u8",
            EntryType::VarInt => "i32",
            EntryType::Float => "f32",
            EntryType::String => "String",
            EntryType::Slot => "Slot",
            EntryType::Boolean => "bool",
        }
    }

    fn from_rust_type(ty: &str) -> Self {
        match ty {
            "u8" => EntryType::Byte,
            "VarInt" => EntryType::VarInt,
            "f32" => EntryType::Float,
            "String" => EntryType::String,
            "bool" => EntryType::Boolean,
            "Slot" => EntryType::Slot,
            _ => panic!("Invalid entry type {}", ty),
        }
    }
}

pub fn entity_metadata(input: TokenStream) -> TokenStream {
    let input: EntityMetadata = syn::parse_macro_input!(input);

    let mut structs = vec![];
    let mut enum_variants = vec![];

    let mut to_raw_metadata_arms = vec![];

    let enum_ident = input.ident.clone();

    for variant in input.variants.values() {
        let entries = get_metadata_entries(&input, variant.clone());

        let variant_ident = &variant.ident;

        let mut struct_fields = vec![];
        let mut struct_impl = vec![];
        let mut to_raw_metadata = vec![];

        let mut new_fn_parameters = vec![];
        let mut new_fn_contents = vec![];

        for entry in entries {
            let entry_ident = entry.name;
            let ty_enum = entry.ty;
            let ty = ty_enum.rust_type();
            let ty_ident = Ident::new(ty, Span::call_site());

            let is_dirty_name = format!("__is_dirty_{}", entry_ident);
            let is_dirty_ident = Ident::new(&is_dirty_name, Span::call_site());

            struct_fields.push(quote! {
                #entry_ident: #ty_ident,
                #is_dirty_ident: bool,
            });

            let set_fn_ident = Ident::new(&format!("set_{}", entry_ident), Span::call_site());
            let get_fn_ident = entry_ident.clone();

            struct_impl.push(quote! {
                pub fn #set_fn_ident(&mut self, val: #ty_ident) {
                    self.#entry_ident = val;
                    self.#is_dirty_ident = true;
                }

                pub fn #get_fn_ident(&self) -> #ty_ident {
                    self.#entry_ident.clone()
                }
            });

            let pass_reference = ty_enum == EntryType::Slot;

            let index = entry.index;
            let set_expr = if pass_reference {
                quote! { meta.set(#index, self.#entry_ident.clone()); }
            } else {
                quote! { meta.set(#index, self.#entry_ident); }
            };
            to_raw_metadata.push(quote! {
                if self.#is_dirty_ident {
                    #set_expr
                    self.#is_dirty_ident = false;
                }
            });

            new_fn_parameters.push(quote! {
                #entry_ident: #ty_ident
            });

            new_fn_contents.push(quote! {
                #entry_ident,
                #is_dirty_ident: true,
            });

            to_raw_metadata_arms.push(quote! {
                #enum_ident::#variant_ident(meta) => meta.to_raw_metadata(),
            })
        }

        struct_impl.push(quote! {
            pub fn new(#(#new_fn_parameters),*) -> Self {
                Self {
                    #(#new_fn_contents)*
                }
            }

            fn to_raw_metadata(&mut self) -> EntityMetadata {
                let mut meta = EntityMetadata::new();
                #(#to_raw_metadata)*
                meta
            }
        });

        structs.push(quote! {
            #[derive(Clone, Debug, Default)]
            pub struct #variant_ident {
                #(#struct_fields)*
            }

            impl #variant_ident {
                #(#struct_impl)*
            }
        });

        enum_variants.push(quote! {
            #variant_ident(#variant_ident),
        })
    }

    let result = quote! {
        #[derive(Clone, Debug)]
        pub enum #enum_ident {
            #(#enum_variants)*
        }

        impl #enum_ident {
            pub fn to_raw_metadata(&mut self) -> EntityMetadata {
                match self {
                    #(#to_raw_metadata_arms)*
                }
            }
        }

        #(#structs)*
    };

    result.into()
}

fn get_metadata_entries(metadata: &EntityMetadata, variant: Variant) -> Vec<Entry> {
    let mut entries = vec![];

    if let Some(inherits_from) = variant.extends.as_ref() {
        let inherits_from = &metadata.variants[inherits_from];
        entries.extend(get_metadata_entries(metadata, inherits_from.clone()).into_iter());
    }

    entries.extend(variant.entries.into_iter());
    entries
}
