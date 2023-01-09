//! Math [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::components::Transform;

/// Math [`Plugin`].
///
/// [`Component`]s:
/// * [`Transform`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_math::prelude::*;
/// #
/// App::builder()
///     .with_plugin(MathPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct MathPlugin {}

impl Plugin for MathPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Transform::default());
    }
}
