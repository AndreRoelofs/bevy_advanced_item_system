use bevy::prelude::*;

mod camera;
mod item;
mod view;

pub use camera::*;
pub use item::*;
pub use view::*;

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, ViewPlugin, ItemPlugin))
        .add_systems(Startup, (setup, spawn_guns))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(
            PLATFORM_HALF * 2.0,
            PLATFORM_THICKNESS,
            PLATFORM_HALF * 2.0,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.3, 0.9),
            ..default()
        })),
        Transform::from_xyz(0.0, PLATFORM_TOP_Y - PLATFORM_THICKNESS * 0.5, 0.0),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, PLATFORM_TOP_Y + EYE_HEIGHT, 0.0),
            Player::default(),
            AmbientLight {
                brightness: 200.0,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.15, 0.15, 0.6))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::srgb(0.8, 0.6, 0.4),
                    unlit: true,
                    ..default()
                })),
                Transform::from_xyz(0.3, -0.3, -0.5),
            ));
            parent.spawn((Transform::from_xyz(-0.3, -0.3, -0.6), Visibility::default()));
        });
}

fn spawn_guns(mut commands: Commands) {
    commands.queue_spawn_scene(bsn! {
        @Magnum
        OnGround
        Transform { translation: Vec3::new(0.0, PLATFORM_TOP_Y + 0.075, -3.0) }
    });
}
