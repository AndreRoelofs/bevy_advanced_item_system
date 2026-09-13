use bevy::prelude::*;

use crate::{Cooldown, Item, OnGround};

const RUST_AFTER_SECS: f32 = 5.0;
const RUST_COOLDOWN_MULT: f32 = 2.0;

#[derive(Component, Clone, Default)]
pub struct Rusty;

pub struct RustyPlugin;

impl Plugin for RustyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rust_grounded_items)
            .add_observer(track_grounded_items)
            .add_observer(attach_rust_modifier)
            .add_observer(detach_rust_modifier);
    }
}

fn attach_rust_modifier(add: On<Add<Rusty>>, items: Query<(), With<Item>>, mut commands: Commands) {
    let model = add.event().entity;
    if items.get(model).is_err() {
        return;
    }
    // TODO: Somehow pass the stat modifier to Gun's cooldown
}

fn detach_rust_modifier(
    remove: On<Remove<Rusty>>,
    items: Query<(), With<Item>>,
    mut commands: Commands,
) {
    let model = remove.event().entity;
    if items.get(model).is_err() {
        return;
    }
    // TODO: Somehow remove the stat modifier from Gun's cooldown
}

#[derive(Component, Clone, Default)]
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
