use bevy::{platform::collections::HashMap, prelude::*, scene::ScenePatch};

use crate::ItemKey;

pub struct ItemViewPlugin;

impl Plugin for ItemViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemViewRegistry>();
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

pub fn build_chrome_patch(
    asset_server: &AssetServer,
    patches: &mut Assets<ScenePatch>,
    scene: impl Scene,
) -> Handle<ScenePatch> {
    let mut patch = ScenePatch::load(asset_server, scene);
    patch
        .resolve(asset_server, patches)
        .expect("chrome scenes have no asset-path dependencies");
    patches.add(patch)
}
