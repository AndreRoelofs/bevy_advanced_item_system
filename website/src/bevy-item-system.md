# Item states in Bevy 0.19

This page implements the [Rusty mechanic](game-mechanics.md).

## Strongly-typed communication
An instance of an item can ever be in one of three states. It can be `OnGround`, `StoredIn` some container or `EquippedBy` some character. As an item moves between the systems, it needs to preserve dynamic properties such as `RemainingAmmo` as other tags that can be changed by the player's behavior. In [Lyra](lyra-item-system.md) this is represented by three completely different systems sharing an `ULyraInventoryItemInstance` object between these systems.

In Bevy we have no need for weakly-typed contracts between independent systems. An item is an `Entity`. The properties of the item come not from a class but from the collection of `Component`'s that the item contains.

# --- Human written above ---

## Item definitions are scenes

Lyra needs a `ULyraInventoryItemDefinition` recipe with an array of fragments. In Bevy the recipe is a scene. The Magnum is described by the components a fresh Magnum entity should start with:

```rust
impl Magnum {
    pub fn scene() -> impl Scene {
        bsn! {
            Item {
                key: {ItemKey(MAGNUM_KEY.to_string())},
                label: {ItemLabel("Magnum".to_string())},
                footprint: {ItemFootprint(UVec2 { x: 4, y: 8 })}
            }
            Shootable { magazine_size: 6 }
            Cooldown::new(0.5)
            Ammo(12) // 2 Magazines
            Visibility
        }
    }
}
```

Spawning a Magnum on the ground is one scene plus the state it starts in:

```rust
commands.queue_spawn_scene(bsn! {
    @Magnum
    OnGround
    Transform { translation: Vec3::new(0.0, PLATFORM_TOP_Y + 0.075, -3.0) }
});
```

The Rifle is built the same way. It adds a `Burst { shots: 3, interval: 0.1 }` component, and that component alone is what makes it fire in bursts. There is no `UInventoryFragment_Burst` to look up and no base class to extend.

`Item` itself stays small. `ItemKey` identifies the item type, much like the definition class in Lyra, and is used to find shared data such as the item's meshes. `ItemLabel` and `ItemFootprint` are the display name and the space it takes in an inventory.

## Dynamic properties are components

Lyra keeps a gun's changing data in `StatTags`, a `HashMap<String, i32>` on the item instance. The Bevy item keeps it in components on the entity:

```rust
#[derive(Component, Clone, Default)]
pub struct Ammo(pub u32);

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct GroundedSecs(pub f32);
```

There is no difference between a "definition" property and an "instance" property. `Shootable { magazine_size: 6 }` and `Ammo(12)` sit next to each other on the same entity. Every Magnum gets its own copy of both, so two Magnums never share ammunition.

Each property has the right type. `Ammo` is a `u32`, `GroundedSecs` is an `f32` and `Rusty` is a marker with no data at all. Compare this with Lyra, where grounded seconds had to be rounded into an `int32` stack count and a boolean condition was written as a count of `1`.

## Three states as relationships

