use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct Position(pub Vec2);

pub struct Acceleration(pub Vec2);

#[derive(Copy, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Copy, Clone)]
pub struct VelocityPosition {
    position: Position,
    velocity: Velocity
}
impl VelocityPosition {
    pub fn new(velocity: Velocity, position: Position) -> Self {
        Self { position, velocity}
    }
    pub fn step(&self, step: f32) -> Self {
        Self {position: new_position(self.position.clone(), self.velocity, step), velocity: self.velocity}
    }
}

fn velocity_from_acceleration(acceleration: Acceleration, delta: f32) -> Velocity {
    let velocity_vector: Vec2 = acceleration.0 * delta;
    return Velocity(velocity_vector);
}

fn new_position(former_position: Position, velocity: Velocity, delta: f32) -> Position {
    let change_in_position_vector: Vec2 = velocity.0 * delta;
    let new_position: Vec2 = former_position.0 + change_in_position_vector;
    return Position(new_position);
}
