// https://rust-lang.github.io/api-guidelines/
use bevy::prelude::*;
use std::f32;

pub mod movement;
mod gravity;

pub struct Map {
    width: f32,
    height: f32,
    pub wrap_around: bool,
    pub initial_velocity: movement::Velocity,
    pub simulation_density: u16,
    pub step: f32
}

struct Particle {
    position: movement::Position,
    velocity: movement::Velocity 
}

struct GhostParticle {
    position: movement::Position,
    acceleration: movement::Acceleration
}

impl Default for Map {
    fn default() -> Self {
        Self {width: 0., height: 0., wrap_around: false, initial_velocity: movement::Velocity(Vec2::new(0., 0.)), simulation_density: 10, step: 1.}
    }
}

impl Map {
    pub fn new(width: f32, height: f32) -> Self {
        Self {width, height, ..default()}
    }
    #[must_use]
    pub fn bake_simulation(&self) -> Result<Vec<crate::movement::Position>, &str> {
        assert!(self.simulation_density > 0);
        let mut angle: f32 = self.initial_velocity.0.to_angle() + f32::consts::PI; // Adding pi as default is returned in rads from -pi to +pi

        let southern_east_corner = Vec2::new(self.width/2., -self.height/2.);
        let northern_east_corner = Vec2::new(self.width/2., self.height/2.);
        let northern_west_corner = Vec2::new(-self.width/2., self.height/2.);
        let southern_west_corner = Vec2::new(-self.width/2., -self.height/2.);

        let mut spawning_positions: Vec<crate::movement::Position> = Vec::new();

        angle = angle % (2.*f32::consts::PI);
        
        if angle >= 0. && angle <= f32::consts::FRAC_PI_8 {
            eprintln!("Approx east-pointing velocity, western-border");
            let position = |distance: f32| -> crate::movement::Position { 
                let pos_vec = northern_west_corner + distance * Vec2::NEG_Y;
                crate::movement::Position(pos_vec)
            };
            let increment = self.height/(self.simulation_density -1) as f32;
            spawning_positions.push(position(0.));
            let mut distance = increment;
            while distance <= self.height {
                spawning_positions.push(position(distance));
                distance += increment;
            }
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions);
        }

        else if angle > f32::consts::FRAC_PI_8 && angle <= 3.*f32::consts::FRAC_PI_8 {
            eprintln!("Approx east-northern pointing velocity, respectively partly sourthern and western border");
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions)
        }

        else if angle > 3.*f32::consts::FRAC_PI_8 && angle <= 5.*f32::consts::FRAC_PI_8 {
            eprintln!("Approx north-pointing velocity, southern border");
            let position = |distance: f32| -> crate::movement::Position {
                let pos_vec = southern_west_corner + distance * Vec2::X;
                crate::movement::Position(pos_vec)
            };
            let increment = self.width/(self.simulation_density -1) as f32;
            spawning_positions.push(position(0.));
            let mut distance = increment;
            while distance <= self.width {
                spawning_positions.push(position(distance));
                distance += increment;
            }
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions);
        }

        else if angle > 5.*f32::consts::FRAC_PI_8 && angle <= 7.*f32::consts::FRAC_PI_8 {
            eprintln!("Approx west-northern pointing velocity, respectively partly eastern and southern border");
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions)
        }

        else if angle > 7.*f32::consts::FRAC_PI_8 && angle <= 9.*f32::consts::FRAC_PI_8 {
            eprintln!("Approx west-pointing velocity, eastern border");
            let position = |distance: f32| -> crate::movement::Position {
                let pos_vec = southern_east_corner + distance * Vec2::Y;
                crate::movement::Position(pos_vec)
            };
            let increment = self.height/(self.simulation_density -1) as f32;
            spawning_positions.push(position(0.));
            let mut distance = increment;
            while distance <= self.height {
                spawning_positions.push(position(distance));
                distance += increment;
            }
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions);
        }

        else if angle > 9.*f32::consts::FRAC_PI_8 && angle <= 11.*f32::consts::FRAC_PI_8 {
            eprintln!("Approx west-southern pointing velocity, respectively partly eastern and northern border");
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions)
        }

        else if angle > 11.*f32::consts::FRAC_PI_8 && angle <= 13.*f32::consts::FRAC_PI_8 {
            eprintln!("Approx southern-pointing velocity, northern border");
            let position = |distance: f32| -> crate::movement::Position {
                let pos_vec = northern_east_corner + distance * Vec2::NEG_X;
                crate::movement::Position(pos_vec)
            };
            let increment = self.width/(self.simulation_density -1) as f32;
            spawning_positions.push(position(0.));
            let mut distance = increment;
            while distance <= self.height {
                spawning_positions.push(position(distance));
                distance += increment;
            }
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions);
        }

        else if angle > 13.*f32::consts::FRAC_PI_8 && angle <= 15.*f32::consts::FRAC_PI_8 {
            eprintln!("Approx east-southern pointing velocity, respectively partly western and northern border");
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions)
        }

        else if angle > 15.*f32::consts::FRAC_PI_8 && angle <= 2.*f32::consts::PI {
            eprintln!("Approx east-pointing velocity, western-border");
            let position = |distance: f32| -> crate::movement::Position {
                let pos_vec = northern_west_corner + distance * Vec2::NEG_Y;
                crate::movement::Position(pos_vec)
            };
            let increment = self.height/(self.simulation_density -1) as f32;
            spawning_positions.push(position(0.));
            let mut distance = increment;
            while distance <= self.height {
                spawning_positions.push(position(distance));
                distance += increment;
            }
            assert_eq!(spawning_positions.len(), self.simulation_density as usize);
            return Ok(spawning_positions);
        }
        else {return Err("Spawning stretch could not be found given the initial velocity's direction.")};
    }
}

