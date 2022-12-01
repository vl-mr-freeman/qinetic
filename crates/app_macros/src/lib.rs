//! Application macros for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

extern crate proc_macro;

mod plugin;
mod runner;
mod stage;

use proc_macro::TokenStream;
use qinetic_utils_macros::manifest::Manifest;
use syn::Path;

/// Generates an impl for [`Plugin`] trait.
#[proc_macro_derive(Plugin)]
pub fn derive_plugin(input: TokenStream) -> TokenStream {
    plugin::derive(input)
}

/// Generates an impl for [`Runner`] trait.
#[proc_macro_derive(Runner)]
pub fn derive_runner(input: TokenStream) -> TokenStream {
    runner::derive(input)
}

/// Generates an impl for [`Stage`] trait.
#[proc_macro_derive(Stage)]
pub fn derive_stage(input: TokenStream) -> TokenStream {
    stage::derive(input)
}

/// Returns a [`Path`] of `qinetic_app`.
pub(crate) fn path() -> Path {
    Manifest::get_path("qinetic_app")
}
