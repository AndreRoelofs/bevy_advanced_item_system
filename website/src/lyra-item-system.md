# Lyra's item system

Lyra has a way to separate the nature of an item from how it is expressed during a state.

## Shared item definitions

`ULyraInventoryItemDefinition` describes a specific item type, such as the Magnum pistol. The magnum in each player's hand has a reference to this one definition.

```cpp
class ULyraInventoryItemDefinition : public UObject
{
public:
    // Just a name like "ID_Magnum"
    FText DisplayName;
    
    // You can see fragments as simpler version of Bevy's Components
    TArray<TObjectPtr<ULyraInventoryItemFragment>> Fragments;
};
```

Each definition contains an array of `ULyraInventoryItemFragment` called `Fragments`. Despite having `Inventory` in their name, `ULyraInventoryItemFragment`s are used everywhere to decide properties of the item in question. To illustrate this further, consider `UInventoryFragment_PickupIcon` below:

```cpp
class UInventoryFragment_PickupIcon : public ULyraInventoryItemFragment
{
public:
    // The model of the gun
    TObjectPtr<USkeletalMesh> SkeletalMesh;
    
    // The color of the pad underneath
    FLinearColor PadColor;
};
```

When a gun is on the ground and ready to be picked up, it is displayed in the world using the properties defined in `UInventoryFragment_PickupIcon` as shown in Figure 1.

![Weapon pickup showing the gun mesh and the colored pad beneath it.](assets/pick-up.png)

*Figure 1: Weapon pickup showing the gun mesh and the colored pad beneath it.*

## Dynamic properties of an item

Item definition in Lyra can be seen as the recipe to create an item. `ULyraInventoryItemInstance` on the other hand is an expression of a concrete item in the world. Two different players can have the same gun equipped. The recipe, or the definition, of these two guns is going to be the same. But their `Instance` will be different. 

```cpp
class ULyraInventoryItemInstance : public UObject
{
public:
    // Dynamic aspects of a concrete item existing in the world
    FGameplayTagStackContainer StatTags;
    
    // A reference to the recipe of the item, shared between
    // all instances.
    TSubclassOf<ULyraInventoryItemDefinition> ItemDef;
};
```

Every item in the `StatTags` property holds a `FName TagName` and `int32 StackCount`. Rust equivalent of `StatTags` is something like `HashMap<String, i32>`.

For ammunition, the tag reads `TagName = "Lyra.ShooterGame.Weapon.Ammo"` with a count of `6` for our Magnum revolver. `5` after we take a shot and so on. Since the `StatTags` are persistent, that means that shooting a bullet from your revolver, dropping it on the ground and picking it back up again results in you still having `5` ammo instead of the original `6`.
