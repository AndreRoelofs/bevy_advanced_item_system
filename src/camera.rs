use bevy::input::mouse::{AccumulatedMouseMotion, MouseButton};
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

const LOOK_SENS: f32 = 0.002;
const MOVE_SPEED: f32 = 5.0;
const GRAVITY: f32 = 9.81;
const MAX_PITCH: f32 = 1.5;

pub const EYE_HEIGHT: f32 = 1.7;

pub const PLATFORM_HALF: f32 = 15.0;
pub const PLATFORM_TOP_Y: f32 = 0.0;
pub const PLATFORM_THICKNESS: f32 = 1.0;

const RESPAWN_Y: f32 = -25.0;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorLocked::default())
            .add_systems(Update, (look_around, move_player, toggle_cursor));
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub yaw: f32,
    pub pitch: f32,
    pub velocity_y: f32,
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

pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Player), With<Camera3d>>,
) {
    let Ok((mut transform, mut player)) = player.single_mut() else {
        return;
    };

    let forward = transform.forward().with_y(0.0).normalize_or_zero();
    let right = transform.right().with_y(0.0).normalize_or_zero();
    let mut dir = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir -= right;
    }
    transform.translation += dir.normalize_or_zero() * MOVE_SPEED * time.delta_secs();

    let stand_y = PLATFORM_TOP_Y + EYE_HEIGHT;
    let pos = transform.translation;
    let over_platform = pos.x.abs() <= PLATFORM_HALF && pos.z.abs() <= PLATFORM_HALF;

    if over_platform && pos.y <= stand_y {
        player.velocity_y = 0.0;
        transform.translation.y = stand_y;
    } else {
        player.velocity_y -= GRAVITY * time.delta_secs();
        transform.translation.y += player.velocity_y * time.delta_secs();
    }

    if transform.translation.y < RESPAWN_Y {
        transform.translation = Vec3::new(0.0, stand_y, 0.0);
        player.velocity_y = 0.0;
    }
}
