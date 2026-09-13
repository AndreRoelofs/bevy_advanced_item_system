use bevy::prelude::*;

mod item;

pub use item::*;

pub fn run() {
    App::new().add_plugins((DefaultPlugins, ItemPlugin)).run();
}
