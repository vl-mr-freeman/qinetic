use crate::{
    event::{Event, EventGroup},
    plugin::{Plugin, PluginGroup, PluginRegistry},
    resource::Resource,
    schedule::Schedule,
    stage::{Stage, StageGroup, StageRegistry},
    state::State,
};
use qinetic_ecs::{component::*, system::*};
use std::mem;

/// A conteiner of application logic.
pub struct App {
    runner: Box<dyn Fn(App) + 'static>,
    schedule: Schedule,
}

impl App {
    /// Returns a [`AppBuilder`] with `default` configuration.
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    /// Starts a [`App`] by calling the [runner function](AppBuilder::with_runner).
    pub fn run(mut self) {
        let runner = mem::replace(&mut self.runner, Box::new(run_once));

        (runner)(self);
    }

    /// Advances the execution of [`App`] by one cycle.
    pub fn update(&mut self) {
        self.schedule.run();
    }
}

/// A `Builder Pattern` for [`App`].
pub struct AppBuilder {
    runner: Option<Box<dyn Fn(App) + 'static>>,
    plugin_registry: PluginRegistry,
    stage_registry: StageRegistry,
}

impl AppBuilder {
    /// Returns a [`AppBuilder`] with set `runner` function.
    #[inline]
    pub fn with_runner(&mut self, runner: impl Fn(App) + 'static) -> &mut Self {
        self.runner = Some(Box::new(runner));
        self
    }

    /// Returns a [`AppBuilder`] with add [`Resource`].
    #[inline]
    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self
    }

    /// Returns a [`AppBuilder`] with add [`Component`].
    #[inline]
    pub fn with_component<T: Component>(&mut self, component: T) -> &mut Self {
        self
    }

    /// Returns a [`AppBuilder`] with add single [`Stage`].
    #[inline]
    pub fn with_stage<T: Stage>(&mut self, stage: T) -> &mut Self {
        self.stage_registry.add(stage);
        self
    }

    /// Returns a [`AppBuilder`] with add single [`Stage`]s.
    #[inline]
    pub fn with_stages<T: StageGroup>(&mut self, group: T) -> &mut Self {
        group.configure(&mut self.stage_registry);

        self
    }

    /// Returns a [`AppBuilder`] with add single [`State`].
    #[inline]
    pub fn with_state<T: State>(&mut self, state: T) -> &mut Self {
        self
    }

    /// Returns a [`AppBuilder`] with add a single [`Event`].
    #[inline]
    pub fn with_event<T: Event>(&mut self) -> &mut Self {
        self
    }

    /// Returns a [`AppBuilder`] with add a multiple [`Event`]s.
    #[inline]
    pub fn with_events<T: EventGroup>(&mut self, group: T) -> &mut Self {
        self
    }

    /// Returns a [`AppBuilder`] with add a single [`System`].
    #[inline]
    pub fn with_system<T: System, S: Stage>(&mut self, system: T, stage: S) -> &mut Self {
        self
    }

    /// Returns a [`AppBuilder`] with add a multiple [`System`]s.
    #[inline]
    pub fn with_systems<T: SystemGroup>(&mut self, group: T) -> &mut Self {
        self
    }

    /// Returns a [`AppBuilder`] with add single [`Plugin`].
    #[inline]
    pub fn with_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugin_registry.add(plugin);
        self
    }

    /// Returns a [`AppBuilder`] with add multiple [`Plugin`]s.
    #[inline]
    pub fn with_plugins<T: PluginGroup>(&mut self, group: T) -> &mut Self {
        group.configure(&mut self.plugin_registry);
        self
    }

    /// Returns a [`App`] configured from [`AppBuilder`].
    pub fn build(&mut self) -> App {
        let plugin_registry = mem::take(&mut self.plugin_registry);
        let stage_registry = mem::take(&mut self.stage_registry);

        let mut app_builder = mem::take(self);

        plugin_registry.build(&mut app_builder);

        App {
            runner: app_builder.runner.unwrap_or(Box::new(run_once)),
            schedule: stage_registry.build(),
        }
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            runner: None,
            plugin_registry: Default::default(),
            stage_registry: Default::default(),
        }
    }
}

fn run_once(mut app: App) {
    app.update();
}
