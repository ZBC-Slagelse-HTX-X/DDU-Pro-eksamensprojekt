use std::time::Duration;
use bevy::prelude::*;
use rand::prelude::*;

use std::collections::HashMap;

#[derive(Resource)]
pub struct TrashSpawnTimer(pub Timer);

impl Default for TrashSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct Health(pub f32);

impl Health {
    pub fn monster() -> Self {
        Self (100.)
    }
    pub fn player() -> Self {
        Self (75.)
    }
    pub fn trawler() -> Self {
        Self (1000.)
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
pub struct Monster;

#[derive(Component)]
pub struct Slimey;

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
        .init_resource::<RemainingTrawlers>()
        .add_message::<SpawnTrashPiece>()
        .add_message::<SpawnTrawler>()
        .add_systems(Startup, setup_pullers)
        .add_systems(Update, (spawn_trawler, trawler_movement, spawn_plastic).chain())
        .add_systems(Update, (change_trash_acceleration, change_trash_velocity, change_trash_position).chain())
        .add_systems(Update, (monster_hunt, change_monster_position).chain())
        .add_systems(Update, (wave_spawner, detect_and_merge_clusters, confirm_for_kill, slime_attack, bury_player, confirm_hit));
}

#[derive(Resource)]
pub struct RemainingTrawlers(pub u32);

impl Default for RemainingTrawlers {
    fn default() -> Self {
        Self (1)
    }
}
fn wave_spawner(
    mut trash_writer: MessageWriter<SpawnTrashPiece>,
    mut trawler_writer: MessageWriter<SpawnTrawler>,
    mut wave_counter: ResMut<Wave>,
    mut timer: ResMut<TrashSpawnTimer>,
    mut remaining_trash: ResMut<RemainingTrash>,
    mut remaining_trawlers: ResMut<RemainingTrawlers>,
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

        trash_writer.write(SpawnTrashPiece{spawn_pos, initial_velocity});
        remaining_trash.0 -= 1;

        if remaining_trash.0 == 0 {
            wave_counter.0 += 1;
            remaining_trash.0 = wave_counter.0*20;
            timer.0.set_duration(Duration::from_secs_f32(0.5/wave_counter.0 as f32));
            if wave_counter.0 == 10 && remaining_trawlers.0 > 0 {
                trawler_writer.write(SpawnTrawler);
                remaining_trawlers.0 -= 1;
            }
        }
    }
}

fn setup_pullers(
    mut commands: Commands,
) {
    const PULLER_MASS: f32 = 1e13; // kg
    commands.spawn((
        Puller,
        Transform::from_xyz(240.,270., 0.),
        Mass(PULLER_MASS)
    ));
}

#[derive(Component)]
pub struct SlimeAttackTimer(pub Timer);

