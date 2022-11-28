use crate::{entity::EntityId, world::World};
use std::any::{type_name, Any};

/// Data conteiner of the [`World`].
pub trait Component: Any + Send + Sync + 'static {
    /// Returns a `type name` of the [`Component`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Indentificator for [`Entity`] within a [`World`].
#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct ComponentId {
    entity_id: EntityId,
    id: usize,
}

impl ComponentId {
    /// Returns a [`ComponentId`] with given `id` for [`EntityId`].
    #[inline(always)]
    pub const fn new(entity_id: EntityId, id: usize) -> Self {
        Self { entity_id, id }
    }

    /// Returns `id` of the [`ComponentId`].
    #[inline(always)]
    pub fn id(self) -> usize {
        self.id
    }
}

/// Facilities addition and remove [`Component`]s.
#[derive(Default)]
pub struct Components {}

impl Components {
    pub(crate) fn init_component<T: Component>(&mut self, component: T) {
        todo!()
    }

    /// Returns a [`ComponentId`] of added [`Component`].
    #[inline]
    pub fn add<T: Component>(&mut self, component: T) -> ComponentId {
        todo!()
    }

    /// Removes a [`Component`] by [`ComponentId`].
    #[inline]
    pub fn remove(&mut self, id: &ComponentId) {
        todo!()
    }

    /// Returns a immutable [`Component`] by [`ComponentId`], if it's present.
    #[inline]
    pub fn get<T: Component>(&self, id: &ComponentId) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Component`] dy [`ComponentId`], if it's present.
    #[inline]
    pub fn get_mut<T: Component>(&mut self, id: &ComponentId) -> Option<&mut T> {
        todo!()
    }
}
