use bevy::prelude::*;

use crate::{Item, OnGround};

pub struct GroundedSecsPlugin;

impl Plugin for GroundedSecsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(track_grounded_items)
            .register_type::<GroundedSecs>();
    }
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct GroundedSecs(pub f32);

/// Adds `GroundedSecs` timer the moment an item is on the ground for the first time
fn track_grounded_items(add: On<Add<OnGround>>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .insert_if_new(GroundedSecs::default());
}
