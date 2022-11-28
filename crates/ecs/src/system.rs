use crate::world::World;
use std::any::{type_name, Any};

/// System of the [`World`].
pub trait System: Any + Send + Sync + 'static {
    /// Returns a `type name` of the [`System`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`System`]s.
#[derive(Default)]
pub struct Systems {}

impl Systems {
    pub(crate) fn init_system<T: System>(&mut self, system: T) {
        todo!()
    }

    /// Adds a [`System`].
    /// If the [`System`] by `T`, was already present, it's replace.
    #[inline]
    pub fn add<T: System>(&mut self, system: T) {
        todo!()
    }

    /// Removes a [`System`] by `T`.
    #[inline]
    pub fn remove<T: System>(&mut self) {
        todo!()
    }
}
