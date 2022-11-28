//! User-interface functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main user-interface functionality.
    #[doc(hidden)]
    pub use crate::{font::*, theme::*, UiPlugin, UiResource, UiStage};
}

/// Provides font facilitate loading.
pub mod font;

/// Provides theme facilitate creating.
pub mod theme;

use crate::font::*;
use crate::theme::*;
use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Adds user-interface functionality to [`App`]
#[derive(Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(UiResource::default());
        app_builder.with_stage(UiStage::default());
    }
}

#[derive(Resource)]
pub struct UiResource {
    /// The requested user-interface client opacity.
    pub opacity: f32,

    /// The requested user-interface client theme.
    pub theme: Theme,

    /// The requested user-interface client font.
    pub font: Font,

    /// The requested user-interface client window padding.
    pub window_padding: (f32, f32),

    /// The requested user-interface client window rounding.
    pub window_rounding: f32,

    /// The requested user-interface client window border size.
    pub window_border_size: f32,

    /// The requested user-interface client window's child padding.
    pub child_window_padding: (f32, f32),

    /// The requested user-interface client window's child rounding.
    pub child_window_rounding: f32,

    /// The requested user-interface client window's child border size.
    pub child_window_border_size: f32,

    /// The requested user-interface client frame padding.
    pub frame_padding: (f32, f32),

    /// The requested user-interface client frame rounding.
    pub frame_rounding: f32,

    /// The requested user-interface client frame border size.
    pub frame_border_size: f32,

    /// The requested user-interface client inner window items spacing.
    pub item_spacing: (f32, f32),

    /// The requested user-interface client inner window items inner spacing.
    pub item_inner_spacing: (f32, f32),

    /// The requested user-interface client scrollbar thickness.
    pub scrollbar_thickness: f32,

    /// The requested user-interface client scrollbar rounding.
    pub scrollbar_rounding: f32,

    /// The requested user-interface client grab size.
    pub grab_size: f32,

    /// The requested user-interface client grab rounding.
    pub grab_rounding: f32,
}

impl Default for UiResource {
    fn default() -> Self {
        Self {
            opacity: 1.0,

            theme: Theme::default(),

            font: Font::default(),

            window_padding: (10.0, 10.0),
            window_rounding: 5.0,
            window_border_size: 1.0,

            child_window_padding: (15.0, 15.0),
            child_window_rounding: 5.0,
            child_window_border_size: 1.0,

            frame_padding: (5.0, 2.5),
            frame_rounding: 2.5,
            frame_border_size: 0.0,

            item_spacing: (5.0, 5.0),
            item_inner_spacing: (5.0, 5.0),

            scrollbar_thickness: 12.5,
            scrollbar_rounding: 5.0,

            grab_size: 10.0,
            grab_rounding: 2.5,
        }
    }
}

/// [`App`]'s user-interface step of execution cycle.
#[derive(Default, Stage)]
pub struct UiStage;
