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

![Weapon pickup showing the gun mesh and the colored pad beneath it.](https://media.githubusercontent.com/media/AndreRoelofs/bevy_advanced_item_system/refs/heads/main/website/src/assets/pick-up.png)

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

## What inventory contains

An inventory holds the concrete items, not their recipes. If we carry two Magnums, the inventory holds two different `ULyraInventoryItemInstance` objects. Each Magnum has its own ammunition count.

`FLyraInventoryList` holds all the items in the inventory. Each entry contains an item instance and its quantity:

```cpp
struct FLyraInventoryList : public FFastArraySerializer
{
public:
    // Reference to the player who owns this inventory
    TObjectPtr<UActorComponent> OwnerComponent;
    
    // The items themselves
    TArray<ULyraInventoryItemInstance> Entries;
};
```

# --- Human written above ---

## Equipped items

Having a Magnum in inventory does not put a gun model in the player's hand or give them the ability to shoot it. Equipping handles that part. Lyra uses another recipe, `ULyraEquipmentDefinition`, to describe what to create when the item is equipped:

```cpp
class ULyraEquipmentDefinition : public UObject
{
public:
    // The class of the runtime equipment object to create
    TSubclassOf<ULyraEquipmentInstance> InstanceType;

    // Actions to give the player, such as firing the gun
    TArray<TObjectPtr<const ULyraAbilitySet>> AbilitySetsToGrant;

    // Actors to spawn and attach, including the gun model
    TArray<FLyraEquipmentActorToSpawn> ActorsToSpawn;
};
```

How does the Magnum item definition know which equipment recipe to use? Through another fragment in its `Fragments` array:

```cpp
class UInventoryFragment_EquippableItem : public ULyraInventoryItemFragment
{
public:
    // The equipment recipe for this item type
    TSubclassOf<ULyraEquipmentDefinition> EquipmentDefinition;
};
```

When the player selects the Magnum in the quick bar, Lyra reads this fragment and asks `ULyraEquipmentManagerComponent` to equip it:

```cpp
class ULyraEquipmentManagerComponent : public UPawnComponent
{
public:
    // Create equipment from its recipe
    ULyraEquipmentInstance* EquipItem(
        TSubclassOf<ULyraEquipmentDefinition> EquipmentDefinition);

    // Remove the equipment, its actors, and its granted abilities
    void UnequipItem(ULyraEquipmentInstance* ItemInstance);
};
```

The result is a `ULyraEquipmentInstance`. This is not a replacement for the inventory item. It is another object that keeps track of the equipped representation. The quick bar sets its `Instigator` to the particular Magnum in our inventory:

```cpp
class ULyraEquipmentInstance : public UObject
{
public:
    UObject* GetInstigator() const { return Instigator; }
    void SetInstigator(UObject* InInstigator) { Instigator = InInstigator; }

private:
    // In the quick-bar flow, points back to our inventory item
    TObjectPtr<UObject> Instigator;

    // The actors created for the equipped item
    TArray<TObjectPtr<AActor>> SpawnedActors;
};
```

If we fire a shot and then switch away from the Magnum, its equipment is removed, but its inventory instance stays. Equipping it again creates new equipment pointing back to the same item, which still has `5` bullets. In Lyra, an equipped gun is therefore still an inventory item.

### Accessing the item from an ability

The firing ability needs to know which Magnum is being fired so it can use that gun's ammunition and condition. `ULyraGameplayAbility_FromEquipment` provides the connection:

```cpp
class ULyraGameplayAbility_FromEquipment : public ULyraGameplayAbility
{
public:
    // The equipment that granted this ability
    ULyraEquipmentInstance* GetAssociatedEquipment() const;

    // The inventory item referenced by that equipment's Instigator
    ULyraInventoryItemInstance* GetAssociatedItem() const;
};
```

For our Magnum, `GetAssociatedItem()` returns the instance holding its remaining ammunition. It does not return the shared definition or another Magnum that happens to be in the same inventory.

## World pickups

The gun we see on the ground is a world actor, not the inventory data object itself. Its mesh and pad can use the pickup fragment shown in Figure 1. What the player receives when collecting it is described separately.

Lyra distinguishes a pickup that creates a new item from a recipe from one that carries an existing item:

```cpp
struct FPickupTemplate
{
    // How many copies to create
    int32 StackCount = 1;

    // The recipe, such as the Magnum definition
    TSubclassOf<ULyraInventoryItemDefinition> ItemDef;
};

struct FPickupInstance
{
    // A particular item, with its existing ammunition and condition
    TObjectPtr<ULyraInventoryItemInstance> Item = nullptr;
};

struct FInventoryPickup
{
    // Existing items offered by this pickup
    TArray<FPickupInstance> Instances;

    // Recipes for fresh items offered by this pickup
    TArray<FPickupTemplate> Templates;
};
```

An actor or component uses `IPickupable` to expose what it offers. The pickup logic reads this list and adds its entries to the inventory:

```cpp
class IPickupable
{
public:
    // What the player receives when collecting this pickup
    virtual FInventoryPickup GetPickupInventory() const = 0;
};
```

A fresh Magnum can come from a template. A Magnum we dropped with `5` bullets must keep its existing data instead of starting over from the recipe.

There is an important gap in Lyra 5.3: the inventory-list operation for adding an existing item instance is unfinished:

```cpp
void FLyraInventoryList::AddEntry(ULyraInventoryItemInstance* Instance)
{
    unimplemented();
}
```

Keeping those `5` bullets through a drop and pickup is therefore not automatic in this version. We need to complete the existing-item transfer, or explicitly copy the item's changing data into the new instance.
