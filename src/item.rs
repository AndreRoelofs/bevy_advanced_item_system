use bevy::prelude::*;

mod components;
mod views;

pub use components::*;
pub use views::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OnGround>()
            .register_type::<EquippedBy>()
            .register_type::<StoredIn>();

        app.world_mut()
            .register_mutually_exclusive_components::<(OnGround, EquippedBy, StoredIn)>();

        app.add_plugins((ItemComponentsPlugin, ItemViewsPlugin));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct OnGround;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EquippedBy;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StoredIn;
