//! Winit [`Runner`]s functionality.

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

/// Winit [`Runner`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_winit::prelude::*;
/// #
/// App::builder()
///     .with_runner(WinitRunner::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct WinitRunner {}

impl Runner for WinitRunner {
    #[allow(unused_variables)]
    fn run(&mut self, app: App) {}
}
