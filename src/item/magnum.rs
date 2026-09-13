use bevy::prelude::*;

pub struct MagnumPlugin;

impl Plugin for MagnumPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Magnum>();
    }
}

#[derive(SceneComponent, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Magnum;

impl Magnum {
    pub fn scene() -> impl Scene {
        bsn! {}
    }
}