impl Default for SlimeAttackTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.5, TimerMode::Repeating))
    }
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

    const WANTED_PERCENTAGE_OF_BOTTLES: u32 = 50;
    const WANTED_PERCENTAGE_OF_CANS: u32 = 35;
    const WANTED_PERCENTAGE_OF_WATERDUNKS: u32 = 15;

    const BOTTLE_UPPER: u32 = WANTED_PERCENTAGE_OF_CANS + WANTED_PERCENTAGE_OF_BOTTLES;
    assert_eq!(
        WANTED_PERCENTAGE_OF_BOTTLES + WANTED_PERCENTAGE_OF_CANS + WANTED_PERCENTAGE_OF_WATERDUNKS,
        100
    );
    
    for message in reader.read() {
        let mut rng = rand::rng();
        
        let pick: u32 = rng.random_range(0..100);

        let common = (
            TrashPiece,
            Transform::from_translation(message.spawn_pos.extend(Z_VALUE)),
            crate::movement::Velocity(message.initial_velocity),
            crate::movement::Acceleration::default(),
            crate::pixel_grid::PIXEL_PERFECT_LAYERS,
            crate::movement::Wrappable,
        );

        match pick {
            0..WANTED_PERCENTAGE_OF_CANS => { 
                commands.spawn((
                    Can,
                    Sprite::from_image(asset_server.load(CAN_PATH)),
                    common,
                ));
            },
            WANTED_PERCENTAGE_OF_CANS..BOTTLE_UPPER => {
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
    const THERMAL_SPEED: f32 = 10.;
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

fn monster_hunt (
    player_query: Single<&Transform, With<crate::player::PlayerAvatar>>,
    mut monster_query: Query<(&Transform, &mut crate::movement::Velocity), With<Slimey>>
) {
    const SPEED: f32 = 18.;

    for (transform, mut velocity) in monster_query.iter_mut() {
        let direction_vector = (player_query.translation.truncate() - transform.translation.truncate()).normalize();
        velocity.0 = direction_vector * SPEED;
    }
}

fn change_monster_position (
    mut monster_query: Query<(&crate::movement::Velocity, &mut Transform), With<Monster>>,
    time: Res<Time>
) {
    for (velocity, mut transform) in &mut monster_query.iter_mut() {
        let change_in_position = velocity.0 * time.delta_secs();
        transform.translation += change_in_position.extend(0.);
    }
}

fn confirm_for_kill (
    mut commands: Commands,
    slime_query: Query<(Entity, &Health), With<Monster>>
) {
    for (entity, health) in slime_query {
        if health.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

// ##################### LAVET AF AI #####################
// Dog tilrettet

fn detect_and_merge_clusters(
    query: Query<(Entity, &Transform), With<TrashPiece>>,
    slime_query: Query<(), With<Slimey>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const MONSTER_PATH: &str = "sprites/green_monster/right_front.png";
    const CLUSTER_RADIUS: f32 = 25.0;
    const CLUSTER_MIN: usize = 8;
    const MAX_AMOUNT_OF_SLIMES: usize = 3;

    let amount_of_monsters = slime_query.iter().count();

    if amount_of_monsters >= MAX_AMOUNT_OF_SLIMES {
        return;
    }

    let entities: Vec<(Entity, Vec3)> = query
        .iter()
        .map(|(e, t)| (e, t.translation))
        .collect();

    let cell_size = CLUSTER_RADIUS;
    let mut grid: HashMap<(i32, i32), Vec<usize>> = HashMap::new();
    for (i, (_, pos)) in entities.iter().enumerate() {
        let cell = ((pos.x / cell_size) as i32, (pos.y / cell_size) as i32);
        grid.entry(cell).or_default().push(i);
    }

    let mut visited = vec![false; entities.len()];
    let mut clusters: Vec<Vec<usize>> = Vec::new();
    for (i, (_, pos)) in entities.iter().enumerate() {
        if visited[i] {
            continue;
        }
        let cell = ((pos.x / cell_size) as i32, (pos.y / cell_size) as i32);
        let mut nearby: Vec<usize> = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(cell_entities) = grid.get(&(cell.0 + dx, cell.1 + dy)) {
                    for &j in cell_entities {
                        if i != j && pos.distance(entities[j].1) <= CLUSTER_RADIUS {
                            nearby.push(j);
                        }
                    }
                }
            }
        }
        if nearby.len() >= CLUSTER_MIN - 1 {
            let mut cluster = vec![i];
            cluster.extend(nearby.iter().filter(|&&j| !visited[j]));
            for &idx in &cluster {
                visited[idx] = true;
            }
            clusters.push(cluster);
        }
    }

    let mut spawned_this_frame = 0;

    for cluster in clusters {
        if amount_of_monsters + spawned_this_frame >= MAX_AMOUNT_OF_SLIMES {
            break;
        }

        let centroid = cluster.iter()
            .map(|&i| entities[i].1)
            .fold(Vec3::ZERO, |acc, p| acc + p)
            / cluster.len() as f32;

        for &i in &cluster {
            commands.entity(entities[i].0).despawn();
        }

        commands.spawn((
            Monster,
            Health::monster(),
            SlimeAttackTimer::default(),
            Slimey,
            Sprite::from_image(asset_server.load(MONSTER_PATH)),
            crate::pixel_grid::PIXEL_PERFECT_LAYERS,
            crate::movement::NonWrappable,
            Transform::from_translation(centroid),
            crate::movement::Velocity::default()
        ));

        spawned_this_frame += 1;
    }
}

fn slime_attack(
    mut slime_query: Query<(&Transform, &mut SlimeAttackTimer), With<Slimey>>,
    mut player_query: Single<(&Transform, &mut Health), With<crate::player::PlayerAvatar>>,
    time: Res<Time>,
) {
    const SLIME_WOUND_DAMAGE: f32 = 25.0;
    const ATTACK_RANGE: f32 = 45.0;

    let (player_transform, ref mut player_health) = *player_query;

    for (transform, mut timer) in slime_query.iter_mut() {
        timer.0.tick(time.delta());

        if !timer.0.just_finished() {
            continue;
        }

        let distance = transform.translation
            .truncate()
            .distance(player_transform.translation.truncate());

        if distance <= ATTACK_RANGE {
            player_health.0 -= SLIME_WOUND_DAMAGE;
            eprintln!("Player hit by slime! Health: {}", player_health.0);
        }
    }
}

#[derive(Component)]
pub struct Trawler;

#[derive(Message)]
pub struct SpawnTrawler;

fn spawn_trawler(
    mut events: MessageReader<SpawnTrawler>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const TRAWLER_PATH: &str = "sprites/trawler/trawler.png";

    if events.read().next().is_none() {
        return;
    }

    let x = 0.0;
    let y = crate::pixel_grid_copy::RES_HEIGHT as f32 / 2.0 - 8.0;

    commands.spawn((
        Monster,
        Trawler,
        Health::trawler(),
        TrawlerMovement::default(),
        Sprite::from_image(asset_server.load(TRAWLER_PATH)),
        crate::pixel_grid::PIXEL_PERFECT_LAYERS,
        crate::movement::NonWrappable,
        Transform::from_translation(Vec3::new(x, y, 0.5)),
        crate::movement::Velocity::default(),
    ));
}

#[derive(Component)]
pub struct TrawlerMovement {
    pub timer: Timer,
    pub time_elapsed: f32,
}


impl Default for TrawlerMovement {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            time_elapsed: 0.0,
        }
    }
}

fn trawler_movement(
    mut query: Query<(&mut Transform, &mut TrawlerMovement), With<Trawler>>,
    time: Res<Time>,
    mut writer: MessageWriter<SpawnTrashPiece>,
) {
    let half_width  = crate::pixel_grid_copy::RES_WIDTH as f32 / 2.0;
    let half_height = crate::pixel_grid_copy::RES_HEIGHT as f32 / 2.0;
    const DESCENT_SPEED: f32 = 20.0;
    const FREQUENCY: f32 = 0.5; // full sweeps per second — lower = slower

    for (mut transform, mut movement) in query.iter_mut() {
        movement.time_elapsed += time.delta_secs();
        movement.timer.tick(time.delta());

        // x oscillates between -half_width and +half_width
        transform.translation.x = (movement.time_elapsed * FREQUENCY * std::f32::consts::TAU).sin() * (half_width - 8.0);

        // y drifts downward, wraps to top
        transform.translation.y -= DESCENT_SPEED * time.delta_secs();
        if transform.translation.y <= -half_height + 8.0 {
            transform.translation.y = half_height - 8.0;
        }

        if movement.timer.just_finished() {
            writer.write(SpawnTrashPiece {
                spawn_pos: transform.translation.truncate(),
                initial_velocity: Vec2::NEG_Y * 5.0,
            });
        }
    }
}

// #######################################################

fn bury_player(
    amount_of_points: Res<crate::items::Points>,
    player_health: Single<&Health, With<crate::player::PlayerAvatar>>,
) {

    if player_health.0 <= 0.0 {
        panic!("GAMEOVER, YOU DIED, FINAL SCORE: {}. SIMULATION OVER", amount_of_points.0);
    }
}

fn confirm_hit(
    mut monster_query: Query<(&Transform, &mut Health, &Sprite), With<Monster>>,
    bullet_query: Query<(Entity, &Transform), With<crate::aim::Bullet>>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
) {
    const SHOT_DAMAGE: f32 = 25.;

    for (monster_transform, mut health, sprite) in monster_query.iter_mut() {
        let half_extent = images
            .get(&sprite.image)
            .map(|img| img.size().as_vec2() / 2.0)
            .unwrap_or(Vec2::splat(8.0));

        let center = monster_transform.translation.truncate();
        let min = center - half_extent;
        let max = center + half_extent;

        for (bullet_entity, bullet_transform) in bullet_query.iter() {
            let bullet_pos = bullet_transform.translation.truncate();

            if bullet_pos.x >= min.x && bullet_pos.x <= max.x
            && bullet_pos.y >= min.y && bullet_pos.y <= max.y {
                health.0 -= SHOT_DAMAGE;
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}
