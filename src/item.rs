use bevy::prelude::*;

mod components;
mod magnum;
mod view;

pub use components::*;
pub use magnum::*;
pub use view::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OnGround>()
            .register_type::<EquippedBy>()
            .register_type::<StoredIn>();

        app.world_mut()
            .register_mutually_exclusive_components::<(OnGround, EquippedBy, StoredIn)>();

        app.add_plugins((ItemComponentsPlugin, MagnumPlugin, ItemViewPlugin));
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct OnGround;

impl OnGround {
    pub const KEY: &str = "core::item_state::on_ground";
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EquippedBy;

impl EquippedBy {
    pub const KEY: &str = "core::item_state::equipped_by";
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StoredIn;

impl StoredIn {
    pub const KEY: &str = "core::item_state::stored_in";
}

#[derive(Component, Clone, Default)]
pub struct Item {
    pub key: ItemKey,
    pub label: ItemLabel,
    pub footprint: ItemFootprint,
}

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct ItemKey(pub String);

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug)]
pub struct ItemLabel(pub String);

#[derive(Clone, PartialEq, Copy, Eq, Hash, Debug)]
pub struct ItemFootprint(pub UVec2);

impl Default for ItemFootprint {
    fn default() -> Self {
        Self(UVec2::ONE)
    }
}
