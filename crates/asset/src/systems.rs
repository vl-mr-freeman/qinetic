//! Asset [`System`]s functionality.

use qinetic_ecs::prelude::*;

/// Asset [`System`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_asset::prelude::*;
/// #
/// App::builder()
///     .with_stage_group(AssetStageGroup::default())
///     .with_system(AssetStage::default(), AssetSystem::default())
///     .build()
///     .unwrap()
///     .run();
/// ```
#[derive(Default, Clone, Debug)]
pub struct AssetSystem {}

impl System for AssetSystem {
    type Data = ();

    fn run(&mut self, data: Self::Data) {}
}
