use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
    let qinetic_app_path = crate::app_path();

    ast.generics
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: 'static });

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
        impl #impl_generics #qinetic_app_path::resource::Resource for #struct_name #type_generics #where_clause {
        }
    })
}
