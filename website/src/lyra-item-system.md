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
    FText DisplayName;
    TObjectPtr<USkeletalMesh> SkeletalMesh;
};
```

When a gun is on the ground and ready to be picked up, it is displayed in the world using the properties defined above in `UInventoryFragment_PickupIcon`. `DisplayName` is text that the player sees above the object and `SkeletalMesh` is the actual model of the gun in question.

TODO: maybe show a picture
