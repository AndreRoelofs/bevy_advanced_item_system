use bevy::{platform::collections::HashMap, prelude::*, scene::ScenePatch};

use crate::ItemKey;

pub struct ItemViewPlugin;

impl Plugin for ItemViewPlugin {
    fn build(&self, _app: &mut App) {}
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
