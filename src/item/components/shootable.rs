use bevy::prelude::*;

use crate::{Cooldown, EquippedBy};

#[derive(Component, Clone, Default)]
pub struct Shootable {
    pub cooldown: Cooldown,
    pub magazine_size: u32,
}

#[derive(Component, Clone, Default)]
pub struct Burst {
    pub shots: u32,
    pub interval: Cooldown,
}

#[derive(Component, Clone, Default)]
pub struct FireControl {
    pub remaining: u32,
    pub next_at: f32,
}

#[derive(Component, Clone, Default)]
pub struct Ammo(pub u32);

#[derive(Component, Clone)]
pub struct LastShotAt(pub f32);

#[derive(Message)]
pub struct ShotFired {
    pub gun: Entity,
}

pub struct ShootablePlugin;

impl Plugin for ShootablePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ShotFired>()
            .add_observer(arm_on_equip)
            .add_observer(disarm_on_unequip);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FireOutcome {
    Fired,
    Cooldown,
    Empty,
}

pub fn try_fire(
    now_secs: f32,
    cooldown: Cooldown,
    ammo: &Ammo,
    last_shot: Option<&LastShotAt>,
) -> FireOutcome {
    if ammo.0 == 0 {
        return FireOutcome::Empty;
    }
    let cooling = last_shot.is_some_and(|last| now_secs - last.0 < cooldown.0);
    if cooling {
        return FireOutcome::Cooldown;
    }
    FireOutcome::Fired
}

fn arm_on_equip(
    insert: On<Insert<EquippedBy>>,
    guns: Query<Has<FireControl>, With<Shootable>>,
    mut commands: Commands,
) {
    let gun = insert.event().entity;
    let Ok(armed) = guns.get(gun) else {
        return;
    };
    if armed {
        return;
    }
    if let Ok(mut gun) = commands.get_entity(gun) {
        gun.try_insert(FireControl::default());
    }
}

fn disarm_on_unequip(
    remove: On<Remove<EquippedBy>>,
    guns: Query<(), With<Shootable>>,
    mut commands: Commands,
) {
    let gun = remove.event().entity;
    if guns.get(gun).is_err() {
        return;
    }
    if let Ok(mut gun) = commands.get_entity(gun) {
        gun.remove::<FireControl>();
    }
}
