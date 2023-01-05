//! Application runner functionality.

use crate::app::App;
use qinetic_utils::prelude::*;
use std::any::Any;

/// Runner that calls in [App::run].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// struct MyRunner;
///
/// impl Runner for MyRunner {
///     fn run(&mut self, app: App) {/* something to do */}
/// }
/// ```
pub trait Runner: DynClone + Any + 'static {
    fn run(&mut self, app: App);
}

clone_trait_object!(Runner);
