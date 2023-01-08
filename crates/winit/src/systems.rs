//! Winit [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Winit [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_winit::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(WinitStageGroup::default())
///     .with_system(WinitStage::default(), WinitSystem::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct WinitSystem {}

impl System for WinitSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
