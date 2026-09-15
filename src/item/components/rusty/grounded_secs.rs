use bevy::prelude::*;

use super::{RUST_AFTER_SECS, Rusty};
use crate::{Item, OnGround};

pub struct GroundedSecsPlugin;

impl Plugin for GroundedSecsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rust_grounded_items)
            .add_observer(track_grounded_items)
            .register_type::<GroundedSecs>();
    }
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct GroundedSecs(pub f32);

fn rust_grounded_items(
    time: Res<Time>,
    mut items: Query<(Entity, &mut GroundedSecs), (With<Item>, With<OnGround>, Without<Rusty>)>,
    mut commands: Commands,
) {
    for (item_e, mut grounded) in &mut items {
        grounded.0 += time.delta_secs();
        if grounded.0 >= RUST_AFTER_SECS {
            commands.entity(item_e).insert(Rusty);
        }
    }
}

/// Adds `GroundedSecs` timer the moment an item is on the ground for the first time
fn track_grounded_items(add: On<Add<OnGround>>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .insert_if_new(GroundedSecs::default());
}
