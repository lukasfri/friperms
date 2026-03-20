extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_quote};

pub fn set_eq_derive(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: DeriveInput = syn::parse2(input).unwrap();

    let crate_name: syn::Path = parse_quote!(::finit);
    let struct_name = &input.ident;

    let Data::Struct(struct_data) = &input.data else {
        unimplemented!("Currently, there is only support for structs.");
    };

    let function_body: Vec<proc_macro2::TokenStream> = match &struct_data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Struct is named.");
                quote! {
                    #crate_name::comparisons::SetEq::set_eq(&self.#field_name, &rhs.#field_name)
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::comparisons::SetEq::set_eq(&self.#i, &rhs.#i)
                }
            })
            .collect(),
        syn::Fields::Unit => {
            return quote! {
              compile_error!("Unit structs can't be a set.")
            }
            .into();
        }
    };

    let function_body = function_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc && #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl #crate_name::comparisons::SetEq<#struct_name> for #struct_name {
            fn set_eq(&self, rhs: &#struct_name) -> bool {
                #function_body
            }
        }
    }
    .into()
}

pub fn subset_of_derive(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: DeriveInput = syn::parse2(input).unwrap();

    let crate_name: syn::Path = parse_quote!(::finit);
    let struct_name = &input.ident;

    let Data::Struct(struct_data) = &input.data else {
        unimplemented!("Currently, there is only support for structs.");
    };

    let function_body: Vec<proc_macro2::TokenStream> = match &struct_data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Struct is named.");
                quote! {
                    #crate_name::comparisons::SubsetOf::subset_of(&self.#field_name, &rhs.#field_name)
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::comparisons::SubsetOf::subset_of(&self.#i, &rhs.#i)
                }
            })
            .collect(),
        syn::Fields::Unit => {
            return quote! {
              compile_error!("Unit structs can't be a set.")
            }
            .into();
        }
    };

    let function_body = function_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc && #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl #crate_name::comparisons::SubsetOf<#struct_name> for #struct_name {
            fn subset_of(&self, rhs: &#struct_name) -> bool {
                #function_body
            }
        }
    }
    .into()
}
