extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, parse_quote};

mod comparisons;
mod operations;

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
        syn::Fields::Unit => {
            return quote! {
              compile_error!("Unit structs can't be a set.")
            }
            .into();
        }
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
        syn::Fields::Unit => unreachable!("Already returned error earlier."),
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
    let trait_path: syn::Path = parse_quote!(::finit::operations::UnionAssign);
    let fn_name = format_ident!("union_assign");
    operations::operation_assign_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(DifferenceAssign)]
pub fn difference_assign_derive(input: TokenStream) -> TokenStream {
    let trait_path: syn::Path = parse_quote!(::finit::operations::DifferenceAssign);
    let fn_name = format_ident!("difference_assign");
    operations::operation_assign_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(IntersectionAssign)]
pub fn intersection_assign_derive(input: TokenStream) -> TokenStream {
    let trait_path: syn::Path = parse_quote!(::finit::operations::IntersectionAssign);
    let fn_name = format_ident!("intersection_assign");
    operations::operation_assign_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(DisjunctiveUnionAssign)]
pub fn disjunctive_union_assign_derive(input: TokenStream) -> TokenStream {
    let trait_path: syn::Path = parse_quote!(::finit::operations::DisjunctiveUnionAssign);
    let fn_name = format_ident!("disjunctive_union_assign");
    operations::operation_assign_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(Union)]
pub fn union_derive(input: TokenStream) -> TokenStream {
    let trait_path: syn::Path = parse_quote!(::finit::operations::Union);
    let fn_name = format_ident!("union");
    operations::operation_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(Difference)]
pub fn difference_derive(input: TokenStream) -> TokenStream {
    let trait_path: syn::Path = parse_quote!(::finit::operations::Difference);
    let fn_name = format_ident!("difference");
    operations::operation_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(Intersection)]
pub fn intersection_derive(input: TokenStream) -> TokenStream {
    let trait_path: syn::Path = parse_quote!(::finit::operations::Intersection);
    let fn_name = format_ident!("intersection");
    operations::operation_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(DisjunctiveUnion)]
pub fn disjunctive_union_derive(input: TokenStream) -> TokenStream {
    let trait_path: syn::Path = parse_quote!(::finit::operations::DisjunctiveUnion);
    let fn_name = format_ident!("disjunctive_union");
    operations::operation_derive(input, &trait_path, &fn_name)
}

#[proc_macro_derive(SetEq)]
pub fn set_eq_derive(input: TokenStream) -> TokenStream {
    comparisons::set_eq_derive(input)
}

#[proc_macro_derive(SubsetOf)]
pub fn subset_of_derive(input: TokenStream) -> TokenStream {
    comparisons::subset_of_derive(input)
}
