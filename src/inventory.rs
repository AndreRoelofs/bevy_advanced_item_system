use bevy::prelude::*;

use crate::{Equips, Item, OnGround, Player, Shootable, Stores, item::ItemCommandsExt};

const PICKUP_RANGE: f32 = 2.0;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Inventory>()
            .register_type::<InventoryOf>()
            .register_type::<OwnsInventory>()
            .add_systems(Update, pick_up_close);
    }
}

#[derive(Reflect, Clone, Default)]
pub struct InventorySize(pub UVec2);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Inventory;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Inventory)]
#[relationship(relationship_target = OwnsInventory)]
pub struct InventoryOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(Inventory)]
#[relationship_target(relationship = InventoryOf)]
pub struct OwnsInventory(Entity);

impl OwnsInventory {
    pub fn entity(&self) -> Option<Entity> {
        (self.0 != Entity::PLACEHOLDER).then_some(self.0)
    }
}

fn pick_up_close(
    players: Query<(Entity, Option<&Equips>, &Transform, &Children), With<Player>>,
    inventories: Query<Entity, With<Inventory>>,
    items: Query<(Entity, &Transform), (With<Item>, With<OnGround>)>,
    mut commands: Commands,
) {
    let Ok((player, equips, pos, children)) = players.single() else {
        return;
    };

    let Some(inventory) = inventories.iter_many(children).next() else {
        return;
    };

    // If the player has nothing equipped then we
    // give them the first gun they walk over.
    let mut has_equip = equips.and_then(Equips::entity).is_some();

    for (item, item_pos) in items {
        if pos
            .translation
            .abs_diff_eq(item_pos.translation, PICKUP_RANGE)
        {
            if !has_equip {
                commands.entity(item).equip_for(player);
                has_equip = true;
            } else {
                commands.entity(item).store_in(inventory);
            }
        }
    }
}
