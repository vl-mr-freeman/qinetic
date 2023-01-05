//! `Stage` derive.

use proc_macro::TokenStream;
use quote::format_ident;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut trait_path = crate::path();
    trait_path.segments.push(format_ident!("stage").into());
    trait_path.segments.push(format_ident!("StageLabel").into());
    qinetic_utils::label::derive_label(input, &trait_path, "stage_label")
}
