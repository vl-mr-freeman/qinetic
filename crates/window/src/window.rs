//! Window creating functionality.

use qinetic_utils::prelude::*;

/// Cross-platform window representation.
#[derive(SmartDefault, Clone, Debug, PartialEq, Eq, Builder, Getters)]
#[getset(get = "pub")]
#[builder(setter(prefix = "with"), default, derive(Debug, PartialEq, Eq))]
pub struct Window {}

impl Window {
    /// Returns a [`WindowBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_window::prelude::*;
    /// #
    /// let window_builder = Window::builder();
    /// ```
    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }
}
