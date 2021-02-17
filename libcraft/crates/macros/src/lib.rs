extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput, Error, Fields, Result, Ident};

#[proc_macro_derive(BlockData)]
pub fn block_data_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    impl_block_data(&ast).unwrap_or_else(|err| err.to_compile_error().into())
}

fn impl_block_data(ast: &DeriveInput) -> Result<TokenStream> {
    let name = &ast.ident;
    let fields: Vec<Ident>;

    if let Data::Struct(data) = &ast.data {
        if let Fields::Named(named_fields) = &data.fields {
            fields = named_fields.named.iter().map(|field| field.to_owned().ident.unwrap()).collect();
        } else {
            return Err(Error::new(
                name.span(),
                "Can't derive BlockData for a struct with no named fields",
            ))
        }
    } else {
        return Err(Error::new(
            name.span(),
            "Can't derive BlockData for a non struct",
        ))
    }

    let gen = quote! {
        impl BlockData for #name {
            fn from_raw(raw: &RawBlockStateProperties) -> Option<Self>
            where
                Self: Sized,
            {
                Some(Self { #(#fields: raw.#fields?),*, })
            }

            fn apply(&self, raw: &mut RawBlockStateProperties) {
                #(raw.#fields.replace(self.#fields));*;
            }
        }
    };

    Ok(gen.into())
}
