//! Window [`Event`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Window [`Event`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_event(WindowEvent::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Debug, Clone, Copy, PartialEq, Eq, Event)]
pub enum WindowEvent {
    /// The [`Event`] that sent whenever a `window` is created.
    #[default]
    Created,

    /// The [`Event`] that sent whenever a `window` is destroyed.
    Destroyed,

    /// The [`Event`] that sent whenever a `window` is resized.
    Resized,

    /// The [`Event`] that sent whenever a `window` is focused.
    Focused,

    /// The [`Event`] that sent whenever a `window` is moved.
    Moved,
}

/// Cursor [`Event`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_event(CursorEvent::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Debug, Clone, Copy, PartialEq, Eq, Event)]
pub enum CursorEvent {
    /// The [`Event`] that sent whenever a cursor is moved in `window`.
    Moved,

    /// The [`Event`] that sent whenever a cursor is entered to `window`.
    #[default]
    Entered,

    /// The [`Event`] that sent whenever a cursor is leaves from `window`.
    Left,
}
