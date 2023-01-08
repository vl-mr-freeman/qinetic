//! Window [`Event`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use crate::window::*;

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
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Event)]
pub enum WindowEvent {
    /// The [`Event`] that sent whenever a [`Window`] is created.
    #[default]
    Created,

    /// The [`Event`] that sent whenever a [`Window`] is destroyed.
    Destroyed,

    /// The [`Event`] that sent whenever a [`Window`] is resized.
    Resized,

    /// The [`Event`] that sent whenever a [`Window`] is focused.
    Focused,

    /// The [`Event`] that sent whenever a [`Window`] is moved.
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
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, Event)]
pub enum CursorEvent {
    /// The [`Event`] that sent whenever a cursor is moved in [`Window`].
    Moved,

    /// The [`Event`] that sent whenever a cursor is entered to [`Window`].
    #[default]
    Entered,

    /// The [`Event`] that sent whenever a cursor is leaves from [`Window`].
    Left,
}
