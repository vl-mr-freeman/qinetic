//! System functionality.

use crate::world::World;
use std::any::{type_name, Any};

pub trait IntoSystem {}

/// System of the [`World`].
pub trait System: Any + Send + Sync + 'static + Sized {
    /// Returns a `type name` of the [`System`].
    fn name(&self) -> &str {
        type_name::<Self>()
    }
}

/// Facilities addition and remove [`System`]s.
#[derive(Default)]
pub struct SystemRegistry {}

impl SystemRegistry {
    pub(crate) fn init_system<T: System>(&mut self, system: T) {
        todo!()
    }
}
