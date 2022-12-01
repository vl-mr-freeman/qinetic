//! Application runner functionality.

use crate::app::App;
use std::any::Any;

/// Runner that calls in [App::run].
///
/// # Examples
/// ```
/// # use qinetic_qpp::prelude::*;
/// #
/// struct MyRunner;
///
/// impl Runner for MyRunner {
///     fn run(&mut self, app: App) {/* something to do */}
/// }
/// ```
pub trait Runner: Any + 'static {
    fn run(&mut self, app: App);
}
