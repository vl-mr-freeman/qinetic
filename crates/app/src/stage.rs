//! Application stage functionality.

use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

use crate::schedule::*;

/// [`Schedule`]'s step of execution cycle.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// struct MyStage;
///
/// impl Stage for MyStage {}
/// ```
pub trait Stage: Send + Sync + Any {
    /// Returns a `type name` of the [`Stage`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Combines multiple [`Stage`]s into a group.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// struct MyStage1;
///
/// impl Stage for MyStage1 {}
///
/// struct MyStage2;
///
/// impl Stage for MyStage2 {}
///
/// struct MyStages;
///
/// impl StageGroup for MyStages {
///     fn configure(&mut self, registry: &mut StageRegistry) {
///         registry.add_stage(MyStage1):
///         registry.add_stage(MyStage2):
///     }
/// }
/// ```
pub trait StageGroup {
    /// Adds a [`Stage`]s in group to the [`StageRegistry`].
    fn configure(&mut self, registry: &mut StageRegistry);

    /// Returns a `type name` of the [`StageGroup`].
    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`Stage`]s.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// #
/// struct MyStage1;
///
/// impl Stage for MyStage1 {}
///
/// struct MyStage2;
///
/// impl Stage for MyStage2 {}
///
/// struct MyStage3;
///
/// impl Stage for MyStage3 {}
///
/// let mut stage_registry = StageRegistry::default();
/// stage_registry.add_stage(MyStage1);
/// stage_registry.add_stage_after::<MyStage1, _>(MyStage2);
/// stage_registry.add_stage_before::<MyStage2, _>(MyStage3);
///
/// # assert!(stage_registry.has_stage::<MyStage1>());
/// # assert!(stage_registry.has_stage::<MyStage2>());
/// # assert!(stage_registry.has_stage::<MyStage3>());
/// ```
#[derive(Default)]
pub struct StageRegistry {
    /// [`Stage`]s by [`TypeId`].
    stages: HashMap<TypeId, Box<dyn Stage>>,

    /// Linear order of [`Stage`]s.
    order: Vec<TypeId>,
}

impl StageRegistry {
    /// Returns a [`StageRegistry`] with added [`Stage`] at the end.
    ///
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyStage;
    ///
    /// impl Stage for MyStage {}
    ///
    /// let mut stage_registry = StageRegistry::default();
    /// stage_registry.add_stage(MyStage);
    ///
    /// # assert!(stage_registry.has_stage::<MyStage>());
    // ```
    pub fn add_stage<T: Stage>(&mut self, stage: T) -> &mut Self {
        let i = self.order.len();
        self.order.push(TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Returns a [`StageRegistry`] with added [`Stage`] after `Target` [`Stage`].
    ///
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {}
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {}
    ///
    /// let mut stage_registry = StageRegistry::default();
    /// stage_registry.add_stage(MyStage1);
    /// stage_registry.add_stage_after::<MyStage1, _>(MyStage2);
    ///
    /// # assert!(stage_registry.has_stage::<MyStage1>());
    /// # assert!(stage_registry.has_stage::<MyStage2>());
    /// ```
    pub fn add_stage_after<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        let i = self.index_of::<Target>();
        let i = match i {
            Some(i) => i + 1,
            None => panic!(
                "Failed to add Stage after, it's does not present in registry: {}.",
                type_name::<Target>()
            ),
        };
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Returns a [`StageRegistry`] with added [`Stage`] before `Target` [`Stage`].
    ///
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {}
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {}
    ///
    /// let mut stage_registry = StageRegistry::default();
    /// stage_registry.add_stage(MyStage1);
    /// stage_registry.add_stage_before::<MyStage1, _>(MyStage2);
    ///
    /// # assert!(stage_registry.has_stage::<MyStage1>());
    /// # assert!(stage_registry.has_stage::<MyStage2>());
    /// ```
    pub fn add_stage_before<Target: Stage, T: Stage>(&mut self, stage: T) -> &mut Self {
        let i = self.index_of::<Target>();
        let i = match i {
            Some(i) => i + 1,
            None => panic!(
                "Failed to add Stage after, it's does not present in registry: {}.",
                type_name::<Target>()
            ),
        };
        self.order.insert(i, TypeId::of::<T>());
        self.upsert(stage, i);
        self
    }

    /// Returns `true`, if [`Stage`] by `T`, present in [`StageRegistry`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {}
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {}
    ///
    /// let mut stage_registry = StageRegistry::default();
    /// stage_registry.add_stage(MyStage1);
    /// stage_registry.add_stage_before::<MyStage1, _>(MyStage2);
    ///
    /// assert!(stage_registry.has_stage::<MyStage1>());
    /// assert!(stage_registry.has_stage::<MyStage2>());
    /// ```
    pub fn has_stage<T: Stage>(&mut self) -> bool {
        self.index_of::<T>().is_some()
    }

    /// Returns a [`StageRegistry`] with added [`Stage`]s of the [`StageGroup`] at the end.
    ///
    /// If the [`Stage`] of the [`StageGroup`] was already present, it's removed from its previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {}
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {}
    ///
    /// struct MyStages;
    ///
    /// impl StageGroup for MyStages {
    ///     fn configure(&mut self, registry: &mut StageRegistry) {
    ///         registry.add_stage(MyStage1);
    ///         registry.add_stage(MyStage2);
    ///     }
    /// }
    /// let mut stage_registry = StageRegistry::default();
    /// stage_registry.add_stage_group(MyStages);
    ///
    /// # assert!(stage_registry.has_stage::<MyStage1>());
    /// # assert!(stage_registry.has_stage::<MyStage2>());
    /// ```
    pub fn add_stage_group<T: StageGroup>(&mut self, mut group: T) -> &mut Self {
        group.configure(self);
        self
    }

    ///
    pub fn build(&mut self) -> Schedule {
        let mut schedule = Schedule::default();

        for p in &self.order {
            if let Some(e) = self.stages.get(p) {}
        }

        schedule
    }

    /// Finds the index of a `Target` [`Stage`].
    fn index_of<Target: Stage>(&mut self) -> Option<usize> {
        self.order.iter().position(|&p| p == TypeId::of::<Target>())
    }

    /// Insert the new [`Stage`] as enabled, and removes its previous ordering
    ///
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
