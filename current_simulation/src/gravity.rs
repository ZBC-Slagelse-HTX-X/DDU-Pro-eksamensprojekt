use bevy::prelude::*;

fn gravitational_acceleration(puller_mass: f32, difference_position: Vec2) -> crate::movement::Acceleration {
    let acceleration_vector: Vec2 = puller_mass/(difference_position.length_squared()) * difference_position.normalize_or(Vec2::ONE);
    return crate::movement::Acceleration(acceleration_vector);
    
}
