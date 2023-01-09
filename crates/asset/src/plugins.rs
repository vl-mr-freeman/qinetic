//! Asset [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::{
    resources::AssetResource,
    stages::{AssetStage, AssetStageGroup},
    systems::AssetSystem,
};

/// Asset [`Plugin`]
///
/// [`Stage`]s:
/// * [`AssetStageGroup`]
///
/// [`Resource`]s:
/// * [`AssetResource`]
///
/// [`System`]s:
/// * [`AssetSystem`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_asset::prelude::*;
/// #
/// App::builder()
///     .with_plugin(AssetPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct AssetPlugin {}

impl Plugin for AssetPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder
            .with_resource(AssetResource::default())
            .with_stage_group(AssetStageGroup::default())
            .with_system(AssetStage::default(), AssetSystem::default());
    }
}
