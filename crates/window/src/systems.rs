//! Window [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Window [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(WindowStageGroup::default())
///     .with_system(WindowStage::Update, WindowSystem, WindowSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct WindowSystem {}

impl System for WindowSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
