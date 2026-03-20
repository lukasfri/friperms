extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn operation_assign_derive(
    input: TokenStream,
    trait_path: &syn::Path,
    fn_name: &syn::Ident,
) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: DeriveInput = syn::parse2(input).unwrap();

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
                    #trait_path::#fn_name(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #trait_path::#fn_name(&mut self.#i, &rhs.#i);
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
        .reduce(|acc, value| quote! { #acc #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl #trait_path<&#struct_name> for #struct_name {
            fn #fn_name(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

pub fn operation_derive(
    input: TokenStream,
    trait_path: &syn::Path,
    fn_name: &syn::Ident,
) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: DeriveInput = syn::parse2(input).unwrap();

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
                    #field_name: #trait_path::#fn_name(self.#field_name, &rhs.#field_name)
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #trait_path::#fn_name(self.#i, &rhs.#i)
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

    let struct_init = match &struct_data.fields {
        syn::Fields::Named(_) => quote!(
          #struct_name {
            #(#function_body,)*
          }
        ),
        syn::Fields::Unnamed(_) => quote!(
          #struct_name {
            #(#function_body),*
          }
        ),
        syn::Fields::Unit => unreachable!("Already returned error earlier."),
    };

    quote! {
        impl #trait_path<&#struct_name> for #struct_name {
            type Output = Self;

            fn #fn_name(self, rhs: &#struct_name) -> Self::Output {
                #struct_init
            }
        }
    }
    .into()
}
