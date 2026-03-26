use bevy::prelude::*;

mod current;
mod enemies;
mod player;
mod movement;
mod aim;
mod splash_screen;

fn setup_camera(mut commands: Commands) {
    commands.spawn(
        Camera2d
    );
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<movement::InputMode>()
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_plugins(splash_screen::splash_plugin)
        .add_systems(OnEnter(GameState::Game), player::spawn_player)
        .add_plugins(player::player_movement_plugin)
        .add_plugins(aim::aim_plugin)
        .run();
}

