use bevy::prelude::*;

pub fn sounds(app: &mut App) {
    app
        .add_systems(Startup, setup_audio)
        .add_systems(Update, advance_track);
}

#[derive(Resource)]
struct Soundtrack {
    tracks: Vec<Handle<AudioSource>>,
    current: usize,
}

#[derive(Component)]
pub struct TrawlerSfx;

#[derive(Component)]
struct OceanAmbience;

#[derive(Component)]
struct SteamboatAmbience;

#[derive(Component)]
struct BackgroundMusic;

fn setup_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(Soundtrack {
        tracks: vec![
            asset_server.load("music/track1.mp3"),
            asset_server.load("music/track2.mp3"),
        ],
        current: 0,
    });

    commands.spawn((
        AudioPlayer::new(asset_server.load("music/ocean.wav")),
        PlaybackSettings::LOOP,
        OceanAmbience,
    ));

    commands.spawn((
        AudioPlayer::new(asset_server.load("music/track1.mp3")),
        PlaybackSettings::DESPAWN, // changed
        BackgroundMusic,
    ));
}

fn advance_track(
    mut commands: Commands,
    mut soundtrack: ResMut<Soundtrack>,
    current: Query<Entity, With<BackgroundMusic>>,
) {
    // If no BackgroundMusic entity exists, the track finished and was despawned
    if current.is_empty() {
        soundtrack.current = (soundtrack.current + 1) % soundtrack.tracks.len();

        commands.spawn((
            AudioPlayer::new(soundtrack.tracks[soundtrack.current].clone()),
            PlaybackSettings::DESPAWN,
            BackgroundMusic,
        ));
    }
}

fn play_steamboat(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/steamboat.wav")),
        PlaybackSettings::LOOP,
        SteamboatAmbience,
    ));
}

fn stop_steamboat(
    mut commands: Commands,
    steamboat: Query<Entity, With<SteamboatAmbience>>,
) {
    for entity in steamboat.iter() {
        commands.entity(entity).despawn();
    }
}
