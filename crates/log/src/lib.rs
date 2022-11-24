//! Log functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Provides main log functionality.
    #[doc(hidden)]
    pub use crate::{LogPlugin, LogResource};
}

use qinetic_app::prelude::*;

/// Adds log functionality to [`App`]
#[derive(Default)]
pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(LogResource::default());
    }
}

pub enum LogLevel {
    Fatal,
    Error,
    Warning,
    Trace,
    Notice,
    Debug,
    Info,
}

#[derive(Resource)]
pub struct LogResource {
    pub min_log_level: LogLevel,
    pub max_log_level: LogLevel,
}

impl Default for LogResource {
    fn default() -> Self {
        Self {
            min_log_level: LogLevel::Fatal,
            max_log_level: LogLevel::Info,
        }
    }
}
