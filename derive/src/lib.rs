extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, parse_quote};

#[proc_macro_derive(Set)]
pub fn set_derive(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let input: DeriveInput = syn::parse2(input).unwrap();

    let crate_name: syn::Path = parse_quote!(::finit);
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
                    #crate_name::Set::is_empty(&self.#field_name)
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::Set::is_empty(&self.#i)
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
                    #field_name: <#field_type as #crate_name::Set>::empty(),
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::Set::is_empty(&self.#i)
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
        impl #crate_name::Set for #struct_name {
            type Empty = Self;

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

#[proc_macro_derive(UnionAssign)]
pub fn union_assign_derive(input: TokenStream) -> TokenStream {
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
                    #crate_name::operations::UnionAssign::union_assign(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::operations::UnionAssign::union_assign(&mut self.#i, &rhs.#i);
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
        impl #crate_name::operations::UnionAssign<&#struct_name> for #struct_name {
            fn union_assign(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(DifferenceAssign)]
pub fn difference_assign_derive(input: TokenStream) -> TokenStream {
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
                    #crate_name::operations::DifferenceAssign::difference_assign(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::operations::DifferenceAssign::difference_assign(&mut self.#i, &rhs.#i);
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
        impl #crate_name::operations::DifferenceAssign<&#struct_name> for #struct_name {
            fn difference_assign(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(IntersectionAssign)]
pub fn intersection_assign_derive(input: TokenStream) -> TokenStream {
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
                    #crate_name::operations::IntersectionAssign::intersection_assign(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::operations::IntersectionAssign::intersection_assign(&mut self.#i, &rhs.#i);
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
        impl #crate_name::operations::IntersectionAssign<&#struct_name> for #struct_name {
            fn intersection_assign(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(DisjunctiveUnionAssign)]
pub fn disjunctive_union_assign_derive(input: TokenStream) -> TokenStream {
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
                    #crate_name::operations::DisjunctiveUnionAssign::disjunctive_union_assign(&mut self.#field_name, &rhs.#field_name);
                }
            })
            .collect(),
        syn::Fields::Unnamed(fields) => fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _field)| {
                quote! {
                    #crate_name::operations::DisjunctiveUnionAssign::disjunctive_union_assign(&mut self.#i, &rhs.#i);
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
        impl #crate_name::operations::DisjunctiveUnionAssign<&#struct_name> for #struct_name {
            fn disjunctive_union_assign(&mut self, rhs: &#struct_name) {
                #function_body
            }
        }
    }
    .into()
}

#[proc_macro_derive(SetEq)]
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
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
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

#[proc_macro_derive(SubsetOf)]
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
        syn::Fields::Unit => panic!("Unit structs can't be a set."),
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
