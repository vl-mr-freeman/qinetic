use crate::plugin::*;

/// A conteiner of application logic.
pub struct App {}

impl Default for App {
    fn default() -> Self {
        let app = App::empty();

        app
    }
}

impl App {
    /// Create a new [`App`] with some default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new [`App`] with minimal configuration.
    pub fn empty() -> Self {
        Self {}
    }

    /// Starts the [`App`].
    pub fn run(&mut self) {}

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
        group.build(&mut builder);
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
        group.build(&mut builder);
        func(&mut builder);
        builder.build(self);

        self
    }
}
