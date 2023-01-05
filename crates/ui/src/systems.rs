//! User-interface [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// User-interface [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ui::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(UiStageGroup::default())
///     .with_system(UiStage::default(), UiSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct UiSystem {}

impl System for UiSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
