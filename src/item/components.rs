use bevy::prelude::*;

mod rusty;
mod shootable;

pub use rusty::*;
pub use shootable::*;

pub struct ItemComponentsPlugin;

impl Plugin for ItemComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ShootablePlugin, RustyPlugin));
    }
}

// TODO: maybe move somewhere else
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cooldown(pub f32);
