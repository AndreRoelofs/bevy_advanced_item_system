use bevy::prelude::*;

pub(crate) mod input;

pub const MOVE_SPEED: f32 = 5.0;
pub const PLATFORM_TOP_Y: f32 = 0.0;
pub const EYE_HEIGHT: f32 = 1.7;
pub const PLATFORM_HALF: f32 = 15.0;
pub const GRAVITY: f32 = 9.81;
pub const RESPAWN_Y: f32 = -25.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(input::PlayerInputPlugin)
            .register_type::<Player>()
            .register_system(move_player);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player {
    pub yaw: f32,
    pub pitch: f32,
    pub velocity_y: f32,
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
