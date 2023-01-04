//! Input [`System`]s functionality.

use qinetic_ecs::prelude::*;

/// Input [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_input::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(InputStageGroup::default())
///     .with_system(InputStage::default(), InputSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(Default, Clone, Debug)]
pub struct InputSystem {}

impl System for InputSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
