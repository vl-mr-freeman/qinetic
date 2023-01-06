//! Application stage schedule functionality.

use std::any::TypeId;
use std::collections::HashMap;

use crate::{
    prelude::StageLabel,
    stage::{Stage, StageGroup},
};
use qinetic_ecs::{system::System, world::World};
use qinetic_utils::prelude::*;

#[derive(SmartDefault, Clone)]
pub struct Schedule {
    stages: HashMap<TypeId, Box<dyn Stage>>,
    order: Vec<TypeId>,
}

impl Schedule {
    #[inline]
    pub fn add_stage<T: Stage>(&mut self, stage: T) -> &mut Self {
        self
    }

    #[inline]
    pub fn add_stage_group<T: StageGroup>(&mut self, group: T) -> &mut Self {
        self
    }

    #[inline]
    pub fn add_system<T: System>(&mut self, stage: impl StageLabel, system: T) -> &mut Self {
        self
    }

    pub fn run(&self, world: &mut World) {}
}
