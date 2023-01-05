//! Application stage schedule functionality.

use std::any::TypeId;
use std::collections::HashMap;

use crate::stage::Stage;
use qinetic_ecs::world::World;
use qinetic_utils::prelude::*;

#[derive(SmartDefault)]
pub struct Schedule {
    stages: HashMap<TypeId, Box<dyn Stage>>,
    order: Vec<TypeId>,
}

impl Schedule {
    pub fn run(&self, world: &mut World) {}
}
