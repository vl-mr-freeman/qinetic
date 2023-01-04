//! System functionality.

use crate::world::World;
use std::any::{type_name, Any};

/// System of the [`World`].
pub trait System: Any + Send + Sync + 'static {
    type Data;

    fn run(&mut self, data: Self::Data);
}

/// Facilities addition and remove [`System`]s.
#[derive(Default)]
pub struct SystemRegistry {}

impl SystemRegistry {
    pub(crate) fn init_system<T: System>(&mut self, system: T) {}
}

qinetic_utils::define_label!(
    /// A strongly-typed class of labels used to identify [`System`].
    SystemLabel,
    /// Strongly-typed identifier for a [`SystemLabel`].
    SystemLabelId,
);
