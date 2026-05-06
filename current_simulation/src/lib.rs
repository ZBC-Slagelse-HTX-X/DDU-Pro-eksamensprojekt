// https://rust-lang.github.io/api-guidelines/
use bevy::prelude::*;
use std::f32;

pub mod movement;
pub mod gravity;

pub struct Map {
    width: f32,
    height: f32,
    pub wrap_around: bool,
    pub initial_velocity: movement::Velocity,
    pub simulation_density: u16,
    pub step_size: f32,
    pub iterations: u32,
    pub attractors: Option<Vec<gravity::Attractor>>
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: 0.,
            height: 0.,
            wrap_around: false,
            initial_velocity: movement::Velocity(Vec2::new(0., 0.)),
            simulation_density: 10,
            step_size: 1.,
            iterations: 10,
            attractors: None
        }
    }
}

impl Map {
    pub fn new(width: f32, height: f32) -> Self {
        Self {width, height, ..default()}
    }
    #[must_use]
    pub fn bake_simulation(&self) -> Result<Vec<crate::movement::VelocityPosition>, &str> {
        assert!(self.simulation_density > 1);
        let mut angle: f32 = self.initial_velocity.0.to_angle(); 

        let southern_east_corner = Vec2::new(self.width/2., -self.height/2.);
        let northern_east_corner = Vec2::new(self.width/2., self.height/2.);
        let northern_west_corner = Vec2::new(-self.width/2., self.height/2.);
        let southern_west_corner = Vec2::new(-self.width/2., -self.height/2.);

        let mut spawning_positions: Vec<crate::movement::Position> = Vec::new();

        angle = angle.rem_euclid(2.0 * f32::consts::PI);
        
        if angle >= 0. && angle <= f32::consts::FRAC_PI_8 {
            eprintln!("Approx east-pointing velocity, western-border");
            let position = |distance: f32| -> crate::movement::Position { 
                let pos_vec = northern_west_corner + distance * Vec2::NEG_Y;
                crate::movement::Position(pos_vec)
            };

            let increment = self.height/(self.simulation_density -1) as f32;
            for i in 0..self.simulation_density {
                spawning_positions.push(position(increment * i as f32));
            }
        }

        else if angle > f32::consts::FRAC_PI_8 && angle <= 3.*f32::consts::FRAC_PI_8 {
            const BORDER_COVERAGE: f32 = 0.66;
            eprintln!("Approx east-northern pointing velocity, respectively partly sourthern and western border");
            let eastern_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = northern_east_corner + distance * Vec2::NEG_Y;
                crate::movement::Position(pos_vec)
            };
            let northern_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = northern_east_corner + distance * Vec2::NEG_X;
                crate::movement::Position(pos_vec)
            };
            let new_positions = if self.simulation_density % 2 == 1 {
                spawning_positions.push(crate::movement::Position(northern_east_corner));
                self.simulation_density -1
            } else {
                self.simulation_density
            };
            let half_of_positions = new_positions / 2;
            let western_increment = self.width * BORDER_COVERAGE / half_of_positions as f32;
            let southern_increment = self.height * BORDER_COVERAGE /half_of_positions as f32;
            for i in 1..half_of_positions+1 {
                spawning_positions.push(eastern_position(i as f32 * southern_increment));
            }
            for i in 1..half_of_positions+1 {
                spawning_positions.push(northern_position(i as f32 * western_increment));
            }
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
        }

        else if angle > 5.*f32::consts::FRAC_PI_8 && angle <= 7.*f32::consts::FRAC_PI_8 {
            const BORDER_COVERAGE: f32 = 0.66;
            eprintln!("Approx west-northern pointing velocity, respectively partly eastern and southern border");
            let northern_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = northern_west_corner + distance * Vec2::NEG_Y;
                crate::movement::Position(pos_vec)
            };
            let western_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = northern_west_corner + distance * Vec2::X;
                crate::movement::Position(pos_vec)
            };
            let new_positions = if self.simulation_density % 2 == 1 {
                spawning_positions.push(crate::movement::Position(northern_west_corner));
                self.simulation_density - 1
            } else {
                self.simulation_density
            };
            let half_of_positions = new_positions / 2;
            let eastern_increment = self.width * BORDER_COVERAGE / half_of_positions as f32;
            let southern_increment = self.height * BORDER_COVERAGE / half_of_positions as f32;
            for i in 1..half_of_positions+1 {
                spawning_positions.push(northern_position(i as f32 * southern_increment));
            }
            for i in 1..half_of_positions+1 {
                spawning_positions.push(western_position(i as f32 * eastern_increment));
            }
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
        }

        else if angle > 9.*f32::consts::FRAC_PI_8 && angle <= 11.*f32::consts::FRAC_PI_8 {
            const BORDER_COVERAGE: f32 = 0.66;
            eprintln!("Approx west-southern pointing velocity, respectively partly eastern and northern border");
            let western_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = southern_west_corner + distance * Vec2::Y;
                crate::movement::Position(pos_vec)
            };
            let southern_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = southern_west_corner + distance * Vec2::X;
                crate::movement::Position(pos_vec)
            };
            let new_positions = if self.simulation_density % 2 == 1 {
                spawning_positions.push(crate::movement::Position(southern_west_corner));
                self.simulation_density - 1
            } else {
                self.simulation_density
            };
            let half_of_positions = new_positions / 2;
            let eastern_increment = self.width * BORDER_COVERAGE / half_of_positions as f32;
            let northern_increment = self.height * BORDER_COVERAGE / half_of_positions as f32;
            for i in 1..half_of_positions+1 {
                spawning_positions.push(western_position(i as f32 * northern_increment));
            }
            for i in 1..half_of_positions+1 {
                spawning_positions.push(southern_position(i as f32 * eastern_increment));
            }
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
        }

        else if angle > 13.*f32::consts::FRAC_PI_8 && angle <= 15.*f32::consts::FRAC_PI_8 {
            const BORDER_COVERAGE: f32 = 0.66;
            eprintln!("Approx east-southern pointing velocity, respectively partly western and northern border");
            let southern_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = southern_east_corner + distance * Vec2::NEG_X;
                crate::movement::Position(pos_vec)
            };
            let eastern_position = |distance: f32| -> crate::movement::Position {
                let pos_vec = southern_east_corner + distance * Vec2::Y;
                crate::movement::Position(pos_vec)
            };
            let new_positions = if self.simulation_density % 2 == 1 {
                spawning_positions.push(crate::movement::Position(southern_east_corner));
                self.simulation_density - 1
            } else {
                self.simulation_density
            };
            let half_of_positions = new_positions / 2;
            let western_increment = self.width * BORDER_COVERAGE / half_of_positions as f32;
            let northern_increment = self.height * BORDER_COVERAGE / half_of_positions as f32;
            for i in 1..half_of_positions+1 {
                spawning_positions.push(southern_position(i as f32 * western_increment));
            }
            for i in 1..half_of_positions+1 {
                spawning_positions.push(eastern_position(i as f32 * northern_increment));
            }
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
        }

        if spawning_positions.len() == self.simulation_density as usize {
            let mut current_velocitypositions: Vec<crate::movement::VelocityPosition> = Vec::new();
            let mut vector_positions_with_velocity: Vec<crate::movement::VelocityPosition> = Vec::new();

            for position in &spawning_positions {
                let vp = crate::movement::VelocityPosition::new(self.initial_velocity, *position);
                current_velocitypositions.push(vp);
            }

            for _ in 0..self.iterations {
                let mut next_positions: Vec<crate::movement::VelocityPosition> = Vec::new();

                for velocityposition in &current_velocitypositions {
                    let mut total_accel = Vec2::ZERO;
    
                    if let Some(attractors) = &self.attractors {
                        for attractor in attractors {
                            let diff = attractor.position.0 - velocityposition.position.0;
                            let accel = gravity::gravitational_acceleration(attractor.mass, diff);
                            total_accel += accel.0;
                        }
                    }

                    let stepped = velocityposition.step(movement::Acceleration(total_accel), self.step_size);
                    next_positions.push(stepped);
                }                        

                for vp in &next_positions {
                    vector_positions_with_velocity.push(vp.clone());
                }

                current_velocitypositions = next_positions;
            }
            return Ok(vector_positions_with_velocity);
        }

        else {return Err("Spawning stretch could not be found given the initial velocity's direction.")};
    }
}

#[cfg(feature = "previewing")]
pub mod previewing;
