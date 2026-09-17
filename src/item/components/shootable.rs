use bevy::prelude::*;
use bevy_enhanced_input::prelude::Fire;

use crate::{Cooldown, EquippedBy, player::input::Shoot};

#[derive(Component, Clone, Default)]
pub struct Shootable {
    pub magazine_size: u32,
}

#[derive(Component, Clone, Default)]
pub struct Burst {
    pub shots: u32,
    pub interval: f32,
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
            .add_observer(on_shoot)
            .add_systems(Update, fire_shots)
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
    cooldown: &Cooldown,
    ammo: &Ammo,
    last_shot: Option<&LastShotAt>,
) -> FireOutcome {
    if ammo.0 == 0 {
        return FireOutcome::Empty;
    }
    let cooling = last_shot.is_some_and(|last| now_secs - last.0 < cooldown.effective());
    if cooling {
        return FireOutcome::Cooldown;
    }
    FireOutcome::Fired
}

fn on_shoot(
    _shoot: On<Fire<Shoot>>,
    time: Res<Time>,
    mut guns: Query<
        (
            &mut FireControl,
            &Ammo,
            &Cooldown,
            Option<&LastShotAt>,
            Option<&Burst>,
        ),
        With<EquippedBy>,
    >,
) {
    let Ok((mut control, ammo, cooldown, last_shot, burst)) = guns.single_mut() else {
        return;
    };

    if control.remaining != 0
        || try_fire(time.elapsed_secs(), cooldown, ammo, last_shot) != FireOutcome::Fired
    {
        return;
    }

    control.remaining = burst.map_or(1, |burst| burst.shots).min(ammo.0);
    control.next_at = time.elapsed_secs();
}

fn fire_shots(
    time: Res<Time>,
    mut guns: Query<(Entity, &mut FireControl, &mut Ammo, Option<&Burst>), With<EquippedBy>>,
    mut shots: MessageWriter<ShotFired>,
    mut commands: Commands,
) {
    for (gun, mut control, mut ammo, burst) in &mut guns {
        while control.remaining > 0 && time.elapsed_secs() >= control.next_at {
            ammo.0 -= 1;
            control.remaining -= 1;
            control.next_at += burst.map_or(0.0, |burst| burst.interval);
            commands.entity(gun).insert(LastShotAt(time.elapsed_secs()));
            shots.write(ShotFired { gun });
        }
    }
}

fn arm_on_equip(
    insert: On<Insert, EquippedBy>,
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
    remove: On<Remove, EquippedBy>,
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
