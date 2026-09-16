use bevy::input::mouse::{AccumulatedMouseMotion, MouseButton};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::Player;

const LOOK_SENS: f32 = 0.002;
const MAX_PITCH: f32 = 1.5;

pub const PLATFORM_THICKNESS: f32 = 1.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorLocked::default())
            .add_systems(Update, look_around)
            .add_systems(Update, toggle_cursor.in_set(CursorSystems));
    }
}

#[derive(Resource, Default)]
pub struct CursorLocked(pub bool);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CursorSystems;

pub fn set_cursor_lock(cursor: &mut CursorOptions, locked: &mut CursorLocked, lock: bool) {
    cursor.grab_mode = if lock {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };
    cursor.visible = !lock;
    locked.0 = lock;
}

pub fn toggle_cursor(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>,
    mut locked: ResMut<CursorLocked>,
) {
    let Ok(mut cursor) = cursors.single_mut() else {
        return;
    };

    if mouse_buttons.just_pressed(MouseButton::Left) {
        set_cursor_lock(&mut cursor, &mut locked, true);
    }
    if keys.just_pressed(KeyCode::Escape) {
        set_cursor_lock(&mut cursor, &mut locked, false);
    }
}

pub fn look_around(
    mouse: Res<AccumulatedMouseMotion>,
    locked: Res<CursorLocked>,
    mut player: Query<(&mut Transform, &mut Player), With<Camera3d>>,
) {
    if !locked.0 {
        return;
    }
    let Ok((mut transform, mut player)) = player.single_mut() else {
        return;
    };

    player.yaw -= mouse.delta.x * LOOK_SENS;
    player.pitch -= mouse.delta.y * LOOK_SENS;
    player.pitch = player.pitch.clamp(-MAX_PITCH, MAX_PITCH);

    transform.rotation = Quat::from_rotation_y(player.yaw) * Quat::from_rotation_x(player.pitch);
}
