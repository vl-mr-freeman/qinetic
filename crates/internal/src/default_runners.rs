//! Default [`Runner`]s.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

/// [`Runner`] for [`App`] that calls once.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_internal::prelude::*;
/// #
/// App::builder()
///     .with_runner(RunOnce::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct RunOnce {}

impl Runner for RunOnce {
    fn run(&mut self, mut app: App) {
        app.update();
    }
}

/// [`Runner`] for [`App`] that calls in loop.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_internal::prelude::*;
/// #
/// App::builder()
///     .with_runner(RunLoop::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct RunLoop {}

impl Runner for RunLoop {
    fn run(&mut self, mut app: App) {
        loop {
            app.update();
        }
    }
}
