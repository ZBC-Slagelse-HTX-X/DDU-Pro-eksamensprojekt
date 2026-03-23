use bevy::prelude::*;

mod current;
mod enemies;
mod player;
mod movement;
mod aim;

pub fn setup_camera(mut commands: Commands) {
commands.spawn(
    Camera2d
);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<movement::InputMode>()
        .add_systems(Startup, (setup_camera, player::spawn_player))
        .add_systems(Update, (player::change_player_acceleration, player::change_player_velocity, player::update_player_position))
        .add_systems(Update, (aim::change_aim_acceleration, aim::change_aim_velocity, aim::update_aim_position))
        .run();
}

