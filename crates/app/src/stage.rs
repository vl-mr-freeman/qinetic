use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

use crate::schedule::*;

/// [`Schedule`]'s step of execution cycle.
pub trait Stage: Send + Sync + Any {
    /// Returns a `type name` of the [`Stage`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Combines multiple [`Stage`]s into a group.
pub trait StageGroup {
    /// Adds a [`Stage`]s in group to the [`StageRegistry`].
    fn configure(&mut self, registry: &mut StageRegistry);

    /// Returns a `type name` of the [`StageGroup`].
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

/// Registers a [`Stage`]s.
#[derive(Default)]
pub struct StageRegistry {
    stages: HashMap<TypeId, Box<dyn Stage>>,
    order: Vec<TypeId>,
}

impl StageRegistry {
    /// Returns a [`StageRegistry`] with added [`Stage`] at the end.
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    pub fn add<T: Stage>(&mut self, stage: T) -> &mut Self {
        let i = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Returns a [`StageRegistry`] with added [`Stage`] after `Target` [`Stage`].
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    pub fn add_after<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        let i = self.index_of::<Target>() + 1;
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Returns a [`StageRegistry`] with added [`Stage`] before `Target` [`Stage`].
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    pub fn add_before<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        let i = self.index_of::<Target>();
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Returns a [`StageRegistry`] with added [`Stage`]s of the [`StageGroup`] at the end.
    /// If the [`Stage`] of the [`StageGroup`] was already present, it's removed from its previous place and add at the end.
    pub fn add_group<T: StageGroup>(&mut self, mut group: T) -> &mut Self {
        group.configure(self);
        self
    }

    /// [builds](Stage::build) the contained [`Stage`]s.
    pub fn build(&mut self) -> Schedule {
        let mut schedule = Schedule::default();

        for p in &self.order {
            if let Some(e) = self.stages.get(p) {}
        }

        schedule
    }

    /// Finds the index of a `Target` [`Stage`].
    /// If the `Target` [`Stage`] isn't found, then `panic`.
    fn index_of<Target: Stage>(&mut self) -> usize {
        let i = self.order.iter().position(|&p| p == TypeId::of::<Target>());

        match i {
            Some(i) => i,
            None => panic!(
                "Stage does not present in registry: {}.",
                type_name::<Target>()
            ),
        }
    }

    /// Insert the new [`Stage`] as enabled, and removes its previous ordering
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    fn upsert<T: Stage>(&mut self, stage: T, index: usize) {
        self.stages.insert(TypeId::of::<T>(), Box::new(stage));

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
