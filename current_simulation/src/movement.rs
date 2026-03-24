use bevy::prelude::*;

struct Position(Vec2);

pub struct Acceleration(pub Vec2);

struct Velocity(Vec2);

fn velocity_from_acceleration(acceleration: Acceleration, delta: f32) -> Velocity {
    let velocity_vector: Vec2 = acceleration.0 * delta;
    return Velocity(velocity_vector);
}

fn new_postion(former_position: Position, velocity: Velocity, delta: f32) -> Position {
    let change_in_position_vector: Vec2 = velocity.0 * delta;
    let new_position: Vec2 = former_position.0 + change_in_position_vector;
    return Position(new_position);
}
