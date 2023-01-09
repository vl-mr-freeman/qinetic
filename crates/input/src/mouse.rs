//! Mouse functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::button::ButtonState;

/// Mouse [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(InputStageGroup::default())
///     .with_system(InputStage::default(), MouseSystem::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct MouseSystem {}

impl System for MouseSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}

/// Mouse button [`Event`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_event(
///         MouseButtonEvent::builder()
///             .with_button(MouseButton::Left)
///             .with_state(ButtonState::Pressed)
///             .build()
///             .unwrap(),
///     )
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Copy, Debug, Derivative, CopyGetters, Builder, Event)]
#[derivative(PartialEq, Eq)]
#[getset(get_copy = "pub")]
#[builder(
    crate = "crate::mouse",
    setter(prefix = "with"),
    derive(Debug, PartialEq, Eq)
)]
pub struct MouseButtonEvent {
    button: MouseButton,

    #[derivative(PartialEq = "ignore")]
    state: ButtonState,
}

impl MouseButtonEvent {
    /// Returns a [`MouseButtonEventBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_input::prelude::*;
    /// #
    /// let mouse_button_event_builder = MouseButtonEvent::builder();
    /// ```
    #[inline]
    pub fn builder() -> MouseButtonEventBuilder { MouseButtonEventBuilder::default() }
}

/// The `button` of a [`MouseEvent`].
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MouseButton {
    /// The `left` mouse button.
    Left,

    /// The `right` mouse button.
    Right,

    /// The `middle` mouse button.
    Middle,

    /// Another associated number mouse button.
    Other(u8),
}

/// Mouse motion [`Event`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_event(
///         MouseMotionEvent::builder()
///             .with_position((0.0, 0.0))
///             .with_delta((-10.0, -10.0))
///             .build()
///             .unwrap(),
///     )
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Copy, Debug, CopyGetters, Builder, Derivative, Event)]
#[derivative(PartialEq, PartialOrd)]
#[getset(get_copy = "pub")]
#[builder(
    crate = "crate::mouse",
    setter(prefix = "with"),
    derive(Debug, PartialEq)
)]
pub struct MouseMotionEvent {
    position: (f32, f32),

    #[derivative(PartialEq = "ignore", PartialOrd = "ignore")]
    delta: (f32, f32),
}

impl MouseMotionEvent {
    /// Returns a [`MouseMotionEventBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_input::prelude::*;
    /// #
    /// let mouse_motion_event_builder = MouseMotionEvent::builder();
    /// ```
    #[inline]
    pub fn builder() -> MouseMotionEventBuilder { MouseMotionEventBuilder::default() }
}

/// Mouse wheel [`Event`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_event(
///         MouseWheelEvent::builder()
///             .with_scroll_x(1.0)
///             .build()
///             .unwrap(),
///     )
///     .build()
///     .unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, CopyGetters, Builder, Event)]
#[getset(get_copy = "pub")]
#[builder(
    crate = "crate::mouse",
    setter(prefix = "with"),
    derive(Debug, PartialEq, PartialOrd)
)]
pub struct MouseWheelEvent {
    #[builder(setter(strip_option), default)]
    scroll_x: Option<f32>,

    #[builder(setter(strip_option), default)]
    scroll_y: Option<f32>,
}

impl MouseWheelEvent {
    /// Returns a [`MouseWheelEventBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_input::prelude::*;
    /// #
    /// let mouse_wheel_event_builder = MouseWheelEvent::builder();
    /// ```
    #[inline]
    pub fn builder() -> MouseWheelEventBuilder { MouseWheelEventBuilder::default() }
}
