use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

use crate::app::*;

/// [`App`]'s additional feature.
pub trait Plugin: Any {
    /// Configures the [`AppBuilder`] to which this plugin is added.
    fn build(&mut self, app_builder: &mut AppBuilder);

    /// Returns a `type name` of the [`Plugin`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Combines multiple [`Plugin`]s into a group.
pub trait PluginGroup {
    /// Adds a [`Plugin`]s in group to the [`PluginRegistry`].
    fn configure(&mut self, registry: &mut PluginRegistry);

    /// Returns a `type name` of the [`PluginGroup`].
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

/// Registers a [`Plugin`]s.
#[derive(Default)]
pub struct PluginRegistry {
    plugins: HashMap<TypeId, Box<dyn Plugin>>,
    order: Vec<TypeId>,
}

impl PluginRegistry {
    /// Returns a [`PluginRegistry`] with added [`Plugin`] at the end.
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    pub fn add<T: Plugin>(&mut self, plugin: T) -> &mut Self {
        let i = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`] after `Target` [`Plugin`].
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    pub fn add_after<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        let i = self.index_of::<Target>() + 1;
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`] before `Target` [`Plugin`].
    /// If the [`Plugin`] was already present, it's removed from it's previous place and add at the end.
    pub fn add_before<Target: Plugin, T: Plugin>(&mut self, plugin: T) -> &mut Self {
        let i = self.index_of::<Target>();
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Returns a [`PluginRegistry`] with added [`Plugin`]s of the [`PluginGroup`] at the end.
    /// If the [`Plugin`] of the [`PluginGroup`] was already present, it's removed from its previous place and add at the end.
    pub fn add_group<T: PluginGroup>(&mut self, mut group: T) -> &mut Self {
        group.configure(self);
        self
    }

    /// [Builds](Plugin::build) the present [`Plugin`]s.
    pub fn build(&mut self, app_builder: &mut AppBuilder) {
        for tp in &mut self.order {
            if let Some(p) = self.plugins.get_mut(tp) {
                p.build(app_builder);
            }
        }
    }

    /// Finds the index of a `Target` [`Plugin`].
    /// If the `Target` [`Plugin`] isn't found, then `panic`.
    fn index_of<Target: Plugin>(&mut self) -> usize {
        let i = self.order.iter().position(|&p| p == TypeId::of::<Target>());

        match i {
            Some(i) => i,
            None => panic!(
                "Plugin does not present in registry: {}.",
                type_name::<Target>()
            ),
        }
    }

    /// Insert the new [`Plugin`] as enabled, and removes its previous ordering
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
