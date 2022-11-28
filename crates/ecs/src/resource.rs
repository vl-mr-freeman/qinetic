use crate::world::World;
use std::any::type_name;

/// Resource of the [`World`].
pub trait Resource: Send + Sync + 'static {
    /// Returns a `type name` of the [`Resource`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`Resource`]s.
#[derive(Default)]
pub struct Resources {}

impl Resources {
    pub(crate) fn init_resource<T: Resource>(&mut self, resource: T) {
        todo!()
    }

    /// Adds a [`Resource`].
    /// If the [`Resource`] by `T`, was already present, it's replace.
    #[inline]
    pub fn add<T: Resource>(&mut self, resource: T) {
        todo!()
    }

    /// Removes a [`Resource`] by `T`.
    #[inline]
    pub fn remove<T: Resource>(&mut self) {
        todo!()
    }

    /// Returns a immutable [`Resource`] by `T`, if it's present.
    #[inline]
    pub fn get<T: Resource>(&self) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Resource`] by `T`, if it's present.
    #[inline]
    pub fn get_mut<T: Resource>(&mut self) -> Option<&mut T> {
        todo!()
    }
}
