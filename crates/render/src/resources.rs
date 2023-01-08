//! Render [`Resource`]s functionality.

use qinetic_ecs::prelude::*;
use qinetic_utils::prelude::*;

/// Render [`Resource`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_resource(RenderResource::default())
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
pub struct RenderResource {
    /// The requested `internal api`.
    pub api: RenderApi,

    /// The requested count of concurrently processed frames.
    #[default = 2]
    pub max_frames_in_flight: u32,

    /// The requested background `clear color`.
    #[default(RGBA::new(0, 0, 0, 255))]
    pub clear_color: RGBA8,
}

impl RenderResource {
    /// Returns a [`RenderResourceBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_render::prelude::*;
    /// #
    /// let render_resource_builder = RenderResource::builder();
    /// ```
    #[inline]
    pub fn builder() -> RenderResourceBuilder {
        RenderResourceBuilder::default()
    }
}

/// Defines which `internal api` use.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_resource(
///         RenderResource::builder()
///             .with_api(RenderApi::default())
///             .build()
///             .unwrap(),
///     )
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault, Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderApi {
    /// Platform specific `api`.
    #[default]
    Automatic,

    /// Vulkan `api`.
    #[cfg(feature = "vulkan")]
    Vulkan,
}
