use bevy::prelude::*;

pub struct Attractor {
    pub mass: f32,
    pub position: crate::movement::Position
}

impl Attractor {
    pub fn new(mass: f32, position: crate::movement::Position) -> Self {
        return Self{mass, position};
    }
}

pub fn gravitational_acceleration(puller_mass: f32, difference_position: Vec2) -> crate::movement::Acceleration {
    const SOFTENING: f32 = 0.001;
    let direction = difference_position.normalize_or(Vec2::ONE);
    let acceleration_vector: Vec2 = -puller_mass/(difference_position.length_squared() + SOFTENING) * direction;
    return crate::movement::Acceleration(acceleration_vector);    
}
