use bevy::prelude::*;

mod components;
mod magnum;

pub use components::*;
pub use magnum::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OnGround>()
            .register_type::<EquippedBy>()
            .register_type::<StoredIn>();

        app.world_mut()
            .register_mutually_exclusive_components::<(OnGround, EquippedBy, StoredIn)>();

        app.add_plugins((ItemComponentsPlugin, MagnumPlugin));
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

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct ItemKey(pub String);

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct ItemLabel(pub String);

#[derive(Component, Clone, Default)]
pub struct Item {
    pub key: ItemKey,
    pub label: ItemLabel,
}
