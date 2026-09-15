use bevy::prelude::*;

mod components;
mod magnum;
mod rifle;
mod view;

pub use components::*;
pub use magnum::*;
pub use rifle::*;
pub use view::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OnGround>()
            .register_type::<EquippedBy>()
            .register_type::<StoredIn>()
            .register_type::<Stores>();

        app.world_mut()
            .register_mutually_exclusive_components::<(OnGround, EquippedBy, StoredIn)>();

        app.add_plugins((
            ItemComponentsPlugin,
            MagnumPlugin,
            RiflePlugin,
            ItemViewPlugin,
        ));
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatOp {
    Flat(f32),
    Mult(f32),
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
#[require(Transform)]
pub struct OnGround;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Equips)]
pub struct EquippedBy(pub Entity);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = EquippedBy)]
pub struct Equips(Vec<Entity>);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Stores)]
pub struct StoredIn(pub Entity);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = StoredIn)]
pub struct Stores(Vec<Entity>);

impl Stores {
    pub fn items(&self) -> &Vec<Entity> {
        &self.0
    }
}

impl Equips {
    pub fn items(&self) -> &Vec<Entity> {
        &self.0
    }
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
