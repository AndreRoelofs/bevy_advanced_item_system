use bevy::prelude::*;

use crate::{Item, OnGround, Player, StoredIn};

const PICKUP_RANGE: f32 = 2.0;

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
    mut player: Query<(Entity, &mut Inventory, &Transform), With<Player>>,
    items: Query<(Entity, &Transform), (With<Item>, With<OnGround>)>,
    mut commands: Commands,
) {
    let Ok((entity, mut inventory, pos)) = player.single_mut() else {
        return;
    };

    for (item, item_pos) in items {
        if pos
            .translation
            .abs_diff_eq(item_pos.translation, PICKUP_RANGE)
        {
            inventory.items.push(item);
            commands.entity(item).insert(StoredIn(entity));
        }
    }
}
