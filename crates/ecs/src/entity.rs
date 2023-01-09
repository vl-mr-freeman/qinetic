//! Entity functionality.

use qinetic_utils::prelude::*;

#[allow(unused_imports)]
use crate::world::*;

/// Identificator for `Entity` within a [`World`].
#[derive(
    SmartDefault, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, CopyGetters, new,
)]
#[getset(get_copy = "pub")]
pub struct EntityId {
    id: usize,
}

/// Facilities addition and remove [`Entity`]s.
#[derive(SmartDefault, Clone, Debug)]
pub struct EntityRegistry {}

impl EntityRegistry {
    /// Returns a [`EntityId`].
    #[inline]
    pub fn add_entity(&mut self) -> EntityId { EntityId::default() }

    /// Removes a [`EntityId`].
    #[inline]
    pub fn remove_entity(&mut self, id: EntityId) { todo!() }

    /// Returns `true`, if [`EntityId`] present.
    #[inline]
    pub fn has_entity(&self, id: EntityId) -> bool { todo!() }
}
