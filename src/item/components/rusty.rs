use bevy::{ecs::component::ComponentIdFor, prelude::*};

use crate::{Cooldown, EquippedBy, Item, OnGround, StatOp, StoredIn, View};

mod grounded_secs;

pub use grounded_secs::*;

const RUST_AFTER_SECS: f32 = 5.0;
const RUST_COOLDOWN_MULT: f32 = 2.0;
const RUST_COLOR: Color = Color::srgb(0.45, 0.22, 0.08);

#[derive(Component, Clone, Default)]
pub struct Rusty;

pub struct RustyPlugin;

impl Plugin for RustyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rust_material)
            .add_observer(attach_rust_modifier)
            .add_observer(detach_rust_modifier)
            .add_observer(rust_on_ground)
            .add_observer(rust_on_equipped)
            .add_observer(rust_on_stored)
            .add_plugins(GroundedSecsPlugin);
    }
}

#[derive(Resource)]
struct RustMaterial(Handle<StandardMaterial>);

fn setup_rust_material(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let material = materials.add(StandardMaterial::from(RUST_COLOR));
    commands.insert_resource(RustMaterial(material));
}

// We need to color the gun brown both when the state is changed for items and
// when those items receive `Rusty` regardless of their state
fn rust_on_ground(
    add: On<Add<(OnGround, Rusty)>>,
    items: Query<&View, (With<Item>, With<OnGround>, With<Rusty>)>,
    mut meshes: Query<&mut MeshMaterial3d<StandardMaterial>>,
    material: Res<RustMaterial>,
) {
    if let Ok(view) = items.get(add.entity)
        && let Some(entity) = view.entity()
        && let Ok(mut mesh_material) = meshes.get_mut(entity)
    {
        mesh_material.0 = material.0.clone();
    }
}

fn rust_on_equipped(
    add: On<Add<(EquippedBy, Rusty)>>,
    items: Query<&View, (With<Item>, With<EquippedBy>, With<Rusty>)>,
    mut meshes: Query<&mut MeshMaterial3d<StandardMaterial>>,
    material: Res<RustMaterial>,
) {
    if let Ok(view) = items.get(add.entity)
        && let Some(entity) = view.entity()
        && let Ok(mut mesh_material) = meshes.get_mut(entity)
    {
        mesh_material.0 = material.0.clone();
    }
}

fn rust_on_stored(
    add: On<Add<(StoredIn, Rusty)>>,
    items: Query<&View, (With<Item>, With<StoredIn>, With<Rusty>)>,
    mut backgrounds: Query<&mut BackgroundColor>,
) {
    if let Ok(view) = items.get(add.entity)
        && let Some(entity) = view.entity()
        && let Ok(mut background) = backgrounds.get_mut(entity)
    {
        background.0 = RUST_COLOR;
    }
}

fn attach_rust_modifier(
    add: On<Add<Rusty>>,
    mut items: Query<&mut Cooldown, With<Item>>,
    rusty_id: ComponentIdFor<Rusty>,
) {
    let model = add.event().entity;
    let Ok(mut cooldown) = items.get_mut(model) else {
        return;
    };
    cooldown.add_contribution(*rusty_id, StatOp::Mult(RUST_COOLDOWN_MULT));
}

fn detach_rust_modifier(
    remove: On<Remove<Rusty>>,
    mut items: Query<&mut Cooldown, With<Item>>,
    rusty_id: ComponentIdFor<Rusty>,
) {
    let model = remove.event().entity;
    let Ok(mut cooldown) = items.get_mut(model) else {
        return;
    };
    cooldown.remove_contribution(*rusty_id);
}
