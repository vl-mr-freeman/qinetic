//! Entity functionality.

use crate::{component::Component, world::World};
use std::any::{type_name, Any};

/// Entity of a [`World`].
pub trait Entity: Any + Send + Sync + 'static {
    /// Adds [`Component`] to [`Entity`].
    #[inline]
    fn add_component<T: Component>(&self, id: EntityId, world: &mut World) {
        world.add_component::<T>(id);
    }

    /// Removes [`Component`] to [`Entity`].
    #[inline]
    fn remove_component<T: Component>(&self, id: EntityId, world: &mut World) {
        world.remove_component::<T>(id);
    }

    /// Returns a immutable [`Component`] of [`Entity`].
    #[inline]
    fn get_component<'a, T: Component>(&'a mut self, id: EntityId, world: &'a World) -> Option<&T> {
        world.get_component::<T>(id)
    }

    /// Returns a mutable [`Component`] of [`Entity`].
    #[inline]
    fn get_component_mut<'a, T: Component>(
        &'a mut self,
        id: EntityId,
        world: &'a mut World,
    ) -> Option<&mut T> {
        world.get_component_mut::<T>(id)
    }

    /// Returns `true`, if [`Entity`] has [`Component`].
    #[inline]
    fn has_component<T: Component>(&self, id: EntityId, world: &World) -> bool {
        world.has_component::<T>(id)
    }

    /// Returns a `type name` of the [`Entity`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Indentificator for [`Entity`] within a [`World`].
#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct EntityId(usize);

impl EntityId {
    /// Returns a [`EntityId`] with given `index`.
    #[inline(always)]
    pub const fn new(index: usize) -> Self {
        Self(index)
    }

    /// Returns `index` of the [`EntityId`].
    #[inline(always)]
    pub fn index(self) -> usize {
        self.0
    }
}

/// Facilities addition and remove [`Entity`]s.
#[derive(Default)]
pub struct EntityRegistry {
    entities: Vec<Option<EntityId>>,
}

impl EntityRegistry {
    /// Returns a [`EntityId`] of added [`Entity`].
    #[inline]
    pub fn add_entity<T: Entity>(&mut self) {
        self.entities.push(Some(EntityId::default()));
    }

    /// Removes a [`Entity`] by [`EntityId`].
    #[inline]
    pub fn remove_entity<T: Entity>(&mut self, id: EntityId) {
        todo!()
    }

    /// Returns a immutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_entity<T: Entity>(&self, id: EntityId) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_entity_mut<T: Entity>(&mut self, id: EntityId) -> Option<&mut T> {
        todo!()
    }

    /// Returns `true`, if [`Entity`] by [`EntityId`] present.
    #[inline]
    pub fn has_entity<T: Entity>(&self, id: EntityId) -> bool {
        todo!()
    }
}
