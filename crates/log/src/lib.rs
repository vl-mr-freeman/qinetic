//! Log functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main log functionality.

    #[doc(hidden)]
    pub use crate::{LogPlugin, LogResource};
}

use qinetic_app::prelude::*;
use qinetic_ecs::prelude::*;
use std::fs::File;

/// Log [`Plugin`] for [`App`]
///
/// [`Resource`]s:
/// * [`LogResource`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ai::prelude::*;
/// #
/// App::builder().with_plugin(LogPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(LogResource::default());
    }
}

/// Level
#[repr(usize)]
pub enum LogLevel {
    Info = 0,
    Debug = 1,
    Notice = 2,
    Trace = 3,
    Warning = 4,
    Error = 5,
    Fatal = 6,
}

/// Log [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_log::prelude::*;
/// #
/// App::builder().with_resource(LogResource::default()).build().run();
/// ```
#[derive(Default, Resource)]
pub struct LogResource {
    /// Minimum log level, that print.
    pub min_level: Option<LogLevel>,

    /// Maximum log level, that print.
    pub max_level: Option<LogLevel>,

    /// File to log.
    pub file: Option<File>,
}
