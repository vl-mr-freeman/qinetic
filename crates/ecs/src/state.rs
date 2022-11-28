use crate::world::World;
use std::any::type_name;

/// State of the [`World`].
pub trait State: Send + Sync + 'static {
    /// Returns a `type name` of the [`State`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`State`]s.
#[derive(Default)]
pub struct States {}

impl States {
    pub(crate) fn init_state<T: State>(&mut self, state: T) {
        todo!()
    }

    /// Adds a [`State`].
    /// If the [`State`] by `T`, was already present, it's replace.
    #[inline]
    pub fn add<T: State>(&mut self, state: T) {
        todo!()
    }

    /// Removes a [`State`] by `T`.
    #[inline]
    pub fn remove<T: State>(&mut self) {
        todo!()
    }

    /// Returns a immutable [`State`] by `T`, if it's present.
    #[inline]
    pub fn get<T: State>(&self) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`State`] by `T`, if it's present.
    #[inline]
    pub fn get_mut<T: State>(&mut self) -> Option<&mut T> {
        todo!()
    }
}
