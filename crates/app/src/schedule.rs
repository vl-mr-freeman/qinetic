use std::any::{type_name, TypeId};
use std::collections::HashMap;

use crate::stage::*;
use qinetic_ecs::world::World;

#[derive(Default)]
pub struct Schedule {
    stages: HashMap<TypeId, Box<dyn Stage>>,
    order: Vec<TypeId>,
}

impl Schedule {
    pub fn run(&self, world: &mut World) {}
}
