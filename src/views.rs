use bevy::prelude::*;

pub struct ViewsPlugin;

impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<View>().register_type::<ViewOf>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship(relationship_target = View)]
pub struct ViewOf(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[relationship_target(relationship = ViewOf, linked_spawn)]
pub struct View(Entity);

impl View {
    pub fn entity(&self) -> Option<Entity> {
        (self.0 != Entity::PLACEHOLDER).then_some(self.0)
    }
}
