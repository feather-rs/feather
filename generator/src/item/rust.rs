//! Generates Rust code for `Item` enum.

use crate::item::ItemReport;
use failure::Error;
use heck::CamelCase;
use proc_macro2::{Ident, Span};

pub fn generate_rust(report: ItemReport) -> Result<String, Error> {
    let mut enum_variants = vec![];
    let mut from_identifier_arms = vec![];
    let mut to_identifier_arms = vec![];

    for (identifier, _) in report.mappings {
        let variant_name = ident(&variant_name(&identifier));
        enum_variants.push(quote! {
            #variant_name
        });

        from_identifier_arms.push(quote! {
            #identifier => Some(Item::#variant_name)
        });

        to_identifier_arms.push(quote! {
            Item::#variant_name => #identifier
        });
    }

    let result = quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ToPrimitive, FromPrimitive)]
        pub enum Item {
            #(#enum_variants, )*
        }

        impl Item {
            pub fn from_identifier(identifier: &str) -> Option<Self> {
                match identifier {
                    #(#from_identifier_arms, )*
                    _ => None,
                }
            }

            pub fn identifier(self) -> &'static str {
                match self {
                    #(#to_identifier_arms, )*
                }
            }
        }
    };

    Ok(result.to_string())
}

/// Strips away the "minecraft:" prefix from a item string ID.
fn strip_prefix(val: &str) -> String {
    val[10..].to_string()
}

/// Returns the enum variant name for the given item identifier.
fn variant_name(identifier: &str) -> String {
    strip_prefix(identifier).to_camel_case()
}

fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}
