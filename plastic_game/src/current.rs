use bevy::prelude::*;
use std::ops;

#[derive(Component)]
pub struct Puller;

#[derive(Component, PartialEq)]
pub struct Position(Vec2);

impl ops::Add for Position {
    type Output = Vec2;
    fn add(self, rhs: Position) -> Self::Output {
        self.0 + rhs.0
    }
}

impl ops::Sub for Position {
    type Output = Vec2;
    fn sub(self, rhs: Position) -> Self::Output {
        self.0 - rhs.0
    }
}

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Acceleration(Vec2);

fn calculate_acceleration_vector(puller_pos: Position, start_pos: Position, weight: f32) {
    debug_assert!(puller_pos != start_pos, "Positions should not be identical");
    let difference_vector = puller_pos - start_pos;
    let direction_vector = Dir2::new_unchecked(difference_vector);
}

