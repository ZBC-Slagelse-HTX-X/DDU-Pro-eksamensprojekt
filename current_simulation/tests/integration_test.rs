use current_simulator::*;
use bevy::prelude::*;

fn test_different_angles() {
    use std::f32;
    let mut test_map = Map::new(15., 10.);

    for i in 0..32 {
        let a = i as f32 * f32::consts::FRAC_PI_8;
        test_map.initial_velocity = current_simulator::movement::Velocity(Vec2::new(a.cos(), a.sin()));
        test_map.bake_simulation();
    }
}

#[test]
fn test_print_guiders() {
    use std::f32;
    let width = 15.;
    let height = 10.;
    let mut test_map = Map::new(width, height);

    test_map.attractors = Some(vec![
        current_simulator::gravity::Attractor::new(1., movement::Position(Vec2::new(7.5, 5.))),
        current_simulator::gravity::Attractor::new(2., movement::Position(Vec2::new(9., 7.5)))
    ]);
    test_map.step_size = 0.5;
    test_map.iterations = 20;

    let a = 6. as f32 * f32::consts::FRAC_PI_8;

    test_map.initial_velocity = current_simulator::movement::Velocity(Vec2::new(a.cos(), a.sin()));
    let guiders = test_map.bake_simulation().unwrap();
    let mut within_bounds_counter = 0;
    for guider in &guiders {
        if guider.position.0.x.abs() <= width/2. && guider.position.0.y.abs() <= height/2. {
            within_bounds_counter += 1;
            eprintln!("{:?} | {:?}", guider.position.0, guider.velocity.0); 
        }
    }
    println!("{}/{} were within bounds", within_bounds_counter, guiders.len())
}
