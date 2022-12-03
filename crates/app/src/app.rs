//! Application functionality.

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
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// App::builder().build().run();
/// ```
pub struct App {
    /// The [runner function](Self::run) is primarily responsible for managing loop.
    runner: Box<dyn Runner>,

    /// Container of [`Stage`]s in a linear order.
    schedule: Schedule,

    /// The ECS [`World`], provides access to all ECS data.
    world: World,
}

impl App {
    /// Returns a [`AppBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// let app_builder = App::builder();
    /// ```
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    /// Starts a [`App`] by calling the [runner function](AppBuilder::with_runner).
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// App::builder().build().run();
    /// ```
    pub fn run(mut self) {
        let mut runner = mem::replace(&mut self.runner, Box::new(RunEmpty));

        runner.run(self);
    }

    /// Advances the execution of [`App`] by one cycle.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyRunner;
    ///
    /// impl Runner for MyRunner {
    ///     fn run(&mut self, mut app: App) {
    ///         app.update();
    ///     }
    /// }
    ///
    /// App::builder()
    ///     .with_runner(MyRunner)
    ///     .build()
    ///     .run();
    /// ```
    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }
}

/// A `Builder Pattern` for [`App`].
#[derive(Default)]
pub struct AppBuilder {
    /// The [runner function](Self::with_runner) is primarily responsible for managing loop.
    runner: Option<Box<dyn Runner>>,

    /// Registers [`Plugin`]s.
    plugin_registry: PluginRegistry,

    /// Registers [`Stage`]s.
    stage_registry: StageRegistry,

    /// The ECS [`WorldBuilder`], builds a [`World`].
    world_builder: WorldBuilder,
}

