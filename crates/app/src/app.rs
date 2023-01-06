//! Application functionality.

use crate::{
    plugin::{Plugin, PluginGroup},
    runner::Runner,
    schedule::Schedule,
    stage::{Stage, StageGroup, StageLabel},
};
use qinetic_ecs::{
    component::Component, event::Event, resource::Resource, state::State, system::System,
    world::World,
};
use qinetic_utils::prelude::*;

/// A conteiner of application logic.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// App::builder().build().unwrap().run();
/// ```
#[derive(SmartDefault, Builder)]
#[builder(crate = "crate::app", setter(prefix = "with"), default)]
pub struct App {
    /// Returns a [`AppBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// let app_builder = App::builder();
    /// ```
    #[builder(setter(custom))]
    #[default(Box::new(RunEmpty))]
    runner: Box<dyn Runner>,

    /// Container of [`Stage`]s in a linear order.
    #[builder(setter(custom))]
    schedule: Schedule,

    /// The ECS [`World`], provides access to all ECS data.
    #[builder(setter(custom))]
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
    #[inline]
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    /// Starts a [`App`] by calling the [runner function](AppBuilder::with_runner).
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// #[derive(Clone)]
    /// struct MyRunner;
    ///
    /// impl Runner for MyRunner {
    ///     fn run(&mut self, mut app: App) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// App::builder().with_runner(MyRunner).build().unwrap().run();
    /// ```
    pub fn run(self) {
        let mut runner = self.runner.clone();
        runner.run(self);
    }

    /// Advances the execution of [`App`] by one cycle.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// #[derive(Clone)]
    /// struct MyRunner;
    ///
    /// impl Runner for MyRunner {
    ///     fn run(&mut self, mut app: App) {
    ///         app.update();
    ///     }
    /// }
    ///
    /// App::builder().with_runner(MyRunner).build().unwrap().run();
    /// ```
    pub fn update(&mut self) {
        self.schedule.run(&mut self.world);
    }
}

impl AppBuilder {
    /// Returns a [`AppBuilder`] with [`Runner`].
    ///
    /// # Examples
    /// ```
    /// #[derive(Clone)]
    /// struct MyRunner;
    ///
    /// impl Runner for MyRunner {
    ///     fn run(&mut self, mut app: App) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// # use qinetic_app::prelude::*;
    /// #
    /// AppBuilder::default().with_runner(MyRunner).build().unwrap();
    /// ```
    #[inline]
    pub fn with_runner<T: Runner>(&mut self, runner: T) -> &mut Self {
        self.runner = Some(Box::new(runner));
        self
    }

    #[inline]
    pub fn with_world(&mut self, world: World) -> &mut Self {
        self.world = Some(world);
        self
    }

    #[inline]
    pub fn with_world_fn<F: FnOnce(&mut World)>(&mut self, func: F) -> &mut Self {
        self
    }

    #[inline]
    pub fn with_schedule(&mut self, schedule: Schedule) -> &mut Self {
        self.schedule = Some(schedule);
        self
    }

