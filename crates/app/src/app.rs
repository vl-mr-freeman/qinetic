use crate::plugin::*;
use std::mem;

/// A conteiner of application logic.
pub struct App {
    runner: Box<dyn Fn(App)>,
}

impl Default for App {
    fn default() -> Self {
        let app = App::empty();

        app
    }
}

impl App {
    /// Returns a [`App`] with some default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a [`App`] with minimal configuration.
    pub fn empty() -> Self {
        Self {
            runner: Box::new(run_once),
        }
    }

    /// Sets the function that will be called when the [`App`] is run.
    pub fn set_runner(&mut self, func: impl Fn(App) + 'static) -> &mut Self {
        self.runner = Box::new(func);

        self
    }

    /// Starts the [`App`].
    pub fn run(&mut self) {
        let mut app = mem::replace(self, App::empty());
        let runner = mem::replace(&mut app.runner, Box::new(run_once));

        (runner)(app);
    }

    /// Advances the execution by one cycle
    pub fn update(&mut self) {}

    /// Adds a single [`Plugin`].
    pub fn add_plugin<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        plugin.build(self);

        self
    }

    /// Adds a group of [`Plugin`]s.
    pub fn add_plugin_group<T>(&mut self, mut group: T) -> &mut Self
    where
        T: PluginGroup,
    {
        let mut builder = PluginGroupBuilder::default();
        group.configure(&mut builder);
        builder.build(self);

        self
    }

    /// Adds a group of [`Plugin`]s with an initializer method.
    pub fn add_plugin_group_with<T, F>(&mut self, mut group: T, func: F) -> &mut Self
    where
        T: PluginGroup,
        F: FnOnce(&mut PluginGroupBuilder) -> &mut PluginGroupBuilder,
    {
        let mut builder = PluginGroupBuilder::default();
        group.configure(&mut builder);
        func(&mut builder);
        builder.build(self);

        self
    }
}

fn run_once(mut app: App) {
    app.update();
}