`OnGround` is a plain marker. `EquippedBy` and `StoredIn` are [relationships](https://docs.rs/bevy/0.19/bevy/ecs/relationship/index.html). They point at the character holding the item and the inventory containing it:

```rust
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
#[require(Transform)]
pub struct OnGround;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Equips)]
pub struct EquippedBy(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = EquippedBy)]
pub struct Equips(Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = Stores)]
pub struct StoredIn(pub Entity);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[relationship_target(relationship = StoredIn)]
pub struct Stores(Vec<Entity>);
```

Bevy maintains the other side of each relationship for us. Inserting `StoredIn(inventory)` on a Magnum adds the Magnum to that inventory's `Stores` list. Removing it, or despawning the Magnum, takes it out again. `Equips` holds a single `Entity` rather than a `Vec`, so a character can only equip one item at a time.

### Keeping the states exclusive

An item must be in exactly one of the three states. Bevy 0.19 has no mutually exclusive components yet, so the three transitions are written as `EntityCommands` extensions. Each one removes the other two states before inserting the new one:

```rust
// "We have mutually exclusive components at home"
/// Ensures exclusivity in item states.
pub trait ItemCommandsExt {
    fn store_in(&mut self, entity: Entity) -> &mut Self;
    fn equip_for(&mut self, entity: Entity) -> &mut Self;
    fn drop(&mut self) -> &mut Self;
}

impl ItemCommandsExt for EntityCommands<'_> {
    fn store_in(&mut self, entity: Entity) -> &mut Self {
        self.remove::<(OnGround, EquippedBy)>()
            .insert(StoredIn(entity))
    }

    fn equip_for(&mut self, entity: Entity) -> &mut Self {
        self.remove::<(OnGround, StoredIn)>()
            .insert(EquippedBy(entity))
    }

    fn drop(&mut self) -> &mut Self {
        self.remove::<(EquippedBy, StoredIn)>().insert(OnGround)
    }
}
```

This is a convention, not a guarantee. Nothing stops a system from inserting `OnGround` directly on an equipped gun. Once mutually exclusive components land in Bevy, `ItemPlugin` can register the three states and let the engine enforce the rule:

```rust
app.world_mut()
    .register_mutually_exclusive_components::<(OnGround, EquippedBy, StoredIn)>();
```

## Inventory

An inventory is an entity with an `Inventory` component. The player's inventory is spawned as one of the player's children:

```rust
commands
    .spawn((Camera3d::default(), Player::default(), /* ... */))
    .with_children(|parent| {
        // ...
        parent.spawn(Inventory);
    });
```

There is no `FLyraInventoryList` to keep in sync. The items in an inventory are whatever entities have `StoredIn` pointing at it, and `Stores` lists them. Two Magnums in the inventory are two entities with their own `Ammo`.

## Equipped items

Equipping an item does not create a second object. `equip_for` swaps `StoredIn` for `EquippedBy` on the same entity:

```rust
fn on_select_magnum(
    select: On<Fire<SelectMagnum>>,
    items: Query<Entity, (With<StoredIn>, With<Magnum>)>,
    mut commands: Commands,
) {
    let Ok(item) = items.single() else {
        return;
    };

    commands.entity(item).equip_for(select.context);
}
```

Since `Equips` holds one entity, equipping the Magnum takes `EquippedBy` away from the gun the player was holding. An observer catches that removal and puts the old gun back into the inventory:

```rust
fn store_unequipped(
    remove: On<Remove, EquippedBy>,
    inventory: Single<Entity, With<Inventory>>,
    mut commands: Commands,
) {
    commands
        .entity(remove.entity)
        .try_insert(StoredIn(*inventory));
}
```

Components that only make sense while equipped are added and removed with the state. `FireControl` tracks a burst in progress. It is added when a `Shootable` item gets `EquippedBy` and removed when it loses it:

```rust
fn arm_on_equip(
    insert: On<Insert, EquippedBy>,
    guns: Query<Has<FireControl>, With<Shootable>>,
    mut commands: Commands,
) {
    // ...
    gun.try_insert(FireControl::default());
}

fn disarm_on_unequip(
    remove: On<Remove, EquippedBy>,
    guns: Query<(), With<Shootable>>,
    mut commands: Commands,
) {
    // ...
    gun.remove::<FireControl>();
}
```

### Accessing the item when firing

Lyra's firing ability has to walk from the ability to its `ULyraEquipmentInstance`, cast its `Instigator` to `ULyraInventoryItemInstance` and handle `nullptr` along the way. In Bevy the equipped item *is* the gun, so the fire system asks for the equipped entity's components directly:

```rust
fn on_shoot(
    _shoot: On<Fire<Shoot>>,
    time: Res<Time>,
    mut guns: Query<
        (
            &mut FireControl,
            &Ammo,
            &Cooldown,
            Option<&LastShotAt>,
            Option<&Burst>,
        ),
        With<EquippedBy>,
    >,
) {
    let Ok((mut control, ammo, cooldown, last_shot, burst)) = guns.single_mut() else {
        return;
    };
    // ...
}
```

The `Ammo` read here is the same `Ammo` the Magnum had on the ground and in the inventory. No cast is needed, and the query only matches guns that have everything it asks for.

## Views of an item

A Lyra item needs a pickup actor on the ground, an equipment actor in the hand and a widget in the inventory. The Bevy item is always the same entity, so its appearance lives in a separate *view* entity. `ViewOf` links a view to its item:

```rust
#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = View)]
pub struct ViewOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = ViewOf, linked_spawn)]
pub struct View(Entity);
```

`linked_spawn` means despawning an item despawns its view as well.

Each item type registers one scene patch per state in the `ItemViewRegistry`, keyed by its `ItemKey`. The state is identified by its component's type path:

```rust
views.register(
    ItemKey(MAGNUM_KEY.to_string()),
    ItemViewDefinition {
        chrome: HashMap::from([
            (OnGround::type_path(), ground),
            (EquippedBy::type_path(), equipped),
            (StoredIn::type_path(), stored),
        ]),
    },
);
```

The Magnum on the ground is a cuboid, in the hand it is a sphere and in the inventory it is a UI node with a background color. The ground and hand meshes are different on purpose: a change such as rust has to show up in every representation.

One observer per state swaps the view whenever the item enters that state. For `EquippedBy`, the new view is parented to the character holding the gun:

```rust
fn change_view_on_equipped(
    add: On<Add, EquippedBy>,
    mut commands: Commands,
    views: Res<ItemViewRegistry>,
    items: Query<(&Item, &EquippedBy)>,
) {
    let Ok((item, equipped_by)) = items.get(add.entity) else {
        return;
    };

    commands.entity(add.entity).despawn_related::<View>();

    let Some(ground) = views
        .get(&item.key)
        .and_then(|definition| definition.chrome.get(EquippedBy::type_path()))
    else {
        return;
    };

    commands.spawn((
        ViewOf(add.entity),
        ChildOf(equipped_by.0),
        // Queues the spawning of the view for the next tick
        ScenePatchInstance(ground.clone()),
        Transform::from_xyz(0.3, -0.3, -0.9),
        Visibility::default(),
    ));
}
```

Views only show the item. They hold none of its data, so throwing one away and spawning another never resets ammunition or condition.

## Picking items up

Lyra's pickups choose between a fresh item from a template and an existing item instance. The existing-instance path is the one left `unimplemented()` in Lyra 5.3. Bevy has nothing to transfer. A gun on the ground is already the item, and picking it up only changes its state:

```rust
fn pick_up_close(
    players: Query<(Entity, Option<&Equips>, &Transform, &Children), With<Player>>,
    inventories: Query<Entity, With<Inventory>>,
    items: Query<(Entity, &Transform), (With<Item>, With<OnGround>)>,
    mut commands: Commands,
) {
    // ...

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
```

A Magnum that fired one shot, is dropped and is picked up again still has `Ammo(11)`. Nothing copied that number. The entity kept it.

The next page, [Rusty in Bevy](rusty-in-bevy.md), builds the Rusty mechanic on top of these states.
