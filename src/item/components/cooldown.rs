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

    pub fn effective(&self) -> f32 {
        0.0
    }
}
