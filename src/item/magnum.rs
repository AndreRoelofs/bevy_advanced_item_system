use bevy::prelude::*;

use crate::{Ammo, Cooldown, Item, ItemKey, ItemLabel, Shootable};

pub struct MagnumPlugin;

impl Plugin for MagnumPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Magnum>();
    }
}

pub const MAGNUM_KEY: &str = "core::item::magnum";

#[derive(SceneComponent, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Magnum;

impl Magnum {
    pub fn scene() -> impl Scene {
        bsn! {
            Item {
                key: {ItemKey(MAGNUM_KEY.to_string())},
                label: {ItemLabel("Magnum".to_string())},
            }
            Shootable { cooldown: {Cooldown(0.5)}, magazine_size: 6 }
            Ammo(12) // 2 Magazines
            Visibility
        }
    }
}
