extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{self, parse_macro_input, Data, DeriveInput, Error, Field, Fields, Ident, Result};

#[proc_macro_derive(BlockData)]
pub fn block_data_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    match get_fields(&ast) {
        Ok(fields) => {
            let block_data = impl_block_data(name, &fields);
            let get_and_set = impl_getters_and_setters(name, &fields);
            quote!(
                #block_data
                #get_and_set
            )
        }
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn get_fields(ast: &DeriveInput) -> Result<Vec<Field>> {
    let name = &ast.ident;
    let mut fields: Vec<Field>;

    if let Data::Struct(data) = &ast.data {
        if let Fields::Named(named_fields) = &data.fields {
            fields = named_fields
                .named
                .iter()
                .map(|field| field.to_owned())
                .collect();

            let valid = fields
                .iter()
                .position(|field| field.to_owned().ident.unwrap() == "valid_properties");

            if let Some(index) = valid {
                fields.remove(index);
            } else {
                return Err(Error::new(
                    name.span(),
                    "Can't derive BlockData for struct without valid_properties field",
                ));
            }
        } else {
            return Err(Error::new(
                name.span(),
                "Can't derive BlockData for a struct with no named fields",
            ));
        }
    } else {
        return Err(Error::new(
            name.span(),
            "Can't derive BlockData for a non struct",
        ));
    }

    Ok(fields)
}

fn impl_block_data(name: &Ident, fields: &[Field]) -> TokenStream2 {
    let idents: Vec<Ident> = fields.iter().map(|f| f.to_owned().ident.unwrap()).collect();
    quote!(
        impl BlockData for #name {
            fn from_raw(raw: &RawBlockStateProperties, valid: &'static ValidProperties) -> Option<Self>
            where
                Self: Sized,
            {
                Some(Self { #(#idents: raw.#idents?),*,valid_properties: valid, })
            }

            fn apply(&self, raw: &mut RawBlockStateProperties) {
                #(raw.#idents.replace(self.#idents));*;
            }
        }
    )
}

fn impl_getters_and_setters(name: &Ident, fields: &[Field]) -> TokenStream2 {
    let mut getters_setters = Vec::new();
    for field in fields {
        let field = field.to_owned();
        let ident = field.ident.unwrap();
        let ty = field.ty;
        let set_ident = format_ident!("set_{}", ident);
        let valid_ident = format_ident!("valid_{}", ident);
        getters_setters.push(quote! {
            pub fn #ident (&self) -> #ty {
                self.#ident
            }

            pub fn #set_ident (&mut self, value: #ty) -> bool {
                if self.valid_properties.#ident.contains(&value) {
                    self.#ident = value;
                    true
                } else {
                    false
                }
            }

            pub fn #valid_ident (&self) -> &[#ty] {
                &self.valid_properties.#ident
            }
        })
    }

    quote! {
        impl #name {
            #(#getters_setters)*
        }
    }
}
