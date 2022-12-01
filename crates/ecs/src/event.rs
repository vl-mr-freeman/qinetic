//! Event functionality.

use crate::world::World;
use std::any::{type_name, Any};

pub trait IntoEvent {}

/// Event of the [`World`].
pub trait Event: Any + Send + Sync + 'static + Sized {
    /// Calls a [`Event`].
    fn call(&self, world: &mut World) {
        world.call_event::<Self>();
    }

    /// Binds a [`Event`].
    fn bind<IE: IntoEvent>(&mut self, into_event: IE, world: &mut World) {
        world.bind_event::<Self, _>(into_event);
    }

    /// Unbinds a [`Event`].
    fn unbind(&mut self, world: &mut World) {
        world.unbind_event::<Self>();
    }

    /// Returns a `type name` of the [`Event`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`Event`]s.
#[derive(Default)]
pub struct EventRegistry {}

impl EventRegistry {
    pub(crate) fn init_event<T: Event>(&mut self, event: T) {
        todo!()
    }

    pub(crate) fn call_event<T: Event>(&self) {
        todo!()
    }

    pub(crate) fn bind_event<T: Event, IE: IntoEvent>(&self, into_event: IE) {
        todo!()
    }

    pub(crate) fn unbind_event<T: Event>(&self) {
        todo!()
    }

    /// Adds a [`Event`].
    /// If the [`Event`] by `T`, was already present, it's replace.
    #[inline]
    pub fn add_event<T: Event>(&mut self, event: T) {
        todo!()
    }

    /// Removes a [`Event`] by `T`.
    #[inline]
    pub fn remove_event<T: Event>(&mut self) {
        todo!()
    }

    /// Returns a immutable [`Event`] by `T`, if it's present.
    #[inline]
    pub fn get_event<T: Event>(&self) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`Event`] by `T`, if it's present.
    #[inline]
    pub fn get_event_mut<T: Event>(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Returns `true`, if [`Event`] by `T` present.
    #[inline]
    pub fn has_event<T: Event>(&self) -> bool {
        todo!()
    }
}
