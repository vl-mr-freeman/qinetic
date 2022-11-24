use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

use crate::app::*;

/// [`App`]'s additional feature.
pub trait Plugin: Any {
    /// Configures the [`AppBuilder`] to which this plugin is added.
    fn build(&self, app_builder: &mut AppBuilder);

    /// Returns a `type name` of the [`Plugin`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Combines multiple [`Plugin`]s into a group.
pub trait PluginGroup {
    /// Configures the [`Plugin`]s that are to be added.
    fn configure(&self, registry: &mut PluginRegistry);
}

struct PluginEntry {
    plugin: Box<dyn Plugin>,
    enabled: bool,
}

/// Facilitates the creation and configuration of a [`PluginGroup`].
#[derive(Default)]
pub struct PluginRegistry {
    plugins: HashMap<TypeId, PluginEntry>,
    order: Vec<TypeId>,
}

impl PluginRegistry {
    /// Adds a [`Plugin`] in [`PluginRegistry`] at the end.
    /// If the plugin was already in the group, its removed from its previous place.
    pub fn add<T>(&mut self, plugin: T) -> &mut Self
    where
        T: Plugin,
    {
        let i = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Adds a [`Plugin`] in [`PluginRegistry`] before `Target` plugin.
    /// If the plugin was already in the group, its removed from its previous place.
    pub fn add_before<Target, T>(&mut self, plugin: T) -> &mut Self
    where
        Target: Plugin,
        T: Plugin,
    {
        let i = self.index_of::<Target>();
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Adds a [`Plugin`] in [`PluginRegistry`] after `Target` plugin.
    /// If the plugin was already in the group, its removed from its previous place.
    pub fn add_after<Target, T>(&mut self, plugin: T) -> &mut Self
    where
        Target: Plugin,
        T: Plugin,
    {
        let i = self.index_of::<Target>() + 1;
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(plugin, i);
        self
    }

    /// Enables a [`Plugin`].
    pub fn enable<T>(&mut self) -> &mut Self
    where
        T: Plugin,
    {
        let mut e = self
            .plugins
            .get_mut(&TypeId::of::<T>())
            .expect("Cannot enable a plugin that does not exist.");
        e.enabled = true;

        self
    }

    /// Disables a [`Plugin`].
    pub fn disable<T>(&mut self) -> &mut Self
    where
        T: Plugin,
    {
        let mut e = self
            .plugins
            .get_mut(&TypeId::of::<T>())
            .expect("Cannot disable a plugin that does not exist.");
        e.enabled = false;

        self
    }

    /// [builds](Plugin::build) the contained [`Plugin`]s.
    pub fn build(&self, app_builder: &mut AppBuilder) {
        for p in &self.order {
            if let Some(e) = self.plugins.get(p) {
                if e.enabled {
                    e.plugin.build(app_builder);
                }
            }
        }
    }

    /// Finds the index of a target [`Plugin`]. Panics if the target's [`TypeId`] is not found.
    fn index_of<Target>(&mut self) -> usize
    where
        Target: Plugin,
    {
        let i = self.order.iter().position(|&p| p == TypeId::of::<Target>());

        match i {
            Some(i) => i,
            None => panic!("Plugin does not exist in group: {}.", type_name::<Target>()),
        }
    }

    /// Insert the new [`Plugin`] as enabled, and removes its previous ordering if it was already present.
    fn upsert<T>(&mut self, plugin: T, index: usize)
    where
        T: Plugin,
    {
        if let Some(e) = self.plugins.insert(
            TypeId::of::<T>(),
            PluginEntry {
                plugin: Box::new(plugin),
                enabled: true,
            },
        ) {
            if e.enabled {
                todo!();
            }

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
}
