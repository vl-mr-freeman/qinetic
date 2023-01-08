//! Gamepad functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Gamepad [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App:builder()
///     .with_stage_group(InputStageGroup::default())
///     .with_system(InputStage::default(), GamepadSystem::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct GamepadSystem {}

impl System for GamepadSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}

/// Gamepad representaiton.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, CopyGetters, Builder)]
#[getset(get_copy = "pub")]
#[builder(
    crate = "crate::gamepad",
    setter(prefix = "with"),
    derive(Debug, PartialEq, Eq, PartialOrd, Ord)
)]
pub struct Gamepad {
    /// The `Id` of device.
    id: usize,
}

impl Gamepad {
    /// Returns a [`GamepadBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_input::prelude::*;
    /// #
    /// let gamepad_builder = Gamepad::builder();
    /// ```
    #[inline]
    pub fn builder() -> GamepadBuilder {
        GamepadBuilder::default()
    }
}

/// [`Gamepad`] [`Event`].
#[derive(Clone, Copy, Debug, CopyGetters, Builder, Event)]
#[getset(get_copy = "pub")]
#[builder(
    crate = "crate::gamepad",
    setter(prefix = "with"),
    derive(Debug, PartialEq, Eq)
)]
pub struct GamepadEvent {
    /// The [`Gamepad`] this [`Event`] corresponds to.
    gamepad: Gamepad,

    /// The [`GamepadEventType`] this [`Event`] corresponds to.
    event_type: GamepadEventType,
}

/// [`Gamepad`] [`Event`] type.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamepadEventType {
    /// The [`Gamepad`] `connected` [`Event`].
    Connected,

    /// The [`Gamepad`] `disconnected` [`Event`].
    Disconnected,

    /// The value of [`Gamepad`] button was changed [`Event`].
    ButtonChanged,

    /// The value of [`Gamepad`] axis was changed [`Event`].
    AxisChanged,
}

/// [`Gamepad`] `button` [`Event`].
#[derive(Clone, Copy, Debug, CopyGetters, Builder, Event)]
#[getset(get_copy = "pub")]
#[builder(
    crate = "crate::gamepad",
    setter(prefix = "with"),
    derive(Debug, PartialEq, Eq)
)]
pub struct GamepadButtonEvent {
    /// The [`Gamepad`] this [`Event`] corresponds to.
    gamepad: Gamepad,

    /// The [`GamepadButtonType`] this [`Event`] corresponds to.
    button_type: GamepadButtonType,
}

/// [`Gamepad`] `button` type.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamepadButtonType {
    /// The `A` button on `Xbox`  equivalent to [GamepadButtonType::Cross].
    A,
    /// The `Cross` button on `PS` equivalent to [GamepadButtonType::A].
    Cross,

    /// The `B` button on `Xbox` equivalent to [GamepadButtonType::Circle].
    B,
    /// The `Circle` button on `PS` equivalent to [GamepadButtonType::B].
    Circle,

    /// The `Y` button on `Xbox` equivalent to [GamepadButtonType::Triangle].
    Y,
    /// The `Triangle` button on `PS` equivalent to [GamepadButtonType::Y].
    Triangle,

    /// The `X` button on `Xbox` equivalent to [GamepadButtonType::Square].
    X,
    /// The `Square` button on `PS` equivalent to [GamepadButtonType::X].
    Square,

    /// The `C` button.
    C,
    /// The `Z` button.
    Z,

    /// The `1st left trigger`.
    LeftTrigger,
    /// The `2nd left trigger`.
    LeftTrigger2,

    /// The `1st right trigger`.
    RightTrigger,
    /// The `2nd right trigger`.
    RightTrigger2,

    /// The `select` button.
    Select,
    /// The `start` button.
    Start,
    /// The `mode` button.
    Mode,

    /// The `left thumb` stick button.
    LeftThumb,
    /// The `right thumb` stick button.
    RightThumb,

    /// The `up button` of the `D-Pad`.
    DPadUp,
    /// The `down button` of the `D-Pad`.
    DPadDown,
    /// The `left button` of the `D-Pad`.
    DPadLeft,
    /// The `right button` of the `D-Pad`.
    DPadRight,

    /// Another associated number gamepad button.
    Other(u8),
}

/// [`Gamepad`] `button` [`Event`].
#[derive(Clone, Copy, Debug, CopyGetters, Builder, Event)]
#[getset(get_copy = "pub")]
#[builder(
    crate = "crate::gamepad",
    setter(prefix = "with"),
    derive(Debug, PartialEq, Eq)
)]
pub struct GamepadAxisEvent {
    /// The [`Gamepad`] this [`Event`] corresponds to.
    gamepad: Gamepad,

    /// The [`GamepadAxisType`] this [`Event`] corresponds to.
    axis_type: GamepadAxisType,
}

/// [`Gamepad`] `axis` type.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GamepadAxisType {
    /// The `horizontal` value of the `left` stick.
    LeftStickX,
    /// The `vertical` value of the `left` stick.
    LeftStickY,

    /// The `horizontal` value of the `right` stick.
    RightStickX,
    /// The `vertical` value of the `right` stick.
    RightStickY,

    /// The value of the left `Z` button.
    LeftZ,
    /// The value of the right `Z` button.
    RightZ,

    /// Another associated number gamepad axis.
    Other(u8),
}
