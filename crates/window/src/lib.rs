//! Window functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main window functionality.

    #[doc(hidden)]
    pub use crate::{
        WindowMode, WindowMonitor, WindowPlugin, WindowPosition, WindowResource,
        WindowSizeConstraints, WindowStage,
    };
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;

/// Window functionality for [`App`]
///
/// [`Stage`]s:
/// * [`WindowStage`]
///
/// [`Resource`]s:
/// * [`WindowResource`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder().with_plugin(WindowPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct WindowPlugin {}

impl Plugin for WindowPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(WindowResource::default());
        app_builder.with_stage(WindowStage::default());
    }
}

/// Window [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window:prelude::*;
/// #
/// App::builder().with_stage(WindowStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct WindowStage {}

/// Window [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_resource(WindowResource::default())
///     .build()
///     .run();
/// ```
#[derive(Resource)]
pub struct WindowResource {
    /// The requested logical width of the `window`'s client area.
    pub width: u32,

    /// The requested logical height of the `window`'s client area.
    pub height: u32,

    /// The requested size constraints of the `window`.
    pub size_constraints: WindowSizeConstraints,

    /// The requested position of the `window`.
    pub position: WindowPosition,

    /// The requested monitor to place the `window`.
    pub monitor: WindowMonitor,

    /// The requested title of the `window`.
    pub title: String,

    /// The requested resizable possibility of the `window`.
    pub resizable: bool,

    /// The requested decorations of the `window`'s clent area.
    pub decorations: bool,

    /// The requested mode of the `window`.
    pub mode: WindowMode,
}

impl Default for WindowResource {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            size_constraints: WindowSizeConstraints::default(),
            position: WindowPosition::default(),
            monitor: WindowMonitor::default(),
            title: "Qinetic App".to_string(),
            resizable: false,
            decorations: true,
            mode: Default::default(),
        }
    }
}

/// Defines on which `mode` show `window` on creation.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_resource(WindowResource {
///         mode: WindowMode::Fullscreen,
///         ..Default::default()
///     })
///     .build()
///     .run();
/// ```
#[derive(Default)]
pub enum WindowMode {
    /// Show `window` on auto mode.
    #[default]
    Automatic,

    /// Show `window` on minimize mode.
    Minimized,

    /// Show `window` on maximize mode.
    Maximized,

    /// Show `window` on fullscreen mode.
    Fullscreen,
}

/// Defines on which `position` show `window` on creation.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_resource(WindowResource {
///         position: WindowPosition::At {x: 128, y: 128},
///         ..Default::default()
///     })
///     .build()
///     .run();
/// ```
#[derive(Default)]
pub enum WindowPosition {
    /// Show `window` on auto position.
    #[default]
    Automatic,

    /// Show `window` on center position.
    Center,

    /// Show `window` on specified position.
    At { x: i32, y: i32 },
}

/// Defines on which `monitor` show `window` on creation.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_resource(WindowResource {
///         monitor: WindowMonitor::Index(2),
///         ..Default::default()
///     })
///     .build()
///     .run();
/// ```
#[derive(Default)]
pub enum WindowMonitor {
    /// Show `window` on current monitor.
    #[default]
    Current,

    /// Show `window` on primary monitor.
    Primary,

    /// Show `window` on specified monitor.
    Index(usize),
}

/// Constraints for `window` size.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
///`App::builder()
///     .with_resource(WindowResource {
///         size_constraints: WindowSizeConstraints {
///             min_width: Some(1280),
///             min_height: Some(720),
///             max_width: None,
///             max_height: None,
///         },
///         ..Default::default()
///     })
///     .build()
///     .run();
/// ```
#[derive(Default)]
pub struct WindowSizeConstraints {
    /// Minimum logical width of the `window`'s client area.
    pub min_width: Option<u32>,

    /// Minimum logical height of the `window`'s client area.
    pub min_height: Option<u32>,

    /// Maximum logical width of the `window`'s client area.
    pub max_width: Option<u32>,

    /// Maximum logical height of the `window`'s client area.
    pub max_height: Option<u32>,
}
