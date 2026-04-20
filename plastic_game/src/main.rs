use bevy::prelude::*;

mod current;
mod enemies;
mod player;
mod movement;
mod aim;
mod splash_screen;
mod pixel_grid;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_resource::<movement::InputMode>()
        .init_state::<GameState>()
        .add_systems(Startup, pixel_grid::setup_camera)
        .add_plugins(splash_screen::splash_plugin)
        .add_systems(OnEnter(GameState::Game), player::spawn_player)
        .add_plugins(player::player_movement_plugin)
        .add_plugins(aim::aim_plugin)
        .add_systems(Update, (pixel_grid::fit_canvas, movement::change_inputmode))
        .run();
}

