//! Default runners

use qinetic_app::plugin::*;
use qinetic_utils::prelude::*;

/// Minimal [`PluginGroup`].
///
/// Includes:
/// * [`CorePlugin`](../qinetic_core/prelude/struct.CorePlugin.html)
///
/// See also [`DefaultPluginGroup`] for a more complete [`PluginGroup`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_internal::prelude::*;
/// #
/// App::builder()
///     .with_plugin_group(MinimalPluginGroup::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault)]
pub struct MinimalPluginGroup {}

impl PluginGroup for MinimalPluginGroup {
    fn configure(&mut self, registry: &mut PluginRegistry) {
        registry.add_plugin(qinetic_core::prelude::CorePlugin::default());
    }
}

/// Default [`PluginGroup`].
///
/// Includes:
/// * [`CorePlugin`](../qinetic_core/prelude/struct.CorePlugin.html)
/// * [`LogPlugin`](../qinetic_log/prelude/struct.LogPlugin.html)
/// * [`AssetPlugin`](../qinetic_asset/prelude/struct.AssetPlugin.html)
/// * [`NetworkPlugin`](../qinetic_network/prelude/struct.NetworkPlugin.html) - feature = `network`
/// * [`AiPlugin`](../qinetic_ai/prelude/struct.Ailugin.html) - feature = `ai`
/// * [`AnimationPlugin`](../qinetic_animation/prelude/struct.AnimationPlugin.html) - feature = `animation`
/// * [`AudioPlugin`](../qinetic_audio/prelude/struct.AudioPlugin.html) - feature = `audio`
/// * [`PhysicsPlugin`](../qinetic_physics/prelude/struct.PhysicsPlugin.html) - feature = `physics`
/// * [`WindowPlugin`](../qinetic_window/prelude/struct.WindowPlugin.html)
/// * [`InputPlugin`](../qinetic_input/prelude/struct.InputPlugin.html)
/// * [`RenderPlugin`](../qinetic_render/prelude/struct.RenderPlugin.html) - feature = `render`
/// * [`UiPlugin`](../qinetic_ui/prelude/struct.UiPlugin.html) - feature = `ui`
/// * [`VrPlugin`](../qinetic_vr/prelude/struct.VrPlugin.html) - feature = `vr`
/// * [`ArPlugin`](../qinetic_ar/prelude/struct.ArPlugin.html) - feature = `ar`
///
/// See also [`MinimalPluginGroup`] for a slimmed down [`PluginGroup`].
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_internal::prelude::*;
/// #
/// App::builder()
///     .with_plugin_group(DefaultPluginGroup::default())
///     .build()
///     .unwrap();
/// ```
#[derive(SmartDefault)]
pub struct DefaultPluginGroup {}

impl PluginGroup for DefaultPluginGroup {
    fn configure(&mut self, registry: &mut PluginRegistry) {
        registry
            .add_plugin(qinetic_core::prelude::CorePlugin::default())
            .add_plugin(qinetic_asset::prelude::AssetPlugin::default());

        #[cfg(feature = "qinetic_log")]
        registry.add_plugin(qinetic_log::prelude::LogPlugin::default());

        #[cfg(feature = "qinetic_network")]
        registry.add_plugin(qinetic_network::prelude::NetworkPlugin::default());

        #[cfg(feature = "qinetic_ai")]
        registry.add_plugin(qinetic_ai::prelude::AiPlugin::default());

        #[cfg(feature = "qinetic_animation")]
        registry.add_plugin(qinetic_animation::prelude::AnimationPlugin::default());

        #[cfg(feature = "qinetic_audio")]
        registry.add_plugin(qinetic_audio::prelude::AudioPlugin::default());

        #[cfg(feature = "qinetic_physics")]
        registry.add_plugin(qinetic_physics::prelude::PhysicsPlugin::default());

        #[cfg(feature = "qinetic_window")]
        registry.add_plugin(qinetic_window::prelude::WindowPlugin::default());

        #[cfg(feature = "qinetic_winit")]
        registry.add_plugin(qinetic_winit::prelude::WinitPlugin::default());

        #[cfg(feature = "qinetic_input")]
        registry.add_plugin(qinetic_input::prelude::InputPlugin::default());

        #[cfg(feature = "qinetic_render")]
        registry.add_plugin(qinetic_render::prelude::RenderPlugin::default());

        #[cfg(feature = "qinetic_pbr")]
        registry.add_plugin(qinetic_pbr::prelude::PbrPlugin::default());

        #[cfg(feature = "qinetic_ui")]
        registry.add_plugin(qinetic_ui::prelude::UiPlugin::default());

        #[cfg(feature = "qinetic_vr")]
        registry.add_plugin(qinetic_vr::prelude::VrPlugin::default());

        #[cfg(feature = "qinetic_ar")]
        registry.add_plugin(qinetic_ar::prelude::ArPlugin::default());
    }
}
