//! Resource functionality.

use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

use qinetic_utils::prelude::*;

/// Resource of the [`World`].
///
/// # Examples
/// ```
/// # use qinetic_ecs::prelude::*;
/// #
/// #[derive(Default, Resource)]
/// struct MyResource {/* Something to do */}
/// ```
pub trait Resource: Any + Send + Sync + 'static {}

/// Facilities addition and remove [`Resource`]s.
#[derive(SmartDefault, Clone, Debug)]
pub struct ResourceRegistry {}

impl ResourceRegistry {
    #[inline]
    pub fn init_resource<T: Resource>(&mut self, resource: T) {}

    /// Returns a immutable [`Resource`] by `T`, if it's present.
    #[inline]
    pub fn get_resource<T: Resource>(&self) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Resource`] by `T`, if it's present.
    #[inline]
    pub fn get_resource_mut<T: Resource>(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Returns `true`, if [`Resource`] by `T` present.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_ecs::prelude::*;
    /// #
    /// #[derive(Default, Resource)]
    /// struct MyResource1;
    ///
    /// #[derive(Default, Resource)]
    /// struct MyResource2;
    ///
    /// let mut resource_registry = ResourceRegistry::default();
    /// resource_registry.init_resource(MyResource1::default());
    ///
    /// assert!(resource_registry.has_resource::<MyResource1>());
    /// assert!(!resource_registry.has_resource::<MyResource2>());
    /// ```
    #[inline]
    pub fn has_resource<T: Resource>(&self) -> bool {
        todo!()
    }
}