    #[inline]
    pub fn with_schedule_fn<F: FnOnce(&mut Schedule)>(&mut self, func: F) -> &mut Self {
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
    /// #[derive(StageLabel)]
    /// struct MyStage;
    ///
    /// AppBuilder::default()
    ///     .with_stage(MyStage, SingleStage::default())
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_stage<T: Stage>(&mut self, label: impl StageLabel, stage: T) -> &mut Self {
        //self.stage_registry.add_stage(label, stage);
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
    /// #[derive(StageLabel)]
    /// struct MyStage1;
    ///
    /// #[derive(StageLabel)]
    /// struct MyStage2;
    ///
    /// AppBuilder::default()
    ///     .with_stage(MyStage1, SingleStage::default())
    ///     .with_stage_after(MyStage1, MyStage2, SingleStage::default())
    ///     .build()
    ///     .unwrap()
    ///     .run();
    /// ```
    #[inline]
    pub fn with_stage_after<T: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: T,
    ) -> &mut Self {
        //self.stage_registry.add_stage_after(target, label, stage);
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
    /// #[derive(StageLabel)]
    /// struct MyStage1;
    ///
    /// #[derive(StageLabel)]
    /// struct MyStage2;
    ///
    /// AppBuilder::default()
    ///     .with_stage(MyStage1, SingleStage::default())
    ///     .with_stage_before(MyStage1, MyStage2, SingleStage::default())
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_stage_before<T: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: T,
    ) -> &mut Self {
        //self.stage_registry.add_stage_before(target, label, stage);
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
    /// #[derive(StageLabel)]
    /// struct MyStage1;
    ///
    /// #[derive(StageLabel)]
    /// struct MyStage2;
    ///
    /// struct MyStageGroup;
    ///
    /// impl StageGroup for MyStageGroup {
    ///     fn configure(&mut self, registry: &mut StageRegistry) {
    ///         registry
    ///             .add_stage(MyStage1, SingleStage::default())
    ///             .add_stage(MyStage2, SingleStage::default());
    ///     }
    /// }
    ///
    /// AppBuilder::default()
    ///     .with_stage_group(MyStageGroup)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_stage_group<T: StageGroup>(&mut self, group: T) -> &mut Self {
        //self.stage_registry.add_stage_group(group);
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
    /// struct MyPlugin;
    ///
    /// impl Plugin for MyPlugin {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// AppBuilder::default().with_plugin(MyPlugin).build().unwrap();
    /// ```
    #[inline]
    pub fn with_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        //self.plugin_registry.add_plugin(plugin);
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
    /// struct MyPlugin1;
    ///
    /// impl Plugin for MyPlugin1 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// AppBuilder::default()
    ///     .with_plugin(MyPlugin1)
    ///     .with_plugin_after::<MyPlugin1, _>(MyPlugin2)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_plugin_after<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        //self.plugin_registry.add_plugin_after::<Target, _>(plugin);
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
    /// struct MyPlugin1;
    ///
    /// impl Plugin for MyPlugin1 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// AppBuilder::default()
    ///     .with_plugin(MyPlugin1)
    ///     .with_plugin_before::<MyPlugin1, _>(MyPlugin2)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_plugin_before<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        //self.plugin_registry.add_plugin_before::<Target, _>(plugin);
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
    /// struct MyPlugin1;
    ///
    /// impl Plugin for MyPlugin1 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// struct MyPluginGroup;
    ///
    /// impl PluginGroup for MyPluginGroup {
    ///     fn configure(&mut self, registry: &mut PluginRegistry) {
    ///         registry.add_plugin(MyPlugin1).add_plugin(MyPlugin2);
    ///     }
    /// }
    ///
    /// AppBuilder::default()
    ///     .with_plugin_group(MyPluginGroup)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_plugin_group<T: PluginGroup>(&mut self, group: T) -> &mut Self {
        //self.plugin_registry.add_plugin_group(group);
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
    /// struct MyComponent {/* Something to do */}
    ///
    /// AppBuilder::default()
    ///     .with_component(MyComponent::default())
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_component<T: Component>(&mut self, component: T) -> &mut Self {
        //self.world_builder.with_component(component);
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
    /// struct MyEvent {/* Something to do */}
    ///
    /// AppBuilder::default()
    ///     .with_event(MyEvent::default())
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_event<T: Event>(&mut self, event: T) -> &mut Self {
        //self.world_builder.with_event(event);
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
    /// struct MyResource {/* Something to do */}
    ///
    /// AppBuilder::default()
    ///     .with_resource(MyResource::default())
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        //self.world_builder.with_resource(resource);
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
    /// AppBuilder::default()
    ///     .with_state(MyState::default())
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_state<T: State>(&mut self, state: T) -> &mut Self {
        //self.world.add_state(state);
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
    /// struct MySystem;
    ///
    /// impl System for MySystem {
    ///     type Data = ();
    ///
    ///     fn run(&mut self, data: Self::Data) {
    ///         /* Something to do */
    ///     }
    /// }
    ///
    /// #[derive(StageLabel)]
    /// struct MyStage;
    ///
    /// AppBuilder::default()
    ///     .with_system(MyStage, MySystem)
    ///     .build()
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn with_system<T: System>(&mut self, stage: impl StageLabel, system: T) -> &mut Self {
        //self.world_builder.with_system(system);
        self
    }
}

#[derive(Clone)]
struct RunEmpty;

impl Runner for RunEmpty {
    fn run(&mut self, app: App) {}
}
