# Rusty items in Unreal Engine Lyra

This page implements the [Rusty mechanic](game-mechanics.md).

## ItemDefinition and `Rustable` fragment 

Let's say some of our items-in particular guns-should become worse after they spend an extended amount of time on the ground. To accomplish this, we need to create a fragment:

```cpp
class UInventoryFragment_Rustable : public ULyraInventoryItemFragment
{
public:
    // Cumulative seconds on the ground before rust appears
    float RustAfterSecs = 5.0f;

    // Rust doubles the time between shots
    float CooldownMultiplier = 2.0f;

    // Brown tint for the gun and inventory background (sRGB)
    FColor RustColor = FColor(115, 56, 20);
};
```

Only items that have `Rustable` in the definition are part of the oxidation system.

--- Human written above ---

Put rust configuration in a custom definition fragment: threshold, cooldown
multiplier, and visual settings. Keep changing values in a custom per-item record
associated with the inventory instance:

| Data | Purpose |
| --- | --- |
| `GroundedSecs` (`float`) | Accumulated ground exposure. |
| `bRusty` (`bool`) | Whether to apply rust's gameplay and visual effects. |
| Location state | Determines whether the ground counter should run. |

These are custom additions, not built-in Lyra fields. Extend the pickup/drop
flow to preserve this record or explicitly copy it between the world pickup and
inventory instance. Creating an item from its definition alone would lose its
accumulated exposure and rust.

While equipped, read the record through the equipment's inventory-item reference
rather than keeping a separate copy. Recreating an equipment actor must not reset
the condition.

## Update `GroundedSecs`

Use a custom pickup component's server-side tick to add `DeltaSeconds` to
`GroundedSecs`. Enable updates only while the item is in the ground state and
not Rusty; disable them on pickup. Initialize the counter only for an item that
has never been grounded, not on every pickup actor spawn.

Read the threshold from the definition fragment. When the counter reaches it,
call a custom `SetRusty(true)` method to update `bRusty`, stop counter updates,
and notify gameplay and presentation listeners.

## Modify the firing cooldown

The [equipment ability base class](lyra-item-system.md#accessing-the-item-from-an-ability)
provides `GetAssociatedItem()`. Its actual implementation in Lyra 5.3 resolves the
inventory item from the equipment's instigator:

Source: `Source/LyraGame/Equipment/LyraGameplayAbility_FromEquipment.cpp`.

```cpp
ULyraInventoryItemInstance* ULyraGameplayAbility_FromEquipment::GetAssociatedItem() const
{
    if (ULyraEquipmentInstance* Equipment = GetAssociatedEquipment())
    {
        return Cast<ULyraInventoryItemInstance>(Equipment->GetInstigator());
    }
    return nullptr;
}
```

Extend the firing ability's cooldown/fire-delay calculation to read the custom
rust record from this item. Handle a null result before accessing it. Use
`bRusty` to include or exclude the configured rust multiplier in that item's
effective cooldown. The accessor above is built into Lyra; the rust record and
cooldown changes are custom additions.

Keep the base cooldown unchanged and use a single rust contribution, rather
than adding another modifier on every update or equip. Calculate it per item,
not as a character-wide penalty that could also slow other weapons.

## Update the three representations

Have pickup and equipment actors apply the configured material or material
parameter, and have the inventory widget apply the configured background tint.
Refresh them both when rust changes and when a representation is created, using
the item's current `bRusty` value. A one-time rust event would miss meshes or
widgets created after the item became Rusty.

Replicate `bRusty` through the custom item state and refresh client views from
its replication notification, so clients display the server's rust decision.
