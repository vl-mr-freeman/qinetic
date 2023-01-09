//! Input [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

pub use crate::keyboard::KeyboardSystem;

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
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct InputSystem {}

impl System for InputSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
