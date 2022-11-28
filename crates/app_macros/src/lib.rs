//! Application macros for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

extern crate proc_macro;

mod stage;

use proc_macro::TokenStream;
use qinetic_utils_macros::manifest::Manifest;
use syn::Path;

#[proc_macro_derive(Stage)]
/// Generates an impl for [`Stage`] trait.
pub fn derive_stage(input: TokenStream) -> TokenStream {
    stage::derive(input)
}

pub(crate) fn app_path() -> Path {
    Manifest::get_path("qinetic_app")
}
