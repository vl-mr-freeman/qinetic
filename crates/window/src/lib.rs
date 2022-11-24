//! Window functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main window functionality.
    #[doc(hidden)]
    pub use crate::{WindowPlugin, WindowResource, WindowStage, WindowStyle};
}

use qinetic_app::prelude::*;

/// Adds window functionality to [`App`]
#[derive(Default)]
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(WindowResource::default());
        app_builder.with_stage(WindowStage::default());
    }
}

#[derive(Resource)]
pub struct WindowResource {
    /// The requested logical width of the window's client area.
    pub width: u32,

    /// The requested logical height of the window's client area.
    pub height: u32,

    /// The requested title of the window.
    pub title: String,

    /// The requested resizable possibility of the window.
    pub resizable: bool,

    /// The requested decorations of the window's clent area.
    pub decorations: bool,

    /// The requested style of the window.
    pub style: WindowStyle,
}

impl Default for WindowResource {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: "Qinetic App".to_string(),
            resizable: false,
            decorations: true,
            style: Default::default(),
        }
    }
}

#[derive(Default)]
pub enum WindowStyle {
    #[default]
    Default,
    Maximized,
    Minimized,
}

/// [`App`]'s window step of execution cycle.
#[derive(Default)]
pub struct WindowStage;

impl Stage for WindowStage {}
