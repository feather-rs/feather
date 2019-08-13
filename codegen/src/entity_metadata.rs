use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::quote;
use std::collections::HashMap;
use std::str::FromStr;
use syn::braced;
use syn::parse::{Parse, ParseBuffer};
use syn::Error;
use syn::Token;

#[derive(Clone)]
struct EntityMetadata {
    ident: Ident,
    variants: HashMap<Ident, Variant>,
}

impl Parse for EntityMetadata {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let ident = input.parse()?;

        let mut variants = HashMap::new();
        while let Ok(variant) = input.parse::<Variant>() {
            variants.insert(variant.ident.clone(), variant);
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

        let extends = if let Ok(_) = input.parse::<Token![:]>() {
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
}

impl Parse for Entry {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let ty = input.parse()?;
        let _ = input.parse::<Token![:]>()?;
        let name = input.parse()?;
        let _ = input.parse::<Token![,]>()?;

        Ok(Self { ty, name })
    }
}

#[derive(PartialEq, Debug, Display, EnumString, Copy, Clone)]
enum EntryType {
    Byte,
    VarInt,
    Float,
    String,
    Chat,
    OptChat,
    Slot,
    Boolean,
    Rotation,
    Position,
    OptPosition,
    Direction,
    OptUuid,
    OptBlockId,
}

impl Parse for EntryType {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let ty = input.parse::<proc_macro2::Ident>()?;

        let entry_ty = EntryType::from_str(&ty.to_string())
            .map_err(|_| Error::new(Span::call_site(), "Invalid metadata entry type"))?;
        Ok(entry_ty)
    }
}

pub fn entity_metadata(input: TokenStream) -> TokenStream {
    let input: EntityMetadata = syn::parse_macro_input!(input);

    let mut structs = vec![];
    let mut enum_variants = vec![];

    for (_, variant) in &input.variants {
        let entries = get_metadata_entries(&input, variant.clone());

        let ident = &variant.ident;

        let mut struct_fields = vec![];

        for entry in entries {
            let ident = entry.name;
            let ty = entry.ty;
            let ty = format!("{:?}", ty);
            let ty_ident = Ident::new(&ty, Span::call_site());
            struct_fields.push(quote! {
                #ident: #ty_ident,
            });
        }

        structs.push(quote! {
            #[derive(Clone, Debug)]
            pub struct #ident {
                #(#struct_fields)*
            }
        });

        enum_variants.push(quote! {
            #ident(#ident),
        })
    }

    let ident = input.ident;

    let result = quote! {
        #[derive(Clone, Debug)]
        pub enum #ident {
            #(#enum_variants)*
        }

        #(structs)*
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
