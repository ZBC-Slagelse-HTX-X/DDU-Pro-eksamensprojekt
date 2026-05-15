use std::time::Duration;
use bevy::prelude::*;
use rand::prelude::*;
#[derive(Resource)]
pub struct TrashSpawnTimer(pub Timer);

impl Default for TrashSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct RemainingTrash(pub u32);

impl Default for RemainingTrash {
    fn default() -> Self {
        Self(50)
    }
}

#[derive(Resource, PartialEq, Eq)]
pub struct Wave(pub u32);

impl Default for Wave {
    fn default() -> Self {
        Self (1)
    }
}

#[derive(Component)]
pub struct TrashPiece;

#[derive(Component)]
pub struct Can;

#[derive(Component)]
pub struct Bottle;

#[derive(Component)]
pub struct WaterDunk;

#[derive(Component)]
pub struct Mass(pub f32); //kg

#[derive(Component)]
pub struct Puller; //kg

#[derive(Message)]
pub struct SpawnTrashPiece {
    pub spawn_pos: Vec2,
    pub initial_velocity: Vec2, 
}

pub fn trash_handler(app: &mut App) {
    app
        .init_resource::<Wave>()
        .init_resource::<TrashSpawnTimer>()
        .init_resource::<RemainingTrash>()
        .add_message::<SpawnTrashPiece>()
        .add_systems(Startup, setup_pullers)
        .add_systems(Update, (change_trash_acceleration, change_trash_velocity, change_trash_position).chain())
        .add_systems(Update, (wave_spawner, spawn_plastic));
}

fn wave_spawner(
    mut writer: MessageWriter<SpawnTrashPiece>,
    mut wave_counter: ResMut<Wave>,
    mut timer: ResMut<TrashSpawnTimer>,
    mut remaining_trash: ResMut<RemainingTrash>,
    time: Res<Time>
) {
    const FINAL_WAVE: u32 = 15;

    timer.0.tick(time.delta());

    if wave_counter.0 <= FINAL_WAVE && remaining_trash.0 > 0 && timer.0.just_finished() {
        let (spawn_pos, initial_velocity) = match wave_counter.0 {
            _ => {
                // Left wall --- more to come
                let north_western_corner = Vec2::new(-(crate::pixel_grid::RES_WIDTH as f32 /2.), crate::pixel_grid::RES_HEIGHT as f32 /2.);
                let position = |numero: u32| -> Vec2 {
                    north_western_corner + numero as f32 * (crate::pixel_grid::RES_HEIGHT as f32 /(wave_counter.0 as f32 *50.)) * Vec2::NEG_Y
                };
                (position(remaining_trash.0 - 1), Vec2::X * 2.)
            },
        };

        writer.write(SpawnTrashPiece{spawn_pos, initial_velocity});
        remaining_trash.0 -= 1;

        if remaining_trash.0 == 0 {
            wave_counter.0 += 1;
            remaining_trash.0 = wave_counter.0*20;
            timer.0.set_duration(Duration::from_secs_f32(0.5/wave_counter.0 as f32))
        }
    }
}

fn setup_pullers(
    mut commands: Commands,
) {
    const PULLER_MASS: f32 = 5e12;
    commands.spawn((
        Puller,
        Transform::from_xyz(160.,270., 0.),
        Mass(PULLER_MASS)
    ));
    commands.spawn((
        Puller,
        Transform::from_xyz(320.,270., 0.),
        Mass(PULLER_MASS)
    ));
}

fn spawn_plastic(
    mut reader: MessageReader<SpawnTrashPiece>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const CAN_PATH: &str = "sprites/trash/can.png";
    const BOTTLE_PATH: &str = "sprites/trash/bottle.png";
    const WATER_DUNK_PATH: &str = "sprites/trash/water_dunk.png";
    const Z_VALUE: f32 = 3.;

    for message in reader.read() {
        let mut rng = rand::rng();
        
        let pick: i32 = rng.random_range(0..3);

        let common = (
            TrashPiece,
            Transform::from_translation(message.spawn_pos.extend(Z_VALUE)),
            crate::movement::Velocity(message.initial_velocity),
            crate::movement::Acceleration::default(),
            crate::pixel_grid::PIXEL_PERFECT_LAYERS,
            crate::movement::Wrappable,
        );

        match pick {
            0 => { 
                commands.spawn((
                    Can,
                    Sprite::from_image(asset_server.load(CAN_PATH)),
                    common,
                ));
            },
            1 => {
                commands.spawn((
                    Bottle,
                    Sprite::from_image(asset_server.load(BOTTLE_PATH)),
                    common,
                ));
            },
            _ => {
                commands.spawn((
                    WaterDunk,
                    Sprite::from_image(asset_server.load(WATER_DUNK_PATH)),
                    common,
                ));
            }
        };
    }
}

fn gravitational_acceleration(mass_kg: f32, pulled_pos: Vec3, puller_pos: Vec3) -> crate::movement::Acceleration {
    const G: f32 = 3.41e-8; // N * px**2 / kg**2
    let pulled_pos = pulled_pos.truncate();
    let puller_pos = puller_pos.truncate();
    let difference_vector = puller_pos - pulled_pos;
    let direction_vector = difference_vector.normalize_or_zero();
    let distance_squared = difference_vector.length_squared(); // px**2
    let acceleration = crate::movement::Acceleration(G * mass_kg / distance_squared * direction_vector);
    acceleration

}

fn change_trash_acceleration (
    mut trash_pieces: Query<(&mut crate::movement::Acceleration, &Transform), With<TrashPiece>>,
    pullers: Query<(&Transform, &Mass), With<Puller>>
) {
    for (mut acceleration, pulled_pos) in &mut trash_pieces {
        acceleration.0 = Vec2::ZERO;
        for (puller_pos, puller_mass) in &pullers {
            let this_acceleration = gravitational_acceleration(puller_mass.0, pulled_pos.translation, puller_pos.translation);
            acceleration.0 += this_acceleration.0;
        }
    }
}

fn change_trash_velocity (
    mut trash_pieces: Query<(&crate::movement::Acceleration, &mut crate::movement::Velocity), With<TrashPiece>>,
    time: Res<Time>
) {
    const THERMAL_SPEED: f32 = 15.;
    for (acceleration, mut velocity) in &mut trash_pieces {
        let change_in_velocity = acceleration.0 * time.delta_secs();
        velocity.0 += change_in_velocity;
        velocity.0 = velocity.0.clamp_length_max(THERMAL_SPEED);

    }
}

fn change_trash_position (
    mut trash_pieces: Query<(&crate::movement::Velocity, &mut Transform), With<TrashPiece>>,
    time: Res<Time>
) {
    for (velocity, mut transform) in &mut trash_pieces {
        let change_in_position = velocity.0 * time.delta_secs();
        transform.translation += change_in_position.extend(0.);
    }
}

