//! Physically based render [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Physically based render [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_pbr::prelude::*;
/// #
/// App::builder()
///     .with_resource(PbrResource::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Debug, Getters, Builder, Resource)]
#[getset(get = "pub")]
#[builder(
    crate = "crate::resources",
    setter(prefix = "with"),
    default,
    derive(Debug, PartialEq, Eq)
)]
pub struct PbrResource {}

impl PbrResource {
    /// Returns a [`PbrResourceBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_pbr::prelude::*;
    /// #
    /// let pbr_resource_builder = PbrResource::builder();
    /// ```
    #[inline]
    pub fn builder() -> PbrResourceBuilder {
        PbrResourceBuilder::default()
    }
}
