use crate::{
    component::{Component, ComponentId, Components},
    entity::{Entities, Entity, EntityId},
    event::{Event, Events},
    resource::{Resource, Resources},
    state::{State, States},
    system::{System, Systems},
};
use std::mem;

/// A representation of `world`.
pub struct World {
    components: Components,
    entities: Entities,
    events: Events,
    resources: Resources,
    states: States,
    systems: Systems,
}

impl World {
    /// Returns a [`WorldBuilder`] with `default` configuration.
    pub fn builder() -> WorldBuilder {
        WorldBuilder::default()
    }

    /// Returns a [`ComponentId`] of added [`Component`].
    #[inline]
    pub fn add_component<T: Component>(&mut self, entity_id: &EntityId) -> EntityId {
        //self.components.add::<T>()
        todo!()
    }

    /// Removes a [`Component`] by [`EntityId`].
    #[inline]
    pub fn remove_component(&mut self, id: &EntityId) {
        //self.components.remove(id);
        todo!()
    }

    /// Returns a immutable [`Component`] of [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_component<T: Component>(&self, entity_id: &EntityId) -> Option<&T> {
        //self.components.get::<T>()
        todo!()
    }

    /// Returns a mutable [`Component`] of [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_component_mut<T: Component>(&mut self, entity_id: &EntityId) -> Option<&mut T> {
        //self.components.get_mut::<T>()
        todo!()
    }

    /// Returns a [`EntityId`] of added [`Entity`].
    #[inline]
    pub fn add_entity(&mut self) -> EntityId {
        //self.entities.add::<T>()
        todo!()
    }

    /// Removes a [`Entity`] by [`EntityId`].
    #[inline]
    pub fn remove_entity(&mut self, id: &EntityId) {
        self.entities.remove(id);
    }

    /// Returns a immutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_entity<T: Entity>(&self, id: &EntityId) -> Option<&T> {
        self.entities.get::<T>(id)
    }

    /// Returns a mutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_entity_mut<T: Entity>(&mut self, id: &EntityId) -> Option<&mut T> {
        self.entities.get_mut::<T>(id)
    }

    /// Returns a immutable [`Event`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_event<T: Event>(&self) -> Option<&T> {
        self.events.get::<T>()
    }

    /// Returns a mutable [`Event`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_event_mut<T: Event>(&mut self) -> Option<&mut T> {
        self.events.get_mut::<T>()
    }

    /// Returns a immutable [`Resource`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        self.resources.get::<T>()
    }

    /// Returns a mutable [`Resource`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// Returns a immutable [`State`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_state<T: State>(&self) -> Option<&T> {
        self.states.get::<T>()
    }

    /// Returns a mutable [`State`] by `T` of [`World`], if it's present.
    #[inline]
    pub fn get_state_mut<T: State>(&mut self) -> Option<&mut T> {
        self.states.get_mut::<T>()
    }
}

/// `Builder pattern` for [`World`].
#[derive(Default)]
pub struct WorldBuilder {
    components: Components,
    entities: Entities,
    events: Events,
    resources: Resources,
    states: States,
    systems: Systems,
}

impl WorldBuilder {
    /// Returns a [`WorldBuilder`] with add [`Component`].
    #[inline]
    pub fn with_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.components.init_component(component);
        self
    }

    /// Returns a [`WorldBuilder`] with add [`Event`].
    #[inline]
    pub fn with_event<T: Event>(&mut self, event: T) -> &mut Self {
        self.events.init_event(event);
        self
    }

    /// Returns a [`WorldBuilder`] with added [`Resource`].
    #[inline]
    pub fn with_resource<T: Resource>(&mut self, resource: T) -> &mut Self {
        self.resources.init_resource(resource);
        self
    }

    /// Returns a [`WorldBuilder`] with added a [`State`].
    #[inline]
    pub fn with_state<T: State>(&mut self, state: T) -> &mut Self {
        self.states.init_state(state);
        self
    }

    /// Returns a [`WorldBuilder`] with add [`System`].
    #[inline]
    pub fn with_system<T: System>(&mut self, system: T) -> &mut Self {
        self.systems.init_system(system);
        self
    }

    /// Returns a [`World`] configured from [`WorldBuilder`].
    pub fn build(&mut self) -> World {
        World {
            components: mem::take(&mut self.components),
            entities: mem::take(&mut self.entities),
            events: mem::take(&mut self.events),
            resources: mem::take(&mut self.resources),
            states: mem::take(&mut self.states),
            systems: mem::take(&mut self.systems),
        }
    }
}
