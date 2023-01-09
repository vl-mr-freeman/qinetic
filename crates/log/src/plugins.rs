//! Log [`Plugin`]s functionality.

use qinetic_app::prelude::*;
#[allow(unused_imports)]
use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

use crate::resources::LogResource;

/// Log [`Plugin`].
///
/// [`Resource`]s:
/// * [`LogResource`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_log::prelude::*;
/// #
/// App::builder()
///     .with_plugin(LogPlugin::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug)]
pub struct LogPlugin {}

impl Plugin for LogPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(LogResource::default());
    }
}
