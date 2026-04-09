use bevy::prelude::*;
use super::GameState;

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/bevy_logo_dark.png");
    commands.spawn((
        // This entity will be despawned when exiting the state
        DespawnOnExit(GameState::Splash),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        OnSplashScreen,
        children![(
            ImageNode::new(icon),
            Node {
                // This will set the logo to be 200px wide, and auto adjust its height
                width: px(400),
                ..default()
            },
        )],
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state.set(GameState::Game);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

pub fn splash_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
}
