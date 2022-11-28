use crate::{
    plugin::{Plugin, PluginGroup, PluginRegistry},
    runner::Runner,
    schedule::Schedule,
    stage::{Stage, StageGroup, StageRegistry},
};
use qinetic_ecs::{
    component::Component,
    event::Event,
    resource::Resource,
    state::State,
    system::System,
    world::{World, WorldBuilder},
};
use std::mem;

/// A conteiner of application logic.
pub struct App {
    runner: Box<dyn Runner>,
    schedule: Schedule,
    world: World,
}

impl App {
    /// Returns a [`AppBuilder`] with `default` configuration.
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    /// Starts a [`App`] by calling the [runner function](AppBuilder::with_runner).
    pub fn run(mut self) {
        let mut runner = mem::replace(&mut self.runner, Box::new(RunEmpty));

        runner.run(self);
    }

    /// Advances the execution of [`App`] by one cycle.
    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }
}

/// A `Builder Pattern` for [`App`].
#[derive(Default)]
pub struct AppBuilder {
    runner: Option<Box<dyn Runner>>,
    plugin_registry: PluginRegistry,
    stage_registry: StageRegistry,
    world_builder: WorldBuilder,
}

impl AppBuilder {
    /// Returns a [`AppBuilder`] with set a [`Runner`].
    #[inline]
    pub fn with_runner<T: Runner>(&mut self, runner: T) -> &mut Self {
        self.runner = Some(Box::new(runner));
        self
    }

    /// Returns a [`AppBuilder`] with added a single [`Stage`].
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    #[inline]
    pub fn with_stage<T: Stage>(&mut self, stage: T) -> &mut Self {
        self.stage_registry.add(stage);
        self
    }

    /// Returns a [`AppBuilder`] with added a single [`Stage`] after `Target` [`Stage`].
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    #[inline]
    pub fn with_stage_after<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        self.stage_registry.add_after::<Target, _>(stage);
        self
    }

    /// Returns a [`AppBuilder`] with added a single [`Stage`] before `Target` [`Stage`].
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    #[inline]
    pub fn with_stage_before<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        self.stage_registry.add_before::<Target, _>(stage);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Stage`]s of the [`StageGroup`] at the end.
    /// If the [`Stage`] of the [`StageGroup`] was already present, it's removed from its previous place and add at the end.
    #[inline]
    pub fn with_stages<T: StageGroup>(&mut self, group: T) -> &mut Self {
        self.stage_registry.add_group(group);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Plugin`] at the end.
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    #[inline]
    pub fn with_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugin_registry.add(plugin);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Plugin`] after `Target` [`Plugin`].
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    #[inline]
    pub fn with_plugin_after<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugin_registry.add_after::<Target, _>(plugin);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Plugin`] before `Target` [`Plugin`].
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    #[inline]
    pub fn with_plugin_before<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugin_registry.add_before::<Target, _>(plugin);
        self
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`]s of the [`PluginGroup`] at the end.
    /// If the [`Plugin`] of the [`PluginGroup`] was already present, it's removed from its previous place and add at the end.
    #[inline]
    pub fn with_plugins<T: PluginGroup>(&mut self, group: T) -> &mut Self {
        self.plugin_registry.add_group(group);
        self
    }

    /// Returns a [`AppBuilder`] with add a [`Component`].
    #[inline]
    pub fn with_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.world_builder.with_component(component);
        self
    }

    /// Returns a [`AppBuilder`] with add a [`Event`].
    #[inline]
    pub fn with_event<T: Event>(&mut self, event: T) -> &mut Self {
        self.world_builder.with_event(event);
        self
    }

    /// Returns a [`AppBuilder`] with add a [`Resource`].
    #[inline]
    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.world_builder.with_resource(resource);
        self
    }

    /// Returns a [`AppBuilder`] with added [`State`].
    #[inline]
    pub fn with_state<T: State>(&mut self, state: T) -> &mut Self {
        self.world_builder.with_state(state);
        self
    }

    /// Returns a [`AppBuilder`] with add a single [`System`].
    #[inline]
    pub fn with_system<T: System, S: Stage>(&mut self, system: T) -> &mut Self {
        self.world_builder.with_system(system);
        self
    }

    /// Returns a [`App`] configured from [`AppBuilder`].
    pub fn build(&mut self) -> App {
        let mut plugin_registry = mem::take(&mut self.plugin_registry);
        let mut stage_registry = mem::take(&mut self.stage_registry);

        let mut app_builder = mem::take(self);

        plugin_registry.build(&mut app_builder);

        App {
            runner: app_builder.runner.unwrap_or_else(|| Box::new(RunEmpty)),
            schedule: stage_registry.build(),
            world: app_builder.world_builder.build(),
        }
    }
}

struct RunEmpty;

impl Runner for RunEmpty {
    #[allow(unused_variables)]
    fn run(&mut self, app: App) {}
}
