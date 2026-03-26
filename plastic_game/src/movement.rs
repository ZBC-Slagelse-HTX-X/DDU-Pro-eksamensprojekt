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
    Controller,
}
