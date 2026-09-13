use bevy::{platform::collections::HashMap, prelude::*, scene::ScenePatch};

use crate::{EquippedBy, Item, ItemKey, OnGround, StoredIn, View, ViewOf};

pub struct ItemViewPlugin;

impl Plugin for ItemViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemViewRegistry>()
            .add_observer(change_view_on_equipped)
            .add_observer(change_view_on_stored)
            .add_observer(change_view_on_ground);
    }
}

#[derive(Default)]
pub struct ItemViewDefinition {
    // TODO: The string should be something else I think
    pub chrome: HashMap<String, Handle<ScenePatch>>,
}

#[derive(Resource, Default)]
pub struct ItemViewRegistry(HashMap<ItemKey, ItemViewDefinition>);

impl ItemViewRegistry {
    pub fn get(&self, key: &ItemKey) -> Option<&ItemViewDefinition> {
        self.0.get(key)
    }

    pub fn register(&mut self, key: ItemKey, view: ItemViewDefinition) {
        self.0.insert(key, view);
    }
}

pub fn build_chrome_patch(asset_server: &AssetServer, scene: impl Scene) -> Handle<ScenePatch> {
    asset_server.add(ScenePatch::load(asset_server, scene))
}

fn change_view_on_ground(
    add: On<Add<OnGround>>,
    mut commands: Commands,
    views: Res<ItemViewRegistry>,
    items: Query<&Item>,
) {
    let Ok(item) = items.get(add.entity) else {
        return;
    };

    commands.entity(add.entity).despawn_related::<View>();

    let Some(ground) = views
        .get(&item.key)
        .and_then(|definition| definition.chrome.get(OnGround::KEY))
    else {
        return;
    };

    commands.spawn((
        ViewOf(add.entity),
        // TODO: Make this point to a hand or something, for now this reads super
        // weird. But it's better than nothing.
        ChildOf(add.entity),
        // Queues the spawning of the view for the next tick
        ScenePatchInstance(ground.clone()),
        Transform::default(),
        Visibility::default(),
    ));
}

fn change_view_on_equipped(
    add: On<Add<EquippedBy>>,
    mut commands: Commands,
    views: Res<ItemViewRegistry>,
    items: Query<&Item>,
) {
    let Ok(item) = items.get(add.entity) else {
        return;
    };

    commands.entity(add.entity).despawn_related::<View>();

    let Some(ground) = views
        .get(&item.key)
        .and_then(|definition| definition.chrome.get(EquippedBy::KEY))
    else {
        return;
    };

    commands.spawn((
        ViewOf(add.entity),
        // TODO: Make this point to a hand or something, for now this reads super
        // weird. But it's better than nothing.
        ChildOf(add.entity),
        // Queues the spawning of the view for the next tick
        ScenePatchInstance(ground.clone()),
        Transform::default(),
        Visibility::default(),
    ));
}

fn change_view_on_stored(
    add: On<Add<StoredIn>>,
    mut commands: Commands,
    views: Res<ItemViewRegistry>,
    items: Query<&Item>,
) {
    let Ok(item) = items.get(add.entity) else {
        return;
    };

    commands.entity(add.entity).despawn_related::<View>();

    let Some(ground) = views
        .get(&item.key)
        .and_then(|definition| definition.chrome.get(StoredIn::KEY))
    else {
        return;
    };

    commands.spawn((
        ViewOf(add.entity),
        // TODO: Make this point to a hand or something, for now this reads super
        // weird. But it's better than nothing.
        ChildOf(add.entity),
        // Queues the spawning of the view for the next tick
        ScenePatchInstance(ground.clone()),
        Transform::default(),
        Visibility::default(),
    ));
}
