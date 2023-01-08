//! Input [`System`]s functionality.

pub use crate::keyboard::KeyboardSystem;
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

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
