//! Button functionality.

use qinetic_utils::prelude::*;

/// `Press` state of the `button`.
#[derive(SmartDefault, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ButtonState {
    /// The `button` is `pressed`.
    Pressed,

    /// The `button` is `released`.
    #[default]
    Released,
}

impl ButtonState {
    /// Returns `true` if `button` is `pressed`.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_input::prelude::*;
    /// let state = ButtonState::Pressed;
    /// assert!(state.is_pressed());
    /// ```
    #[inline(always)]
    pub const fn is_pressed(&self) -> bool { matches!(self, ButtonState::Pressed) }

    /// Returns `true` if `button` is `released`.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_input::prelude::*;
    /// let state = ButtonState::Released;
    /// assert!(state.is_released());
    /// ```
    #[inline(always)]
    pub const fn is_released(&self) -> bool { matches!(self, ButtonState::Released) }
}
