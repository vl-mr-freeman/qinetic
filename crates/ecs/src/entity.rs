//! Entity functionality.

use crate::{component::Component, world::World};
use std::any::{type_name, Any};

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
    /// Returns a [`EntityId`].
    #[inline]
    pub fn add_entity(&mut self) -> EntityId {
        self.entities.push(Some(EntityId::default()));
        EntityId::default()
    }

    /// Removes a [`EntityId`].
    #[inline]
    pub fn remove_entity(&mut self, id: EntityId) {
        todo!()
    }

    /// Returns `true`, if [`EntityId`] present.
    #[inline]
    pub fn has_entity(&self, id: EntityId) -> bool {
        todo!()
    }
}