impl AppBuilder {
    /// Returns a [`AppBuilder`] with set a [`Runner`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// #[derive(Runner)]
    /// struct MyRunner;
    ///
    /// App::builder()
    ///     .with_runner(MyRunner)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_runner<T: Runner>(&mut self, runner: T) -> &mut Self {
        self.runner = Some(Box::new(runner));
        self
    }

    /// Returns a [`AppBuilder`] with added a single [`Stage`].
    ///
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// # use qinetic_ecs::world::World;
    /// #
    /// #[derive(Stage)]
    /// struct MyStage;
    ///
    /// App::builder()
    ///     .with_stage(MyStage)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_stage<T: Stage>(&mut self, stage: T) -> &mut Self {
        self.stage_registry.add_stage(stage);
        self
    }

    /// Returns a [`AppBuilder`] with added a single [`Stage`] after `Target` [`Stage`].
    ///
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// # use qinetic_ecs::world::World;
    /// #
    /// #[derive(Stage)]
    /// struct MyStage1;
    ///
    /// #[derive(Stage)]
    /// struct MyStage2;
    ///
    /// App::builder()
    ///     .with_stage(MyStage1)
    ///     .with_stage_after::<MyStage1, _>(MyStage2)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_stage_after<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        self.stage_registry.add_stage_after::<Target, _>(stage);
        self
    }

    /// Returns a [`AppBuilder`] with added a single [`Stage`] before `Target` [`Stage`].
    ///
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    ///
    /// ```
    /// # use qinetic_app::prelude::*;
    /// # use qinetic_ecs::world::World;
    /// #
    /// #[derive(Stage)]
    /// struct MyStage1;
    ///
    /// #[derive(Stage)]
    /// struct MyStage2;
    ///
    /// App::builder()
    ///     .with_stage(MyStage1)
    ///     .with_stage_before::<MyStage1, _>(MyStage2)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_stage_before<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        self.stage_registry.add_stage_before::<Target, _>(stage);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Stage`]s of the [`StageGroup`] at the end.
    ///
    /// If the [`Stage`] of the [`StageGroup`] was already present, it's removed from its previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// # use qinetic_ecs::world::World;
    /// #
    /// #[derive(Stage)]
    /// struct MyStage1;
    ///
    /// #[derive(Stage)]
    /// struct MyStage2;
    ///
    /// struct MyStages;
    ///
    /// impl StageGroup for MyStages {
    ///     fn configure(&mut self, registry: &mut StageRegistry) {
    ///         registry.add_stage(MyStage1);
    ///         registry.add_stage(MyStage2);
    ///     }
    /// }
    ///
    /// App::builder()
    ///     .with_stages(MyStages)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_stages<T: StageGroup>(&mut self, group: T) -> &mut Self {
        self.stage_registry.add_stage_group(group);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Plugin`] at the end.
    ///
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// #[derive(Plugin)]
    /// struct MyPlugin;
    ///
    /// App::builder()
    ///     .with_plugin(MyPlugin)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugin_registry.add_plugin(plugin);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Plugin`] after `Target` [`Plugin`].
    ///
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// #[derive(Plugin)]
    /// struct MyPlugin1;
    ///
    /// #[derive(Plugin)]
    /// struct MyPlugin2;
    ///
    /// App::builder()
    ///     .with_plugin(MyPlugin1)
    ///     .with_plugin_after::<MyPlugin1, _>(MyPlugin2)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_plugin_after<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugin_registry.add_plugin_after::<Target, _>(plugin);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Plugin`] before `Target` [`Plugin`].
    ///
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    ///
    /// #[derive(Plugin)]
    /// struct MyPlugin1;
    ///
    /// #[derive(Plugin)]
    /// struct MyPlugin2;
    ///
    /// App::builder()
    ///     .with_plugin(MyPlugin1)
    ///     .with_plugin_before::<MyPlugin1, _>(MyPlugin2)
    ///      .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_plugin_before<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        self.plugin_registry.add_plugin_before::<Target, _>(plugin);
        self
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`]s of the [`PluginGroup`] at the end.
    ///
    /// If the [`Plugin`] of the [`PluginGroup`] was already present, it's removed from its previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// #[derive(Plugin)]
    /// struct MyPlugin1;
    ///
    /// #[derive(Plugin)]
    /// struct MyPlugin2;
    ///
    /// struct MyPlugins;
    ///
    /// impl PluginGroup for MyPlugins {
    ///     fn configure(&mut self, registry: &mut PluginRegistry) {
    ///         registry.add_plugin(MyPlugin1);
    ///         registry.add_plugin(MyPlugin2);
    ///     }
    /// }
    ///
    /// App::builder()
    ///     .with_plugins(MyPlugins)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_plugins<T: PluginGroup>(&mut self, group: T) -> &mut Self {
        self.plugin_registry.add_plugin_group(group);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Component`].
    ///
    /// If the [`Component`], was already present, it's replace.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// use qinetic_ecs::prelude::*;
    ///  
    /// #[derive(Default, Component)]
    /// struct MyComponent {/* something */}
    ///
    /// App::builder()
    ///     .with_component(MyComponent::default())
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.world_builder.with_component(component);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Event`].
    ///
    /// If the [`Event`], was already present, it's replace.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// use qinetic_ecs::prelude::*;
    ///
    /// #[derive(Default, Event)]
    /// struct MyEvent {/* something */}
    ///
    /// App::builder()
    ///     .with_event(MyEvent::default())
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_event<T: Event>(&mut self, event: T) -> &mut Self {
        self.world_builder.with_event(event);
        self
    }

    /// Returns a [`AppBuilder`] with added [`Resource`].
    ///
    /// If the [`Resource`], was already present, it's replace.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// use qinetic_ecs::prelude::*;
    ///  
    /// #[derive(Default, Resource)]
    /// struct MyResource {/* something */}
    ///
    /// App::builder()
    ///     .with_resource(MyResource::default())
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.world_builder.with_resource(resource);
        self
    }

    /// Returns a [`AppBuilder`] with added [`State`].
    ///
    /// If the [`State`], was already present, it's replace.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// use qinetic_ecs::prelude::*;
    ///  
    /// #[derive(Default, State)]
    /// enum MyState {
    ///     #[default]
    ///     State,
    /// }
    ///
    /// App::builder()
    ///     .with_state(MyState::default())
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_state<T: State>(&mut self, state: T) -> &mut Self {
        self.world_builder.with_state(state);
        self
    }

    /// Returns a [`AppBuilder`] with add a single [`System`].
    ///
    /// If the [`System`], was already present, it's replace.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// use qinetic_ecs::prelude::*;
    ///  
    /// #[derive(System)]
    /// struct MySystem;
    ///
    /// #[derive(Stage)]
    /// struct MyStage;
    ///
    /// App::builder()
    ///     .with_system::<_, MyStage>(MySystem)
    ///     .build()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_system<T: System, S: Stage>(&mut self, system: T) -> &mut Self {
        self.world_builder.with_system(system);
        self
    }

    /// Returns a [`App`] configured from [`AppBuilder`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// App::builder().build().run();
    /// ```
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

/// [`Runner`], that's doing nothing.
struct RunEmpty;

impl Runner for RunEmpty {
    #[allow(unused_variables)]
    fn run(&mut self, app: App) {}
}
