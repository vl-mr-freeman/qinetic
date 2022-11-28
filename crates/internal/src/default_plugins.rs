use qinetic_app::plugin::*;

/// Default [`PluginGroup`].
/// * [`CorePlugin`](../qinetic_core/struct.CorePlugin.html)
///
/// See also [`DefaultPlugins`] for a more complete group of [`Plugin`]s.
#[derive(Default)]
pub struct MinimalPlugins;

impl PluginGroup for MinimalPlugins {
    fn configure(&mut self, registry: &mut PluginRegistry) {
        registry.add(qinetic_core::CorePlugin::default());
    }
}

/// Minimal [`PluginGroup`].
/// * [`CorePlugin`](../qinetic_core/struct.CorePlugin.html)
/// * [`LogPlugin`](../qinetic_log/struct.LogPlugin.html)
/// * [`AssetPlugin`](../qinetic_asset/struct.AssetPlugin.html)
/// * [`NetworkPlugin`](../qinetic_network/struct.NetworkPlugin.html) - feature = `network`
/// * [`AiPlugin`](../qinetic_ai/struct.Ailugin.html) - feature = `ai`
/// * [`AnimationPlugin`](../qinetic_animation/struct.AnimationPlugin.html) - feature = `animation`
/// * [`AudioPlugin`](../qinetic_audio/struct.AudioPlugin.html) - feature = `audio`
/// * [`PhysicsPlugin`](../qinetic_physics/struct.PhysicsPlugin.html) - feature = `physics`
/// * [`WindowPlugin`](../qinetic_window/struct.WindowPlugin.html)
/// * [`InputPlugin`](../qinetic_input/struct.InputPlugin.html)
/// * [`RenderPlugin`](../qinetic_render/struct.RenderPlugin.html) - feature = `render`
/// * [`UiPlugin`](../qinetic_ui/struct.UiPlugin.html) - feature = `ui`
///
/// See also [`MinimalPlugins`] for a slimmed down group of [`Plugin`]s.
#[derive(Default)]
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn configure(&mut self, registry: &mut PluginRegistry) {
        registry.add(qinetic_core::CorePlugin::default());
        #[cfg(feature = "qinetic_log")]
        registry.add(qinetic_log::LogPlugin::default());
        registry.add(qinetic_asset::AssetPlugin::default());
        #[cfg(feature = "qinetic_network")]
        registry.add(qinetic_network::NetworkPlugin::default());
        #[cfg(feature = "qinetic_ai")]
        registry.add(qinetic_ai::AiPlugin::default());
        #[cfg(feature = "qinetic_animation")]
        registry.add(qinetic_animation::AnimationPlugin::default());
        #[cfg(feature = "qinetic_audio")]
        registry.add(qinetic_audio::AudioPlugin::default());
        #[cfg(feature = "qinetic_physics")]
        registry.add(qinetic_physics::PhysicsPlugin::default());
        registry.add(qinetic_window::WindowPlugin::default());
        registry.add(qinetic_input::InputPlugin::default());
        #[cfg(feature = "qinetic_render")]
        registry.add(qinetic_render::RenderPlugin::default());
        #[cfg(feature = "qinetic_ui")]
        registry.add(qinetic_ui::UiPlugin::default());
    }
}
