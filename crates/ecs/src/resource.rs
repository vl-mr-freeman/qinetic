//! Resource functionality.

use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;

/// Resource of the [`World`].
///
/// # Examples
/// ```
/// # use qinetic_ecs::prelude::*;
/// #
/// #[derive(Default, Resource)]
/// struct MyResource {/* something to do */}
/// ```
pub trait Resource: Any + Send + Sync + 'static {
    /// Returns a `type name` of the [`Resource`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`Resource`]s.
#[derive(Default)]
pub struct ResourceRegistry {
    resources: HashMap<TypeId, Box<dyn Resource>>,
}

impl ResourceRegistry {
    #[inline]
    pub fn init_resource<T: Resource>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

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
        self.resources
            .iter()
            .position(|r| *r.0 == TypeId::of::<T>())
            .is_some()
    }
}
