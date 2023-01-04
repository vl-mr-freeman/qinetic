//! Math [`Plugin`]s functionality.

use crate::components::Transform;

use qinetic_app::prelude::*;
use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use qinetic_ecs::prelude::*;

/// Math [`Plugin`].
///
/// [`Component`]s:
/// * [`Transform`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_math::prelude::*;
///`#
/// App::builder()
///     .with_plugin(MathPlugin::default())
///     .build()
///     .unwrap()
///     .run();
///
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct MathPlugin {}

impl Plugin for MathPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_component(Transform::default());
    }
}
