//! Application stage functionality.

use std::{
    any::{type_name, TypeId},
    collections::HashMap,
};

use qinetic_ecs::world::World;
use qinetic_utils::prelude::*;

use crate::schedule::*;

/// [`Schedule`]'s step of execution cycle.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ecs::world::World;
/// #
/// struct MyStage;
///
/// impl Stage for MyStage {
///     fn run(&mut self, world: &mut World) {
///         // Something to do
///     }
/// }
/// ```
pub trait Stage: DynClone + DowncastSync {
    /// Runs step of execution.
    fn run(&mut self, world: &mut World);
}

impl_downcast!(sync Stage);
clone_trait_object!(Stage);

qinetic_utils::define_label!(
    /// A strongly-typed class of labels used to identify [`Stage`].
    StageLabel,
    /// Strongly-typed identifier for a [`StageLabel`].
    StageLabelId,
);

/// Combines multiple [`Stage`]s into a group.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ecs::world::World;
/// #
/// struct MyStage1;
///
/// impl Stage for MyStage1 {
///     fn run(&mut self, world: &mut World) {
///         // Something to do
///     }
/// }
///
/// struct MyStage2;
///
/// impl Stage for MyStage2 {
///     fn run(&mut self, world: &mut World) {
///         // Something to do
///     }
/// }
///
/// struct MyStageGroup;
///
/// impl StageGroup for MyStageGroup {
///     fn configure(&mut self, registry: &mut StageRegistry) {
///         registry.add_stage(MyStage1).add_stage(MyStage2);
///     }
/// }
/// ```
pub trait StageGroup {
    /// Adds a [`Stage`]s in group to the [`StageRegistry`].
    fn configure(&mut self, registry: &mut StageRegistry);
}

/// Facilities addition and remove [`Stage`]s.
///
/// # Examples
/// ```
/// # use qinetic_app::prelude::*;
/// # use qinetic_ecs::world::World;
/// #
/// struct MyStage1;
///
/// impl Stage for MyStage1 {
///     fn run(&mut self, world: &mut World) { // Something to do
///     }
/// }
///
/// struct MyStage2;
///
/// impl Stage for MyStage2 {
///     fn run(&mut self, world: &mut World) { // Something to do
///     }
/// }
///
/// struct MyStage3;
///
/// impl Stage for MyStage3 {
///     fn run(&mut self, world: &mut World) { // Something to do
///     }
/// }
///
/// let mut stage_registry = StageRegistry::default()
///     .add_stage(MyStage1)
///     .add_stage_after(MyStage1, MyStage2)
///     .add_stage_before(MyStage2, MyStage3);
///
/// # assert!(stage_registry.has_stage::<MyStage1>());
/// # assert!(stage_registry.has_stage::<MyStage2>());
/// # assert!(stage_registry.has_stage::<MyStage3>());
/// ```
#[derive(SmartDefault)]
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
    /// # use qinetic_ecs::world::World;
    /// #
    /// struct MyStage;
    ///
    /// impl Stage for MyStage {
    ///     fn run(&mut self, world: &mut World) {/* Something to do */}
    /// }
    ///
    /// let mut stage_registry =
    ///     StageRegistry::default()
    ///         .add_stage(MyStage);
    ///
    /// # assert!(stage_registry.has_stage::<MyStage>());
    // ```
    pub fn add_stage<T: Stage>(&mut self, label: impl StageLabel, stage: T) -> &mut Self {
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
    /// # use qinetic_ecs::world::World;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// let mut stage_registry = StageRegistry::default()
    ///     .add_stage(MyStage1)
    ///     .add_stage_after(MyStage1, MyStage2);
    ///
    /// # assert!(stage_registry.has_stage::<MyStage1>());
    /// # assert!(stage_registry.has_stage::<MyStage2>());
    /// ```
    pub fn add_stage_after<T: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: T,
    ) -> &mut Self {
        // let i = self.index_of::<Target>();
        // let i = match i {
        // Some(i) => i + 1,
        // None => panic!(
        // "Failed to add Stage after, it's does not present in registry: {}.",
        // type_name::<Target>()
        // ),
        // };
        // self.order.insert(i, TypeId::of::<T>());
        // self.upsert(stage, i);
        self
    }

    /// Returns a [`StageRegistry`] with added [`Stage`] before `Target` [`Stage`].
    ///
    /// If the [`Stage`] was already present, it's removed from it's previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// # use qinetic_ecs::world::World;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// let mut stage_registry = StageRegistry::default()
    ///     .add_stage(MyStage1)
    ///     .add_stage_before(MyStage1, MyStage2);
    ///
    /// # assert!(stage_registry.has_stage::<MyStage1>());
    /// # assert!(stage_registry.has_stage::<MyStage2>());
    /// ```
    pub fn add_stage_before<T: Stage>(
        &mut self,
        target: impl StageLabel,
        label: impl StageLabel,
        stage: T,
    ) -> &mut Self {
        // let i = self.index_of::<Target>();
        // let i = match i {
        // Some(i) => i + 1,
        // None => panic!(
        // "Failed to add Stage after, it's does not present in registry: {}.",
        // type_name::<Target>()
        // ),
        // };
        // self.order.insert(i, TypeId::of::<T>());
        // self.upsert(stage, i);
        self
    }

    /// Returns `true`, if [`Stage`] by `T`, present in [`StageRegistry`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// # use qinetic_ecs::world::World;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// let mut stage_registry = StageRegistry::default()
    ///     .add_stage(MyStage1)
    ///     .add_stage_before(MyStage1, MyStage2);
    ///
    /// assert!(stage_registry.has_stage::<MyStage1>());
    /// assert!(stage_registry.has_stage::<MyStage2>());
    /// ```
    pub fn has_stage<T: Stage>(&mut self) -> bool { self.index_of::<T>().is_some() }

    /// Returns a [`StageRegistry`] with added [`Stage`]s of the [`StageGroup`] at the end.
    ///
    /// If the [`Stage`] of the [`StageGroup`] was already present, it's removed from its previous place and add at the end.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_app::prelude::*;
    /// # use qinetic_ecs::world::World;
    /// #
    /// struct MyStage1;
    ///
    /// impl Stage for MyStage1 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// struct MyStage2;
    ///
    /// impl Stage for MyStage2 {
    ///     fn run(&mut self, world: &mut World) { // Something to do
    ///     }
    /// }
    ///
    /// struct MyStageGroup;
    ///
    /// impl StageGroup for MyStageGroup {
    ///     fn configure(&mut self, registry: &mut StageRegistry) {
    ///         registry.add_stage(MyStage1).add_stage(MyStage2);
    ///     }
    /// }
    /// let mut stage_registry = StageRegistry::default().add_stage_group(MyStageGroup);
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

#[derive(SmartDefault, Clone)]
pub struct SingleStage {}

impl Stage for SingleStage {
    fn run(&mut self, world: &mut World) {}
}

#[derive(SmartDefault, Clone)]
pub struct ParallelStage {}

impl Stage for ParallelStage {
    fn run(&mut self, world: &mut World) {}
}
