use qinetic_app::plugin::{PluginGroup, PluginGroupBuilder};

/// Minimal plugin group includes:
/// * [`CorePlugin`]
///
/// See also [`DefaultPluginGroup`] for a more complete set of plugins.
pub struct MinimalPluginGroup;

impl PluginGroup for MinimalPluginGroup {
    fn configure(&mut self, builder: &mut PluginGroupBuilder) {
        builder.add(qinetic_core::CorePlugin::default());
    }
}

/// Default plugin group includes:
/// * [`CorePlugin`]
/// * [`LogPlugin`]
/// * [`AssetPlugin`]
/// * [`NetworkPlugin`](../qinetic_network/struct.NetworkPlugin.html) - feature = `network`
/// * [`EcsPlugin`]
/// * [`AnimationPlugin`](../qinetic_animation/struct.AnimationPlugin.html) - feature = `animation`
/// * [`AudioPlugin`](../qinetic_audio/struct.AudioPlugin.html) - feature = `audio`
/// * [`PhysicsPlugin`](../qinetic_physics/struct.PhysicsPlugin.html) - feature = `physics`
/// * [`WindowPlugin`]
/// * [`InputPlugin`]
/// * [`RenderPlugin`](../qinetic_render/struct.RenderPlugin.html) - feature = `render`
/// * [`UiPlugin`](../qinetic_ui/struct.UiPlugin.html) - feature = `ui`
///
/// See also [`MinimalPluginGroup`] for a slimmed down option.
pub struct DefaultPluginGroup;

impl PluginGroup for DefaultPluginGroup {
    fn configure(&mut self, builder: &mut PluginGroupBuilder) {
        builder.add(qinetic_core::CorePlugin::default());
        builder.add(qinetic_log::LogPlugin::default());
        builder.add(qinetic_asset::AssetPlugin::default());
        #[cfg(feature = "network")]
        builder.add(qinetic_network::NetworkPlugin::default());
        builder.add(qinetic_ecs::EcsPlugin::default());
        #[cfg(feature = "animation")]
        builder.add(qinetic_animation::AnimationPlugin::default());
        #[cfg(feature = "audio")]
        builder.add(qinetic_audio::AudioPlugin::default());
        #[cfg(feature = "physics")]
        builder.add(qinetic_physics::PhysicsPlugin::default());
        builder.add(qinetic_window::WindowPlugin::default());
        builder.add(qinetic_input::InputPlugin::default());
        #[cfg(feature = "render")]
        builder.add(qinetic_render::RenderPlugin::default());
        #[cfg(feature = "ui")]
        builder.add(qinetic_ui::UiPlugin::default());
    }
}
