//! Log [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;
use std::clone::Clone;
use std::default::Default;
use std::fs::File;
use std::option::Option;

#[derive(Add)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

/// Log [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_log::prelude::*;
/// #
/// App::builder()
///     .with_resource(LogResource::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Debug, Getters, Builder, Resource)]
#[getset(get = "pub")]
#[builder(
    crate = "crate::resources",
    setter(prefix = "with"),
    default,
    derive(Debug)
)]
pub struct LogResource {
    /// Minimum log level, that print.
    #[default(LogLevel::Info)]
    pub min_level: LogLevel,

    /// Maximum log level, that print.
    #[default(LogLevel::Fatal)]
    pub max_level: LogLevel,

    /// File to log.
    #[builder(setter(skip))]
    pub file: Option<File>,
}

impl LogResource {
    /// Returns a [`LogResourceBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_log::prelude::*;
    /// #
    /// let log_resource_builder = LogResource::builder();
    /// ```
    #[inline]
    pub fn builder() -> LogResourceBuilder {
        LogResourceBuilder::default()
    }
}

/// Level of logging.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_log::prelude::*;
/// #
/// App::builder()
///     .with_resource(
///         LogResource::builder()
///             .with_min_level(LogLevel::Info)
///             .with_max_level(LogLevel::Fatal)
///             .build()
///             .unwrap(),
///     )
///     .build()
///     .unwrap()
///     .run()
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum LogLevel {
    /// The minimal [`LogLevel`] for `info` logging.
    #[default]
    Info = 0,

    /// The [`LogLevel`] for `debug` logging.
    Debug = 1,

    /// The [`LogLevel`] for `notice` logging.
    Notice = 2,

    /// The [`LogLevel`] for `trace` logging.
    Trace = 3,

    /// The [`LogLevel`] for `warning` logging.
    Warning = 4,

    /// The [`LogLevel`] for `error` logging.
    Error = 5,

    /// The maximal [`LogLevel`] for `fatal error` logging.
    Fatal = 6,
}
