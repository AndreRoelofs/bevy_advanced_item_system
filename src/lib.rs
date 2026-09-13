use bevy::prelude::*;

mod item;
mod views;

pub use item::*;
pub use views::*;

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, ViewsPlugin, ItemPlugin))
        .run();
}
