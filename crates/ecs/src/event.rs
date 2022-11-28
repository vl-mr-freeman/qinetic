use crate::world::World;
use std::any::{type_name, Any};

/// Event of the [`World`].
pub trait Event: Any + Send + Sync + 'static {
    /// Calls a [`Event`].
    fn call(&self) {}

    /// Binds a [`Event`].
    fn bind(&mut self);

    /// Unbinds a [`Event`].
    fn unbind(&mut self);

    /// Returns a `type name` of the [`Event`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`Event`]s.
#[derive(Default)]
pub struct Events {}

impl Events {
    pub(crate) fn init_event<T: Event>(&mut self, event: T) {
        todo!()
    }

    /// Adds a [`Event`].
    /// If the [`Event`] by `T`, was already present, it's replace.
    #[inline]
    pub fn add<T: Event>(&mut self, event: T) {
        todo!()
    }

    /// Removes a [`Event`] by `T`.
    #[inline]
    pub fn remove<T: Event>(&mut self) {
        todo!()
    }

    /// Returns a immutable [`Event`] by `T`, if it's present.
    #[inline]
    pub fn get<T: Event>(&self) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Event`] by `T`, if it's present.
    #[inline]
    pub fn get_mut<T: Event>(&mut self) -> Option<&mut T> {
        todo!()
    }
}
