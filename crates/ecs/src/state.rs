//! State functionality.

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
pub struct StateRegistry {}

impl StateRegistry {
    pub(crate) fn init_state<T: State>(&mut self, state: T) {}

    /// Adds a [`State`].
    /// If the [`State`] by `T`, was already present, it's replace.
    #[inline]
    pub fn add_state<T: State>(&mut self, state: T) {
        todo!()
    }

    /// Removes a [`State`] by `T`.
    #[inline]
    pub fn remove_state<T: State>(&mut self) {
        todo!()
    }

    /// Returns a immutable [`State`] by `T`, if it's present.
    #[inline]
    pub fn get_state<T: State>(&self) -> Option<&T> {
        todo!()
    }

    /// Returns a mutable [`State`] by `T`, if it's present.
    #[inline]
    pub fn get_state_mut<T: State>(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Returns `true`, if [`State`] by `T` present.
    #[inline]
    pub fn has_state<T: State>(&self) -> bool {
        todo!()
    }
}
