// https://rust-lang.github.io/api-guidelines/
use bevy::prelude::*;

mod movement;
mod gravity;

struct Map {
    width: f32,
    height: f32,
    pub wrap_around: bool,
    pub initial_velocity: Vec2,
    pub simulation_density: u16,
    pub step: f32
}

impl Default for Map {
    fn default() -> Self {
        Self {width: 0., height: 0., wrap_around: false, initial_velocity: Vec2::new(0., 0.), simulation_density: 10, step: 1.}
    }
}

impl Map {
    fn new(width: f32, height: f32) -> Self {
        Self {width, height, ..default()}
    }

}

