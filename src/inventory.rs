use bevy::prelude::*;

use crate::{Item, OnGround, Player, StoredIn};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Inventory>()
            .add_systems(Update, pick_up_close);
    }
}

#[derive(Reflect, Clone, Default)]
pub struct InventorySize(pub UVec2);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Inventory {
    items: Vec<Entity>,
    // TODO: Maybe we just go with list?
    /// Controls packing, Tarkov-style
    cells: Vec<Entity>,
    size: InventorySize,
}

impl Inventory {
    pub fn size(&self) -> InventorySize {
        self.size.clone()
    }
}

fn pick_up_close(
    _inventory: Single<&Inventory, With<Player>>,
    items: Query<Entity, (With<Item>, With<OnGround>)>,
    mut commands: Commands,
) {
    for item in items {
        commands.entity(item).insert(StoredIn);
    }
}
