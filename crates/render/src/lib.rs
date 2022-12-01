//! Render functionality for Qinetic.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg",
    html_favicon_url = "https://raw.githubusercontent.com/vl-mr-freeman/qinetic/master/assets/qinetic_icon.svg"
)]

pub mod prelude {
    //! Main render functionality.

    #[doc(hidden)]
    pub use crate::{Mesh, RenderApi, RenderPlugin, RenderResource, RenderStage};
}

use qinetic_app::prelude::*;
use qinetic_core::color::Color4;
use qinetic_ecs::prelude::*;

/// Render functionality for [`App`]
///
/// [`Component`]s:
/// * [`Mesh`]
/// * [`Camera`]
///
/// [`Stage`]s:
/// * [`RenderStage`]
///
/// [`Resource`]s:
/// * [`RenderResource`]
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder().with_plugin(RenderPlugin::default()).build().run();
/// ```
#[derive(Default)]
pub struct RenderPlugin {}

impl Plugin for RenderPlugin {
    fn build(&mut self, app_builder: &mut AppBuilder) {
        app_builder.with_resource(RenderResource::default());
        app_builder.with_component(Mesh::default());
        app_builder.with_component(Camera::default());
        app_builder.with_stage(RenderStage::default());
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
///     .with_resource(RenderResource {
///         api: RenderApi::default(),
///         ..Default::default()
///     })
///     .build()
///     .run();
/// ```
#[derive(Default)]
pub enum RenderApi {
    /// Platform specific `api`.
    #[default]
    Automatic,

    /// Vulkan `api`.
    #[cfg(feature = "vulkan")]
    Vulkan,

    /// DirectX12 `api`.
    #[cfg(feature = "directx12")]
    DirectX12,
}

/// Render [`Resource`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder()
///     .with_resource(RenderResource {
///         ..Default::default()
///     })
///     .build()
///     .run();
/// ```
#[derive(Resource)]
pub struct RenderResource {
    /// The requested `internal api`.
    pub api: RenderApi,

    /// The requested count of concurrently processed frames.
    pub max_frames_in_flight: u32,

    /// The requested background `clear color`.
    pub clear_color: Color4,
}

impl Default for RenderResource {
    fn default() -> Self {
        Self {
            api: RenderApi::default(),
            max_frames_in_flight: 2,
            clear_color: Color4::BLACK,
        }
    }
}

/// Render [`Stage`] for [`App`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder().with_stage(RenderStage::default()).build().run();
/// ```
#[derive(Default, Stage)]
pub struct RenderStage {}

/// Mesh [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder().with_component(Mesh::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Mesh {}

/// Camera [`Component`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_render::prelude::*;
/// #
/// App::builder().with_component(Camera::default()).build().run();
/// ```
#[derive(Default, Component)]
pub struct Camera {}
