use bevy::prelude::*;
use bevy_enhanced_input::prelude::{
    Action, Fire, InputAction, InputContextAppExt, Press, actions, bindings,
};

use crate::{ItemCommandsExt, Magnum, Rifle, StoredIn};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<GameplayInput>()
            .add_observer(on_select_magnum)
            .add_observer(on_select_rifle);
    }
}

/// Shared player context, independent of the equipped weapon.
#[derive(Component)]
pub struct GameplayInput;

/// Pull the trigger; the equipped weapon will determine the firing pattern.
#[derive(InputAction)]
#[action_output(bool)]
pub struct Shoot;

#[derive(InputAction)]
#[action_output(bool)]
pub struct SelectMagnum;

#[derive(InputAction)]
#[action_output(bool)]
pub struct SelectRifle;

pub fn gameplay_input() -> impl Bundle {
    (
        GameplayInput,
        actions!(GameplayInput[
            (
                Action::<Shoot>::new(),
                Press::default(),
                bindings![MouseButton::Left],
            ),
            (
                Action::<SelectMagnum>::new(),
                Press::default(),
                bindings![KeyCode::Digit1],
            ),
            (
                Action::<SelectRifle>::new(),
                Press::default(),
                bindings![KeyCode::Digit2],
            ),
        ]),
    )
}

fn on_select_magnum(
    select: On<Fire<SelectMagnum>>,
    items: Query<Entity, (With<StoredIn>, With<Magnum>)>,
    mut commands: Commands,
) {
    let Ok(item) = items.single() else {
        return;
    };

    commands.entity(item).equip_for(select.context);
}

fn on_select_rifle(
    select: On<Fire<SelectRifle>>,
    items: Query<Entity, (With<StoredIn>, With<Rifle>)>,
    mut commands: Commands,
) {
    let Ok(item) = items.single() else {
        return;
    };

    commands.entity(item).equip_for(select.context);
}
