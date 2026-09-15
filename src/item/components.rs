use bevy::prelude::*;

mod cooldown;
mod rusty;
mod shootable;
mod grounded_secs;

pub use cooldown::*;
pub use rusty::*;
pub use shootable::*;
pub use grounded_secs::*;

pub struct ItemComponentsPlugin;

impl Plugin for ItemComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ShootablePlugin, RustyPlugin, GroundedSecsPlugin));
    }
}
