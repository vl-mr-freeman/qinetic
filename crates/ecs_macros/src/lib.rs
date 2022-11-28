//! Entity-component-system macros for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

extern crate proc_macro;

mod component;
mod resource;

use proc_macro::TokenStream;
use qinetic_utils_macros::manifest::Manifest;
use syn::Path;

#[proc_macro_derive(Component)]
/// Generates an impl for [`Component`] trait.
pub fn derive_component(input: TokenStream) -> TokenStream {
    component::derive(input)
}

#[proc_macro_derive(Resource)]
/// Generates an impl for [`Resource`] trait.
pub fn derive_resource(input: TokenStream) -> TokenStream {
    resource::derive(input)
}

pub(crate) fn ecs_path() -> Path {
    Manifest::get_path("qinetic_ecs")
}
