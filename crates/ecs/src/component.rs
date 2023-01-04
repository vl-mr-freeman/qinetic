//! Component functionality.

use crate::entity::EntityId;
use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

/// Data conteiner of the [`World`].
///
/// # Examples
/// ```
/// # use qinetic_ecs::prelude::*;
/// #
/// #[derive(Default, Component)]
/// struct MyComponent {/* something to do */}
/// ```
pub trait Component: Any + Send + Sync + 'static {}

struct ComponentInfo {
    entity_id: EntityId,
    type_id: TypeId,
}

/// Facilities addition and remove [`Component`]s.
#[derive(Default)]
pub struct ComponentRegistry {
    components: HashMap<TypeId, Vec<Option<ComponentInfo>>>,
}

impl ComponentRegistry {
    pub(crate) fn init_component<T: Component>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), Vec::with_capacity(32));
    }

    /// Adds a [`Component`] to [`Entity`] dy [`EntityId`].
    #[inline]
    pub fn add_component<T: Component>(&mut self, entity_id: EntityId) {
        todo!()
    }

    /// Removes a [`Component`] from [`Entity`] dy [`EntityId`], if it's present.
    #[inline]
    pub fn remove_component<T: Component>(&mut self, entity_id: EntityId) {
        todo!()
    }

    /// Returns a immutable [`Component`] of [`Entity`] dy [`EntityId`], if it's present.
    #[inline]
    pub fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Component`] of [`Entity`] dy [`EntityId`], if it's present.
    #[inline]
    pub fn get_component_mut<T: Component>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        todo!()
    }

    /// Returns `true`, if [`Component`] of [`Entity`] by [`EntityId`] present.
    #[inline]
    pub fn has_component<T: Component>(&self, entity_id: EntityId) -> bool {
        todo!()
    }
}
