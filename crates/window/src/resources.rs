//! Window [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use crate::window::*;

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
#[builder(
    crate = "crate::resources",
    setter(prefix = "with"),
    default,
    derive(Debug, PartialEq, Eq)
)]
pub struct WindowResource {
    /// The logical width of the [`Window`]'s client area.
    #[default = 1280]
    width: u32,

    /// The logical height of the [`Window`]'s client area.
    #[default = 720]
    height: u32,

    /// The size constraints of the [`Window`].
    size_constraints: WindowSizeConstraints,

    /// The position of the [`Window`].
    position: WindowPosition,

    /// The monitor to place the [`Window`].
    monitor: WindowMonitor,

    /// The title of the [`Window`].
    #[builder(setter(into))]
    #[default = "Qinetic App"]
    title: String,

    /// The resizable possibility of the [`Window`].
    resizable: bool,

    /// The decorations of the [`Window`]'s clent area.
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

/// Defines on which `mode` show [`Window`] on creation.
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
///             .unwrap(),
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowMode {
    /// Show [`Window`] on auto mode.
    #[default]
    Automatic,

    /// Show [`Window`] on minimize mode.
    Minimized,

    /// Show [`Window`] on maximize mode.
    Maximized,

    /// Show [`Window`] on fullscreen mode.
    Fullscreen,
}

/// Defines on which `position` show [`Window`] on creation.
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
///             .unwrap(),
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowPosition {
    /// Show [`Window`] on auto position.
    #[default]
    Automatic,

    /// Show [`Window`] on center position.
    Center,

    /// Show [`Window`] on specified position.
    At { x: i32, y: i32 },
}

/// Defines on which `monitor` show [`Window`] on creation.
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
///             .unwrap(),
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowMonitor {
    /// Show [`Window`] on current monitor.
    #[default]
    Current,

    /// Show [`Window`] on primary monitor.
    Primary,

    /// Show [`Window`] on specified monitor.
    Index(usize),
}

/// Constraints for [`Window`] size.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_resource(
///         WindowResource::builder()
///             .with_size_constraints(WindowSizeConstraints::default())
///             .build()
///             .unwrap(),
///     )
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Getters, Builder)]
#[getset(get = "pub")]
#[builder(
    crate = "crate::resources",
    setter(prefix = "with"),
    default,
    derive(Debug, PartialEq, Eq)
)]
pub struct WindowSizeConstraints {
    /// Minimum logical width of the [`Window`]'s client area.
    min_width: Option<u32>,

    /// Minimum logical height of the [`Window`]'s client area.
    min_height: Option<u32>,

    /// Maximum logical width of the [`Window`]'s client area.
    max_width: Option<u32>,

    /// Maximum logical height of the [`Window`]'s client area.
    max_height: Option<u32>,
}
