use crate::world::World;
use std::any::{type_name, Any};

/// Entity of a [`World`].
pub trait Entity: Any + Send + Sync + 'static {
    /// Call with [begin](World::begin) of the [`World`].
    fn begin(&mut self, entity_id: &EntityId, world: &World);

    /// Call with [end](World::end) of the [`World`].
    fn end(&mut self, entity_id: &EntityId, world: &World);

    /// Call with [update](World::update) of the [`World`].
    fn update(&mut self, entity_id: &EntityId, world: &World);

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
pub struct Entities {}

impl Entities {
    /// Returns a [`EntityId`] of added [`Entity`].
    #[inline]
    pub fn add<T: Entity>(&mut self) -> EntityId {
        todo!()
    }

    /// Removes a [`Entity`] by [`EntityId`].
    #[inline]
    pub fn remove(&mut self, id: &EntityId) {
        todo!()
    }

    /// Returns a immutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get<T: Entity>(&self, id: &EntityId) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Entity`] by [`EntityId`], if it's present.
    #[inline]
    pub fn get_mut<T: Entity>(&mut self, id: &EntityId) -> Option<&mut T> {
        todo!()
    }
}
