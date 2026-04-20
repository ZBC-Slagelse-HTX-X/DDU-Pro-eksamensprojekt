use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self (Vec2 { x: 0., y: 0. })
    }
}

#[derive(Component)]
pub struct Acceleration(pub Vec2);

impl Default for Acceleration {
    fn default() -> Self {
        Self (Vec2 { x: 0., y: 0. })
    }
}

#[derive(Resource, Default, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Keyboard,
    Controller
}

pub fn change_inputmode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>,
) {
    match *mode {
        crate::movement::InputMode::Keyboard => {
            if let Some(gamepad) = gamepad {
                *mode = crate::movement::InputMode::Controller;
            } 
        },
        crate::movement::InputMode::Controller => {
            if gamepad.is_none() {
                *mode = crate::movement::InputMode::Keyboard;
            } 
        }
    }
}
