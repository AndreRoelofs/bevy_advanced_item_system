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

`UInventoryFragment_Rustable` can now be a part of the ItemDefinition system that tells the game how an item's `Instance` should be constructed. During gameplay we also want to add a `Rusty` tag to an item instance, but only after it has spent 5 seconds on the ground. Where does `Rusty` fit?

## `Rusty` fits into the square hole!

That's right. Much like every other tag, `Rusty` and it's required counterpart `GroundedSecs` fit into the `StatTags` array that an `ItemInstance` contains. The reason why `StatTags` can be seen as a square hole that fits any shape is that fundamental parts of the gunplay like `MagazineAmmo` and `RemainingMagazines` also sit in `StatTags` alongside our newly added components. In fact every dynamic property of an `ItemInstance` is a part of String-to-Int nature of `StatTags`. This allows the system to support arbitrary functionalities of any item at the cost of type safety and code verbosity.

`Rusty` will be defined as:

```cpp
UE_DEFINE_GAMEPLAY_TAG_STATIC(TAG_Rusty, "Item.Condition.Rusty");
```

As `StatTags` is the equivalent of `HashMap<String, int32>`, we will express a gun having rust as `HashMap<"Item.Condition.Rusty", 1>`. 

`GroundedSecs` in turn is just:

```cpp
UE_DEFINE_GAMEPLAY_TAG_STATIC(TAG_GroundedSecs, "Item.Stat.GroundedSecs");
```

The `int32` part of the `GroundedSecs` tag counts seconds that the gun has spent lying on the ground. Once that counter reaches 5 or more - we switch the value of `ItemCondition.Rusty` to `1`.

## `Rusty` should affect gameplay

An item left too long on the ground should shoot slower and look worse. In technical terms, this means that multiple independent systems should be able to consume and react to `Rusty`. Preferably in a way that is generalizable and allows for other stat modifiers to be added later and work alongside all the old ones. For now let's consider only the gameplay aspect.

First let's create a `GetEffectiveFireDelay` function that would calculate the the effects various tags can have on the final cooldown:

```cpp
double ULyraGameplayAbility_RangedWeapon::GetEffectiveFireDelay(
    double BaseDelaySeconds) const
{
    double FlatAdjustment = 0.0;
    double Multiplier = 1.0;

    if (const ULyraInventoryItemInstance* Item = GetAssociatedItem())
    {
        if (Item->HasStatTag(LyraGameplayTags::Item_Condition_Rusty))
        {
            if (const auto* Rust =
                Item->FindFragmentByClass<UInventoryFragment_Rustable>())
            {
                Multiplier *= FMath::Max(1.0f, Rust->CooldownMultiplier);
            }
        }
    }

    return (BaseDelaySeconds + FlatAdjustment) * Multiplier;
```

Various effects in our game can have both a `Flat` and a `Mult` consequence for the final timer until we can fire our next shot. The eagle-eyed ones might already see a potential issue; what if the game grows? What if there are 10 different conditions that affect an item's performance? Something along the lines of:

```cpp
double ULyraGameplayAbility_RangedWeapon::GetEffectiveFireDelay(
    double BaseDelaySeconds) const
{
    double FlatAdjustment = 0.0;
    double Multiplier = 1.0;

    if (const ULyraInventoryItemInstance* Item = GetAssociatedItem())
    {
        if (Item->HasStatTag(LyraGameplayTags::Item_Condition_Rusty))
        {
            if (const auto* Rust =
                Item->FindFragmentByClass<UInventoryFragment_Rustable>())
            {
                Multiplier *= Rust->CooldownMultiplier;
            }
        }

        if (Item->HasStatTag(LyraGameplayTags::Item_Condition_QualityRare))
        {
            if (const auto* Quality =
                Item->FindFragmentByClass<UInventoryFragment_QualityRare>())
            {
                Multiplier *= Quality->CooldownMultiplier;
            }
        }

        if (Item->HasStatTag(LyraGameplayTags::Item_Condition_TunedAction))
        {
            if (const auto* TunedAction =
                Item->FindFragmentByClass<UInventoryFragment_TunedAction>())
            {
                FlatAdjustment += TunedAction->CooldownAdjustmentSeconds;
            }
        }
        
        // ... + 7 more
    }

    return (BaseDelaySeconds + FlatAdjustment) * Multiplier;
}
```

We have now created an unwieldy beast of a function. Not to mention that our code is now littered with countless template definitions for various `UInventoryFragment` classes that we decided to create for our game.

# TODO: next talk about implementation of cooldown impact and visual tinting of Rusted items

# --- Human written above ---

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
