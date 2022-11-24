use std::any::{type_name, TypeId};
use std::collections::HashMap;

use crate::stage::*;

#[derive(Default)]
pub struct Schedule {
    stages: HashMap<TypeId, Box<dyn Stage>>,
    order: Vec<TypeId>,
}

impl Schedule {
    pub fn run(&self) {}
}
