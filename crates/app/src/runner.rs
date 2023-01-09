//! Application runner functionality.

use std::any::Any;

use qinetic_utils::prelude::*;

use crate::app::App;

/// Runner that calls in [App::run].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// #[derive(Clone)]
/// struct MyRunner;
///
/// impl Runner for MyRunner {
///     fn run(&mut self, mut app: App) {
///         // Something to do
///     }
/// }
/// ```
pub trait Runner: DynClone + Any + 'static {
    fn run(&mut self, app: App);
}

clone_trait_object!(Runner);
