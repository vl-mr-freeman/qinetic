//! Entity-component-system macros for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

extern crate proc_macro;

mod component;
mod event;
mod resource;
mod state;
mod system_label;

use proc_macro::TokenStream;
use qinetic_utils::manifest::Manifest;
use syn::Path;

/// Generates an impl for [`Component`] trait.
#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    component::derive(input)
}

/// Generates an impl for [`Resource`] trait.
#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    resource::derive(input)
}

/// Generates an impl for [`Event`] trait.
#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    event::derive(input)
}

/// Generates an impl for [`State`] trait.
#[proc_macro_derive(State)]
pub fn devive_state(input: TokenStream) -> TokenStream {
    state::derive(input)
}

/// Generates an impl for [`SystemLabel`] trait.
#[proc_macro_derive(SystemLabel)]
pub fn derive_system_label(input: TokenStream) -> TokenStream {
    system_label::derive(input)
}

/// Returns a [`Path`] of `qinetic_ecs`.
pub(crate) fn path() -> Path {
    Manifest::get_path("qinetic_ecs")
}
