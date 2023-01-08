//! Network [`Plugin`]s functionality.

use crate::{
    stages::{NetworkStage, NetworkStageGroup},
    systems::NetworkSystem,
};

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Network [`Plugin`]
///
/// [`Stage`]s:
/// * [`NetworkStageGroup`]
///
/// [`System`]s:
/// * [`NetworkSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_network::prelude::*;
/// #
/// App::builder()
///     .with_plugin(NetworkPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct NetworkPlugin {}

impl Plugin for NetworkPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_stage_group(NetworkStageGroup::default())
            .with_system(NetworkStage::default(), NetworkSystem::default());
    }
}
