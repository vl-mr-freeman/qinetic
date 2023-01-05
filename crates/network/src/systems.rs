//! Network [`System`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Network [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_window::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(NetworkStageGroup::default())
///     .with_system(NetworkStage::default(), NetworkSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct NetworkSystem {}

impl System for NetworkSystem {
    type Data = ();

    #[allow(unused_variables)]
    fn run(&mut self, data: Self::Data) {}
}
