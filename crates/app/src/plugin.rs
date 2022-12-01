//! Application plugin functionality.

use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

use crate::app::*;

/// [`App`]'s additional feature.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
/// }
/// ```
pub trait Plugin: Any + 'static {
    /// Configures the [`AppBuilder`] to which this plugin is added.
    fn build(&mut self, app_builder: &mut AppBuilder);

    /// Returns a `type name` of the [`Plugin`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Combines multiple [`Plugin`]s into a group.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// struct MyPlugin1;
///
/// impl Plugin for MyPlugin1 {
///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
/// }
///
/// struct MyPlugin2;
///
/// impl Plugin for MyPlugin2 {
///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
/// }
///
/// struct MyPlugins;
///
/// impl PluginGroup for MyPlugins {
///     fn configure(&mut self, registry: &mut PluginRegistry) {
///         registry.add_plugin(MyPlugin1):
///         registry.add_plugin(MyPlugin2):
///     }
/// }
/// ```
pub trait PluginGroup {
    /// Adds a [`Plugin`]s in group to the [`PluginRegistry`].
    fn configure(&mut self, registry: &mut PluginRegistry);

    /// Returns a `type name` of the [`PluginGroup`].
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`Plugin`]s.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// struct MyPlugin1;
///
/// impl Plugin for MyPlugin1 {
///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
/// }
///
/// struct MyPlugin2;
///
/// impl Plugin for MyPlugin2 {
///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
/// }
///
/// struct MyPlugin3;
///
/// impl Plugin for MyPlugin3 {
///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
/// }
///
/// let mut plugin_registry = PluginRegistry::default();
/// plugin_registry.add_plugin(MyPlugin1);
/// plugin_registry.add_plugin_after::<MyPlugin1, _>(MyPlugin2);
/// plugin_registry.add_plugin_before::<MyPlugin2, _>(MyPlugin3);
///
/// # assert!(plugin_registry.has_plugin::<MyPlugin1>());
/// # assert!(plugin_registry.has_plugin::<MyPlugin2>());
/// # assert!(plugin_registry.has_plugin::<MyPlugin3>());
/// ```
#[derive(Default)]
pub struct PluginRegistry {
    /// [`Plugin`]s by [`TypeId`].
    plugins: HashMap<TypeId, Box<dyn Plugin>>,

    /// Linear order of [`Plugin`]s.
    order: Vec<TypeId>,
}

impl PluginRegistry {
    /// Returns a [`PluginRegistry`] with added [`Plugin`] at the end.
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
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// let mut plugin_registry = PluginRegistry::default();
    /// plugin_registry.add_plugin(MyPlugin);
    ///
    /// # assert!(plugin_registry.has_plugin::<MyPlugin>());
    // ```
    pub fn add_plugin<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        let i = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`] after `Target` [`Plugin`].
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
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// let mut plugin_registry = PluginRegistry::default();
    /// plugin_registry.add_plugin(MyPlugin1);
    /// plugin_registry.add_plugin_after::<MyPlugin1, _>(MyPlugin2);
    ///
    /// # assert!(plugin_registry.has_plugin::<MyPlugin1>());
    /// # assert!(plugin_registry.has_plugin::<MyPlugin2>());
    /// ```
    pub fn add_plugin_after<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        let i = self.index_of::<Target>();
        let i = match i {
            Some(i) => i + 1,
            None => panic!(
                "Failed to add Plugin after, it's does not present in registry: {}.",
                type_name::<Target>()
            ),
        };
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`] before `Target` [`Plugin`].
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
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// let mut plugin_registry = PluginRegistry::default();
    /// plugin_registry.add_plugin(MyPlugin1);
    /// plugin_registry.add_plugin_before::<MyPlugin1, _>(MyPlugin2);
    ///
    /// # assert!(plugin_registry.has_plugin::<MyPlugin1>());
    /// # assert!(plugin_registry.has_plugin::<MyPlugin2>());
    /// ```
    pub fn add_plugin_before<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        let i = self.index_of::<Target>();
        let i = match i {
            Some(i) => i,
            None => panic!(
                "Failed to add Plugin before, it's does not present in registry: {}.",
                type_name::<Target>()
            ),
        };
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Returns `true`, if [`Plugin`] by `T`, present in [`PluginRegistry`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyPlugin1;
    ///
    /// impl Plugin for MyPlugin1 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// let mut plugin_registry = PluginRegistry::default();
    /// plugin_registry.add_plugin(MyPlugin1);
    /// plugin_registry.add_plugin_before::<MyPlugin1, _>(MyPlugin2);
    ///
    /// assert!(plugin_registry.has_plugin::<MyPlugin1>());
    /// assert!(plugin_registry.has_plugin::<MyPlugin2>());
    /// ```
    pub fn has_plugin<T: Plugin>(&mut self) -> bool {
        self.index_of::<T>().is_some()
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`]s of the [`PluginGroup`] at the end.
    /// If the [`Plugin`] of the [`PluginGroup`] was already present, it's removed from its previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyPlugin1;
    ///
    /// impl Plugin for MyPlugin1 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// struct MyPlugins;
    ///
    /// impl PluginGroup for MyPlugins {
    ///     fn configure(&mut self, registry: &mut PluginRegistry) {
    ///         registry.add_plugin(MyPlugin1);
    ///         registry.add_plugin(MyPlugin2);
    ///     }
    /// }
    /// let mut plugin_registry = PluginRegistry::default();
    /// plugin_registry.add_plugin_group(MyPlugins);
    ///
    /// # assert!(plugin_registry.has_plugin::<MyPlugin1>());
    /// # assert!(plugin_registry.has_plugin::<MyPlugin2>());
    /// ```
    pub fn add_plugin_group<T: PluginGroup>(&mut self, mut group: T) -> &mut Self {
        group.configure(self);
        self
    }

    /// [Builds](Plugin::build) the present [`Plugin`]s.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyPlugin1;
    ///
    /// impl Plugin for MyPlugin1 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// struct MyPlugin2;
    ///
    /// impl Plugin for MyPlugin2 {
    ///     fn build(&mut self, app_builder: &mut AppBuilder) {/* something to do */}
    /// }
    ///
    /// let mut app_builder = App::builder();
    ///
    /// PluginRegistry::default()
    ///     .add_plugin(MyPlugin1)
    ///     .add_plugin(MyPlugin2)
    ///     .build(&mut app_builder);
    /// ```
    pub fn build(&mut self, app_builder: &mut AppBuilder) {
        for tp in &mut self.order {
            if let Some(p) = self.plugins.get_mut(tp) {
                p.build(app_builder);
            }
        }
    }

    /// Finds the index of a `Target` [`Plugin`].
    fn index_of<Target: Plugin>(&mut self) -> Option<usize> {
        self.order.iter().position(|&p| p == TypeId::of::<Target>())
    }

    /// Insert the new [`Plugin`] as enabled, and removes its previous ordering.
    ///
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    fn upsert<T: Plugin>(&mut self, plugin: T, index: usize) {
        self.plugins.insert(TypeId::of::<T>(), Box::new(plugin));

        if let Some(r) = self
            .order
            .iter()
            .enumerate()
            .find(|(i, p)| *i != index && **p == TypeId::of::<T>())
            .map(|(i, _)| i)
        {
            self.order.remove(r);
        }
    }
}
