use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct Position(pub Vec2);

pub struct Acceleration(pub Vec2);

#[derive(Copy, Clone)]
pub struct Velocity(pub Vec2);

#[derive(Copy, Clone)]
pub struct VelocityPosition {
    pub position: Position,
    pub velocity: Velocity
}
impl VelocityPosition {
    pub fn new(velocity: Velocity, position: Position) -> Self {
        Self { position, velocity}
    }
    pub fn step(&self, acceleration: Acceleration, step: f32) -> Self {
        let new_velocity = Velocity(self.velocity.0 + acceleration.0 * step);
        let new_position = Position(self.position.0 + new_velocity.0 * step);
        Self { position: new_position, velocity: new_velocity }
    }
}

fn new_position(former_position: Position, velocity: Velocity, delta: f32) -> Position {
    let change_in_position_vector: Vec2 = velocity.0 * delta;
    let new_position: Vec2 = former_position.0 + change_in_position_vector;
    return Position(new_position);
}
