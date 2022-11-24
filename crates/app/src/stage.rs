use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::app::App;
use crate::schedule::*;

/// [`App`]'s step of execution cycle.
pub trait Stage: Any {
    /// Returns a `type name` of the [`Stage`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Combines multiple [`Stage`]s into a group.
pub trait StageGroup {
    /// Adds a [`Stage`]s in group to the [`StageRegistry`].
    fn configure(&self, registry: &mut StageRegistry);

    /// Returns a `type name` of the [`StageGroup`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

struct StageEntry {
    stage: Box<dyn Stage>,
    enabled: bool,
}

/// Facilities the .
#[derive(Default)]
pub struct StageRegistry {
    stages: HashMap<TypeId, StageEntry>,
    order: Vec<TypeId>,
}

impl StageRegistry {
    /// Adds a [`Stage`] in [`StageRegistry`] at the end.
    /// If the stage was already in the group, its removed from its previous place.
    pub fn add<T>(&mut self, stage: T) -> &mut Self
    where
        T: Stage,
    {
        let i = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Adds a [`Stage`] in [`StageRegistry`] before `Target` stage.
    /// If the stage was already in the group, its removed from its previous place.
    pub fn add_before<Target, T>(&mut self, stage: T) -> &mut Self
    where
        Target: Stage,
        T: Stage,
    {
        let i = self.index_of::<Target>();
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Adds a [`Stage`] in [`StageRegistry`] after `Target` stage.
    /// If the stage was already in the group, its removed from its previous place.
    pub fn add_after<Target, T>(&mut self, stage: T) -> &mut Self
    where
        Target: Stage,
        T: Stage,
    {
        let i = self.index_of::<Target>() + 1;
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Enables a [`Stage`].
    pub fn enable<T>(&mut self) -> &mut Self
    where
        T: Stage,
    {
        let mut e = self
            .stages
            .get_mut(&TypeId::of::<T>())
            .expect("Cannot enable a stage that does not exist.");
        e.enabled = true;

        self
    }

    /// Disables a [`Stage`].
    pub fn disable<T>(&mut self) -> &mut Self
    where
        T: Stage,
    {
        let mut e = self
            .stages
            .get_mut(&TypeId::of::<T>())
            .expect("Cannot disable a stage that does not exist.");
        e.enabled = false;

        self
    }

    /// [builds](Stage::build) the contained [`Stage`]s.
    pub fn build(&self) -> Schedule {
        let mut schedule = Schedule::default();

        for p in &self.order {
            if let Some(e) = self.stages.get(p) {
                if e.enabled {}
            }
        }

        schedule
    }

    /// Finds the index of a target [`Stage`]. Panics if the target's [`TypeId`] is not found.
    fn index_of<Target>(&mut self) -> usize
    where
        Target: Stage,
    {
        let i = self.order.iter().position(|&p| p == TypeId::of::<Target>());

        match i {
            Some(i) => i,
            None => panic!("Stage does not exist in group: {}.", type_name::<Target>()),
        }
    }

    /// Insert the new [`Stage`] as enabled, and removes its previous ordering if it was already present.
    fn upsert<T>(&mut self, stage: T, index: usize)
    where
        T: Stage,
    {
        if let Some(e) = self.stages.insert(
            TypeId::of::<T>(),
            StageEntry {
                stage: Box::new(stage),
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
