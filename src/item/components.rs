use bevy::prelude::*;

mod cooldown;
mod grounded_secs;
mod rusty;
mod shootable;

pub use cooldown::*;
pub use grounded_secs::*;
pub use rusty::*;
pub use shootable::*;

pub struct ItemComponentsPlugin;

impl Plugin for ItemComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ShootablePlugin, RustyPlugin, GroundedSecsPlugin));
    }
}
