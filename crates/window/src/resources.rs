//! Window [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

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
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug, Getters, Builder, Resource)]
#[getset(get = "pub")]
#[builder(setter(prefix = "with"), default, derive(Debug, PartialEq, Eq))]
pub struct WindowResource {
    /// The logical width of the `window`'s client area.
    #[default = 1280]
    width: u32,

    /// The logical height of the `window`'s client area.
    #[default = 720]
    height: u32,

    /// The size constraints of the `window`.
    size_constraints: WindowSizeConstraints,

    /// The position of the `window`.
    position: WindowPosition,

    /// The monitor to place the `window`.
    monitor: WindowMonitor,

    /// The title of the `window`.
    #[builder(setter(into))]
    #[default = "Qinetic App"]
    title: String,

    /// The resizable possibility of the `window`.
    resizable: bool,

    /// The decorations of the `window`'s clent area.
    #[default = true]
    decorations: bool,

    /// The mode of the [`Window`].
    mode: WindowMode,
}

impl WindowResource {
    /// Returns a [`WindowResourceBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_window::prelude::*;
    /// #
    /// let window_resource_builder = WindowResource::builder();
    /// ```
    #[inline]
    pub fn builder() -> WindowResourceBuilder {
        WindowResourceBuilder::default()
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
///     .with_resource(
///         WindowResource::builder()
///             .with_mode(WindowMode::default())
///             .build()
///             .unwrap()
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq)]
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
///     .with_resource(
///         WindowResource::builder()
///             .with_position(WindowPosition::default())
///             .build()
///             .unwrap()
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq)]
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
///     .with_resource(
///         WindowResource::builder()
///             .with_monitor(WindowMonitor::default())
///             .build()
///             .unwrap()
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq)]
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
///     .with_resource(
///         WindowResource::builder()
///             .with_size_constraints(WindowSizeConstraints::default())
///             .build()
///             .unwrap()
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Builder, Getters)]
#[getset(get = "pub")]
#[builder(setter(prefix = "with"), default, derive(Debug, PartialEq, Eq))]
pub struct WindowSizeConstraints {
    /// Minimum logical width of the `window`'s client area.
    min_width: Option<u32>,

    /// Minimum logical height of the `window`'s client area.
    min_height: Option<u32>,

    /// Maximum logical width of the `window`'s client area.
    max_width: Option<u32>,

    /// Maximum logical height of the `window`'s client area.
    max_height: Option<u32>,
}
