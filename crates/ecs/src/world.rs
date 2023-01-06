//! World functionality.

use crate::{
    component::{Component, ComponentRegistry},
    entity::{EntityId, EntityRegistry},
    event::{Event, EventRegistry},
    resource::{Resource, ResourceRegistry},
    state::{State, StateRegistry},
};

use qinetic_utils::prelude::*;

/// A representation of ECS `world`.
///
/// # Examples
/// ```
/// # use qinetic_ecs::prelude::*;
/// #
/// let world = World::default();
/// ```
#[derive(SmartDefault, Clone)]
pub struct World {
    component_registry: ComponentRegistry,
    entity_registry: EntityRegistry,
    event_registry: EventRegistry,
    resource_registry: ResourceRegistry,
    state_registry: StateRegistry,
}

impl World {
    /// Returns a [`World`] with `default` configuration.
    pub fn new() -> Self {
        World::default()
    }

    /// Adds a [`Component`] to [`Entity`] by [`EntityId`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// #[derive(Default, Component)]
    /// struct MyComponent {/* someting to do */}
    ///
    /// let mut world = World::builder()
    ///     .with_component(MyComponent::default())
    ///     .build()
    ///     .add_component::<MyComponent>(EntityId::default());
    /// #
    /// # assert!(world.has_component::<MyComponent>(EntityId::default()));
    /// ```
    #[inline]
    pub fn add_component<T: Component>(&mut self, entity_id: EntityId) -> &mut Self {
        self.component_registry.add_component::<T>(entity_id);
        self
    }

    /// Removes a [`Component`] of [`Entity`] by [`EntityId`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// #[derive(Default, Component)]
    /// struct MyComponent {/* someting to do */}
    ///
    /// let mut world = World::builder()
    ///     .with_component(MyComponent::default())
    ///     .build()
    ///     .remove_component::<MyComponent>(EntityId::default());
    /// #
    /// # assert!(!world.has_component::<MyComponent>(EntityId::default()));
    /// ```
    #[inline]
    pub fn remove_component<T: Component>(&mut self, entity_id: EntityId) {
        self.component_registry.remove_component::<T>(entity_id);
    }

    /// Returns `true`, if [`Component`] of [`Entity`] by [`EntityId`] present.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// #[derive(Default, Component)]
    /// struct MyComponent {/* someting to do */}
    ///
    /// let mut world = World::builder()
    ///     .with_component(MyComponent::default())
    ///     .build();
    ///
    /// world.add_component::<MyComponent>(EntityId::default());
    ///
    /// assert!(world.has_component::<MyComponent>(EntityId::default()));
    /// ```
    #[inline]
    pub fn has_component<T: Component>(&self, entity_id: EntityId) -> bool {
        self.component_registry.has_component::<T>(entity_id)
    }

    /// Returns a immutable [`Component`] of [`Entity`] by [`EntityId`], if it's present.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// #[derive(Default, Component)]
    /// struct MyComponent {/* someting to do */}
    ///
    /// let mut world = World::builder()
    ///     .with_component(MyComponent::default())
    ///     .build();
    ///
    /// world.add_component::<MyComponent>(EntityId::default());
    ///
    /// # assert!(world.has_component::<MyComponent>(EntityId::default()));
    /// let component = world
    ///     .get_component::<MyComponent>(EntityId::default())
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<&T> {
        self.component_registry.get_component::<T>(entity_id)
    }

    /// Returns a mutable [`Component`] of [`Entity`] by [`EntityId`], if it's present.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// #[derive(Default, Component)]
    /// struct MyComponent {/* someting to do */}
    ///
    /// let mut world = World::builder()
    ///     .with_component(MyComponent::default())
    ///     .build();
    ///
    /// world.add_component::<MyComponent>(EntityId::default());
    /// #
    /// # assert!(world.has_component::<MyComponent>(EntityId::default()));
    /// #
    /// let component = world
    ///     .get_component_mut::<MyComponent>(EntityId::default())
    ///     .unwrap();
    /// // component.something = ...
    /// ```
    #[inline]
    pub fn get_component_mut<T: Component>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        self.component_registry.get_component_mut::<T>(entity_id)
    }

    /// Adds a [`Entity`].
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// #[derive(Default, Entity)]
    /// struct MyEntity {/* someting to do */}
    ///
    /// let mut world = World::builder().build().add_entity::<MyEntity>();
    /// #
    /// # assert!(world.)
    /// ```
    #[inline]
    pub fn add_entity(&mut self) -> EntityId {
        self.entity_registry.add_entity()
    }

    /// Removes a [`Entity`] by [`EntityId`].
    #[inline]
    pub fn remove_entity(&mut self, id: EntityId) {
        self.entity_registry.remove_entity(id);
    }

    /// Returns `true`, if [`Entity`] by [`EntityId`] present.
    #[inline]
    pub fn has_entity(&self, id: EntityId) -> bool {
        self.entity_registry.has_entity(id)
    }

    /// Returns a immutable [`Event`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn add_event<T: Event>(&self, event: T) -> Option<&T> {
        self.event_registry.get_event::<T>()
    }

    /// Returns a immutable [`Event`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn remove_event<T: Event>(&self) -> Option<&T> {
        self.event_registry.get_event::<T>()
    }

    /// Returns `true`, if [`Event`] by `T` present.
    #[inline]
    pub fn has_event<T: Event>(&self) -> bool {
        self.event_registry.has_event::<T>()
    }

    /// Returns a immutable [`Resource`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn add_resource<T: Resource>(&self, resource: T) -> Option<&T> {
        self.resource_registry.get_resource::<T>()
    }

    /// Returns a immutable [`Resource`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn remove_resource<T: Resource>(&self) -> Option<&T> {
        self.resource_registry.get_resource::<T>()
    }

    /// Returns a immutable [`Resource`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        self.resource_registry.get_resource::<T>()
    }

    /// Returns a mutable [`Resource`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resource_registry.get_resource_mut::<T>()
    }

    /// Returns `true`, if [`Resource`] by `T` present.
    #[inline]
    pub fn has_resource<T: Resource>(&self) -> bool {
        self.resource_registry.has_resource::<T>()
    }

    /// Returns a immutable [`State`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn add_state<T: State>(&self, state: T) -> Option<&T> {
        self.state_registry.get_state::<T>()
    }

    /// Returns a immutable [`State`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn remove_state<T: State>(&self) -> Option<&T> {
        self.state_registry.get_state::<T>()
    }

    /// Returns a immutable [`State`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_state<T: State>(&self) -> Option<&T> {
        self.state_registry.get_state::<T>()
    }

    /// Returns a mutable [`State`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_state_mut<T: State>(&mut self) -> Option<&mut T> {
        self.state_registry.get_state_mut::<T>()
    }

    /// Returns `true`, if [`State`] by `T` present.
    #[inline]
    pub fn has_state<T: State>(&self) -> bool {
        self.state_registry.has_state::<T>()
    }
}
