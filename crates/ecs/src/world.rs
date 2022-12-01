//! World functionality.

use crate::{
    component::{Component, ComponentRegistry},
    entity::{Entity, EntityId, EntityRegistry},
    event::{Event, EventRegistry, IntoEvent},
    resource::{Resource, ResourceRegistry},
    state::{State, StateRegistry},
    system::{IntoSystem, System, SystemRegistry},
};
use std::mem;

/// A representation of ECS `world`.
///
/// # Examples
/// ```
/// # use qinetic_ecs::prelude::*;
/// #
/// World::builder().build();
/// ```
pub struct World {
    component_registry: ComponentRegistry,
    entity_registry: EntityRegistry,
    event_registry: EventRegistry,
    resource_registry: ResourceRegistry,
    state_registry: StateRegistry,
    system_registry: SystemRegistry,
}

impl World {
    /// Returns a [`WorldBuilder`] with `default` configuration.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// let world_builder = World::builder();
    /// ```
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
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
    pub fn add_component<T: Component>(&mut self, entity_id: EntityId) {
        self.component_registry.add_component::<T>(entity_id);
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
    /// let component = world.get_component::<MyComponent>(EntityId::default()).unwrap();
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
    /// let component = world.get_component_mut::<MyComponent>(EntityId::default()).unwrap();
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
    /// let mut world = World::builder()
    ///     .build()
    ///     .add_entity::<MyEntity>();
    /// #
    /// # assert!(world.)
    /// ```
    #[inline]
    pub fn add_entity<T: Entity>(&mut self) {
        self.entity_registry.add_entity::<T>();
    }

    /// Removes a [`Entity`] by [`EntityId`].
    #[inline]
    pub fn remove_entity<T: Entity>(&mut self, id: EntityId) {
        self.entity_registry.remove_entity::<T>(id);
    }

    /// Returns a immutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_entity<T: Entity>(&self, id: EntityId) -> Option<&T> {
        self.entity_registry.get_entity::<T>(id)
    }

    /// Returns a mutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_entity_mut<T: Entity>(&mut self, id: EntityId) -> Option<&mut T> {
        self.entity_registry.get_entity_mut::<T>(id)
    }

    /// Returns `true`, if [`Entity`] by [`EntityId`] present.
    #[inline]
    pub fn has_entity<T: Entity>(&self, id: EntityId) -> bool {
        self.entity_registry.has_entity::<T>(id)
    }

    /// Returns a immutable [`Event`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_event<T: Event>(&self) -> Option<&T> {
        self.event_registry.get_event::<T>()
    }

    /// Returns a mutable [`Event`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_event_mut<T: Event>(&mut self) -> Option<&mut T> {
        self.event_registry.get_event_mut::<T>()
    }

    /// Returns `true`, if [`Event`] by `T` present.
    #[inline]
    pub fn has_event<T: Event>(&self) -> bool {
        self.event_registry.has_event::<T>()
    }

    pub(crate) fn call_event<T: Event>(&self) {
        self.event_registry.call_event::<T>();
    }

    pub(crate) fn bind_event<T: Event, IE: IntoEvent>(&self, into_event: IE) {
        self.event_registry.bind_event::<T, _>(into_event);
    }

    pub(crate) fn unbind_event<T: Event>(&self) {
        self.event_registry.unbind_event::<T>();
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

/// `Builder pattern` for [`World`].
#[derive(Default)]
pub struct WorldBuilder {
    component_registry: ComponentRegistry,
    entity_registry: EntityRegistry,
    event_registry: EventRegistry,
    resource_registry: ResourceRegistry,
    state_registry: StateRegistry,
    system_registry: SystemRegistry,
}

impl WorldBuilder {
    /// Returns a [`WorldBuilder`] with add [`Component`].
    #[inline]
    pub fn with_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.component_registry.init_component(component);
        self
    }

    /// Returns a [`WorldBuilder`] with add [`Event`].
    #[inline]
    pub fn with_event<T: Event>(&mut self, event: T) -> &mut Self {
        self.event_registry.init_event(event);
        self
    }

    /// Returns a [`WorldBuilder`] with added [`Resource`].
    #[inline]
    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resource_registry.init_resource(resource);
        self
    }

    /// Returns a [`WorldBuilder`] with added a [`State`].
    #[inline]
    pub fn with_state<T: State>(&mut self, state: T) -> &mut Self {
        self.state_registry.init_state(state);
        self
    }

    /// Returns a [`WorldBuilder`] with add [`System`].
    #[inline]
    pub fn with_system<T: System>(&mut self, system: T) -> &mut Self {
        self.system_registry.init_system(system);
        self
    }

    /// Returns a [`World`] configured from [`WorldBuilder`].
    pub fn build(&mut self) -> World {
        World {
            component_registry: mem::take(&mut self.component_registry),
            entity_registry: mem::take(&mut self.entity_registry),
            event_registry: mem::take(&mut self.event_registry),
            resource_registry: mem::take(&mut self.resource_registry),
            state_registry: mem::take(&mut self.state_registry),
            system_registry: mem::take(&mut self.system_registry),
        }
    }
}
