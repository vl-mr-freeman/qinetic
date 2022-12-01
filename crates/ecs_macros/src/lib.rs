//! Entity-component-system macros for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

extern crate proc_macro;

mod component;
mod entity;
mod event;
mod resource;
mod state;
mod system;

use proc_macro::TokenStream;
use qinetic_utils_macros::manifest::Manifest;
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

/// Generates an impl for [`Entity`] trait.
#[proc_macro_derive(Entity)]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    entity::derive(input)
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

/// Generates an impl for [`System`] trait.
#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    system::derive(input)
}

/// Returns a [`Path`] of `qinetic_ecs`.
pub(crate) fn path() -> Path {
    Manifest::get_path("qinetic_ecs")
}
