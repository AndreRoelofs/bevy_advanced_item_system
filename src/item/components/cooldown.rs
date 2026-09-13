use bevy::{ecs::component::ComponentId, prelude::*};
use smallvec::SmallVec;

use crate::StatOp;

pub struct CooldownPlugin;

impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct Cooldown {
    base: f32,
    contributions: SmallVec<[(ComponentId, StatOp); 2]>,
}

impl Cooldown {
    pub fn new(value: f32) -> Self {
        Self {
            base: value,
            ..Default::default()
        }
    }

    pub fn add_contribution(&mut self, component_id: ComponentId, op: StatOp) {
        if let Some((_, contribution)) = self
            .contributions
            .iter_mut()
            .find(|(id, _)| *id == component_id)
        {
            *contribution = op;
        } else {
            self.contributions.push((component_id, op));
        }
    }

    pub fn remove_contribution(&mut self, component_id: ComponentId) {
        self.contributions.retain(|(id, _)| *id != component_id);
    }

    pub fn effective(&self) -> f32 {
        let mut flat = 0.0;
        let mut mult = 1.0;

        for (_, op) in &self.contributions {
            match op {
                StatOp::Flat(value) => flat += value,
                StatOp::Mult(value) => mult *= value,
            }
        }

        (self.base + flat) * mult
    }
}
