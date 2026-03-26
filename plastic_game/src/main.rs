use bevy::prelude::*;

mod current;
mod enemies;
mod player;
mod movement;
mod aim;
mod splash_screen;

mod setup {
    use bevy::prelude::*;
    use crate::player;

    pub fn setup_camera(mut commands: Commands) {
        commands.spawn(
            Camera2d
        );
    }
    pub fn setup_plugin(app: &mut App) {
        app
            .add_systems(Startup, (setup_camera, player::spawn_player));
    }
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
        .add_plugins(splash_screen::splash_plugin)
        .add_plugins(setup::setup_plugin)
        .add_plugins(player::player_movement_plugin)
        .add_plugins(aim::aim_plugin)
        .run();
}

