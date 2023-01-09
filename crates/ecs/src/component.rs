//! Component functionality.

use qinetic_utils::prelude::*;

use crate::entity::EntityId;
#[allow(unused_imports)]
use crate::world::*;

/// Data conteiner of the [`World`].
///
/// # Examples
/// ```
/// # use qinetic_ecs::prelude::*;
/// #
/// #[derive(Default, Component)]
/// struct MyComponent {
///     // Something to do
/// }
/// ```
pub trait Component: Send + Sync + 'static {}

/// Identificator for [`Component`] within a [`World`].
#[derive(
    SmartDefault, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, CopyGetters, new,
)]
#[getset(get_copy = "pub")]
pub struct ComponentId {
    id: usize,
}

/// Facilities addition and remove [`Component`]s.
#[derive(SmartDefault, Clone, Debug)]
pub struct ComponentRegistry {}

impl ComponentRegistry {
    /// Adds a [`Component`] to [`Entity`] dy [`EntityId`].
    #[inline]
    pub fn add_component<T: Component>(&mut self, entity_id: EntityId) { todo!() }

    /// Removes a [`Component`] from [`Entity`] dy [`EntityId`], if it's present.
    #[inline]
    pub fn remove_component<T: Component>(&mut self, entity_id: EntityId) { todo!() }

    /// Returns a immutable [`Component`] of [`Entity`] dy [`EntityId`], if it's present.
    #[inline]
    pub fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<&T> { todo!() }

    /// Returns a mutable [`Component`] of [`Entity`] dy [`EntityId`], if it's present.
    #[inline]
    pub fn get_component_mut<T: Component>(&mut self, entity_id: EntityId) -> Option<&mut T> {
        todo!()
    }

    /// Returns `true`, if [`Component`] of [`Entity`] by [`EntityId`] present.
    #[inline]
    pub fn has_component<T: Component>(&self, entity_id: EntityId) -> bool { todo!() }
}
