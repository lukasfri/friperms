extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

#[proc_macro_derive(Set)]
pub fn set_derive_method(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: DeriveInput = syn::parse2(input).unwrap();
    let struct_name = &input.ident;

    let Data::Struct(struct_data) = &input.data else {
        unimplemented!("Currently, there is only support for structs.");
    };

    let is_empty_body: Vec<proc_macro2::TokenStream> = match &struct_data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Struct is named.");
                quote! {
                    ::friperms::Set::is_empty(&self.#field_name)
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    ::friperms::Set::is_empty(&self.#i)
                }
            })
            .collect(),
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
    };

    let is_empty_body = is_empty_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc & #value})
        .expect("No unit structs means there must be atleast 1 field.");

    let empty_body: Vec<proc_macro2::TokenStream> = match &struct_data.fields {
        syn::Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let field_name = field.ident.as_ref().expect("Struct is named.");
                let field_type = &field.ty;
                quote! {
                    #field_name: <#field_type as ::friperms::Set>::empty(),
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    ::friperms::Set::is_empty(&self.#i)
                }
            })
            .collect(),
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
    };

    let empty_body = empty_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl ::friperms::Set for #struct_name {
            fn is_empty(&self) -> bool {
                #is_empty_body
            }

            fn empty() -> Self {
                Self {
                    #empty_body
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(UnionInPlace)]
pub fn union_in_place_derive_method(input: TokenStream) -> TokenStream {
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
                    ::friperms::UnionInPlace::union_in_place(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    ::friperms::UnionInPlace::union_in_place(&mut self.#i, &rhs.#i);
                }
            })
            .collect(),
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
    };

    let function_body = function_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl ::friperms::UnionInPlace<&#struct_name> for #struct_name {
            fn union_in_place(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(DifferenceInPlace)]
pub fn difference_in_place_derive_method(input: TokenStream) -> TokenStream {
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
                    ::friperms::DifferenceInPlace::difference_in_place(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    ::friperms::DifferenceInPlace::difference_in_place(&mut self.#i, &rhs.#i);
                }
            })
            .collect(),
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
    };

    let function_body = function_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl ::friperms::DifferenceInPlace<&#struct_name> for #struct_name {
            fn difference_in_place(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(IntersectionInPlace)]
pub fn intersection_in_place_derive_method(input: TokenStream) -> TokenStream {
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
                    ::friperms::IntersectionInPlace::intersection_in_place(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    ::friperms::IntersectionInPlace::intersection_in_place(&mut self.#i, &rhs.#i);
                }
            })
            .collect(),
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
    };

    let function_body = function_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl ::friperms::IntersectionInPlace<&#struct_name> for #struct_name {
            fn intersection_in_place(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(DisjunctiveUnionInPlace)]
pub fn disjunctive_union_in_place_derive_method(input: TokenStream) -> TokenStream {
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
                    ::friperms::DisjunctiveUnionInPlace::disjunctive_union_in_place(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    ::friperms::DisjunctiveUnionInPlace::disjunctive_union_in_place(&mut self.#i, &rhs.#i);
                }
            })
            .collect(),
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
    };

    let function_body = function_body
        .into_iter()
        .reduce(|acc, value| quote! { #acc #value})
        .expect("No unit structs means there must be atleast 1 field.");

    quote! {
        impl ::friperms::DisjunctiveUnionInPlace<&#struct_name> for #struct_name {
            fn disjunctive_union_in_place(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}
