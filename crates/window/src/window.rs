//! Window creating functionality.

use crate::event_loop::EventLoop;

/// Represent a `window`.
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

/// A `Builder pattern` for [`Window`].
pub struct WindowBuilder {}

impl WindowBuilder {
    /// Returns a [`WindowBuilder`] with set a `title`.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_window::prelude::*;
    /// #
    /// Window::builder()
    ///     .with_title("Qinetic")
    ///     .build()
    ///     .run();
    /// ```
    pub fn with_title<T: Into<String>>(&mut self, title: T) -> &mut Self {
        todo!()
    }

    /// Returns a [`Window`] configured from [`WindowBuilder`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_window::prelude::*;
    /// #
    /// Window::builder().build();
    /// ```
    pub fn build(&mut self, event_loop: &EventLoop) -> Window {
        Window {}
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {}
    }
}
