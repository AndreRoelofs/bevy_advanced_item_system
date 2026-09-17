use bevy::prelude::*;
use bevy_enhanced_input::prelude::{
    Action, InputAction, InputContextAppExt, Press, actions, bindings,
};

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<GameplayInput>();
    }
}

#[derive(Component)]
pub struct GameplayInput;

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
