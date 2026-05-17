use bevy::prelude::*;

mod current;
mod enemies;
mod player;
mod movement;
mod aim;
mod music;
mod splash_screen;
mod pixel_grid;
mod pixel_grid_copy;
mod items;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash1,
    Splash2,
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(items::items)
        .init_resource::<movement::InputMode>()
        .init_state::<GameState>()
        .add_systems(Startup, pixel_grid_copy::setup_camera)
        .add_plugins(splash_screen::splash_plugin)
        .add_plugins(music::sounds)
        .add_systems(OnEnter(GameState::Game), player::spawn_player)
        .add_plugins(player::player_movement_plugin)
        .add_plugins(movement::wrap_non_wrap)
        .add_plugins(aim::aim_plugin)
        .add_plugins(enemies::trash_handler)
        .add_systems(Update, (pixel_grid_copy::fit_canvas, movement::change_inputmode))
        .run();
}

