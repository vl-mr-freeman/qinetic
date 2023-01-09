//! [![](https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_logo.svg)](https://github.com/vl-mr-freeman/qinetic)
//!
//! Application macros for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

extern crate proc_macro;

mod stage_label;

use proc_macro::TokenStream;
use qinetic_utils::manifest::Manifest;
use syn::Path;

/// Generates an impl for [`StageLabel`] trait.
#[proc_macro_derive(StageLabel)]
pub fn derive_stage(input: TokenStream) -> TokenStream { stage_label::derive(input) }

/// Returns a [`Path`] of `qinetic_app`.
pub(crate) fn path() -> Path { Manifest::get_path("qinetic_app") }
