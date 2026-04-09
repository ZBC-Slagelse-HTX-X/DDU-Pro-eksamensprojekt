use current_simulator::*;
use bevy::prelude::*;

#[test]
fn test_simulation() {
    use std::f32;
    let mut test_map = Map::new(15., 10.);
    for i in 0..32 {
        let a = i as f32 * f32::consts::FRAC_PI_8;
        test_map.initial_velocity = current_simulator::movement::Velocity(Vec2::new(a.cos(), a.sin()));
        test_map.bake_simulation();
    }
}
