use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerAim;
pub fn aim_plugin (app: &mut App) {
    app
        .add_systems(Update, (
            change_aim_acceleration,
            change_aim_velocity,
            update_aim_position,
            minimum_aim_distance,
            maximum_aim_distance,
            shooting.run_if(in_state(crate::items::CharacterMode::Shooting)),
            travel_bullet,
            despawn_bullets
        ));
}

pub fn change_aim_acceleration (
    mut current_acceleration: Single<&mut crate::movement::Acceleration, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mode: Res<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>,
    hand_mode: Res<crate::movement::HandMode>
) {
    const SPEED_OF_ACCELERATION: f32 = 3000.;
    let mut weighed_direction: Vec2 = Vec2::ZERO;
    match *mode {
        crate::movement::InputMode::Keyboard => {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                weighed_direction.y += 0.3;
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                weighed_direction.x -= 0.3;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                weighed_direction.y -= 0.3;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                weighed_direction.x += 0.3;
            }
        },
        crate::movement::InputMode::Controller => {
            if let Some(gamepad) = gamepad {
                const THRESHOLD: f32 = 0.025;
                const SENSITIVITY: f32 = 0.5;
                
                let (stick_x, stick_y) = match *hand_mode {
                    crate::movement::HandMode::RightHand => (GamepadAxis::RightStickX, GamepadAxis::RightStickY),
                    crate::movement::HandMode::LeftHand  => (GamepadAxis::LeftStickX,  GamepadAxis::LeftStickY),
                };
                
                let current_stick_x = gamepad.get(stick_x).unwrap();
                let current_stick_y = gamepad.get(stick_y).unwrap();
                
                if current_stick_x.abs() > THRESHOLD {
                    weighed_direction.x += current_stick_x * SENSITIVITY;
                }
                if current_stick_y.abs() > THRESHOLD {
                    weighed_direction.y += current_stick_y * SENSITIVITY;
                }
            }
        }  // <-- closes Controller arm
    }      // <-- closes match *mode
    current_acceleration.0 = weighed_direction.clamp_length_max(1.0) * SPEED_OF_ACCELERATION;
}

pub fn change_aim_velocity (
    mut current_velocity: Single<&mut crate::movement::Velocity, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    current_acceleration: Single<&crate::movement::Acceleration, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    time: Res<Time>
)  {
    const THERMAL_SPEED: f32 = 125.;
    const FRICTION: f32 = 300.;
    let change_in_velocity = current_acceleration.0 * time.delta_secs();
    current_velocity.0 += change_in_velocity;
    current_velocity.0 = current_velocity.0.clamp_length_max(THERMAL_SPEED);
    if current_velocity.0.length() > 0. {
        if FRICTION * time.delta_secs() > current_velocity.0.length() {
            current_velocity.0 = Vec2::ZERO;
        }
        else {
            let direction_of_movement = current_velocity.0.clone().normalize_or_zero();
            current_velocity.0 += FRICTION * -direction_of_movement * time.delta_secs(); 
        }
    }
}

pub fn minimum_aim_distance (
    mut current_shooting_pos: Single<&mut Transform, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    mut current_velocity: Single<&mut crate::movement::Velocity, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
) {
    const MINIMUM_DISTANCE: f32 = 20.;
    let flat_pos = current_shooting_pos.translation.truncate();
    let normalized = flat_pos.normalize_or_zero();
    if normalized != Vec2::ZERO && flat_pos.length() < MINIMUM_DISTANCE {
        let z_value: f32 = current_shooting_pos.translation.z;
        current_shooting_pos.translation = (normalized * -MINIMUM_DISTANCE * 1.25).extend(z_value);
        current_velocity.0 = Vec2::ZERO;
    }
}

pub fn maximum_aim_distance (
    mut current_shooting_pos: Single<&mut Transform, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    mut current_velocity: Single<&mut crate::movement::Velocity, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
) {
    const MAXIMUM_DISTANCE: f32 = 270.;
    let flat_pos = current_shooting_pos.translation.truncate();
    if flat_pos.length() > MAXIMUM_DISTANCE {
        let z_value: f32 = current_shooting_pos.translation.z;
        let flat_pos = flat_pos.clamp_length_max(MAXIMUM_DISTANCE);
        current_shooting_pos.translation = flat_pos.extend(z_value);
        current_velocity.0 = Vec2::ZERO;
    }
}


pub fn update_aim_position (
    query: Single<(&mut Transform, &crate::movement::Velocity), (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    time: Res<Time>
) {
    let (mut aim_position, velocity) = query.into_inner();
    let change_in_position = velocity.0 * time.delta_secs();
    aim_position.translation += change_in_position.extend(0.0);
}

#[derive(Component)]
pub struct Bullet;

fn shooting(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mode: Res<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>,
    current_shooting_pos: Single<&GlobalTransform, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    current_player_pos: Single<&Transform, (With<crate::player::PlayerAvatar>, Without<PlayerAim>)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let fired = match *mode {
        crate::movement::InputMode::Keyboard => keyboard_input.just_pressed(KeyCode::Space),
        crate::movement::InputMode::Controller => {
            if let Some(gamepad) = gamepad {
                gamepad.just_pressed(GamepadButton::RightTrigger2)
            } else {
                false
            }
        }
    };

    if !fired {
        return;
    }

    const PROJECTILE_PATH: &str = "sprites/bullet/projectile.png";
    const BULLET_SPEED: f32 = 200.; // Px s^-1
    const ARM_LENGTH: f32 = 20.; // Px
    const Z_VALUE: f32 = 0.25;

    let player_pos = current_player_pos.translation.truncate();
    let target = current_shooting_pos.translation().truncate();
    let direction = (target - player_pos).normalize();
    let bullet_origin = player_pos + direction*ARM_LENGTH;
    let angle = direction.to_angle(); // angle in radians from Vec2
    let rotation = Quat::from_rotation_z(angle + std::f32::consts::FRAC_PI_2);

    eprintln!("Shot from: {} at: {}", player_pos, target);
    commands.spawn((
        Bullet,
        Sprite::from_image(asset_server.load(PROJECTILE_PATH)),
        Transform::from_translation(bullet_origin.extend(Z_VALUE)).with_rotation(rotation),
        crate::movement::Velocity::from_vec(direction*BULLET_SPEED),
        crate::pixel_grid::PIXEL_PERFECT_LAYERS,
    ));
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/shot.mp3")),
        PlaybackSettings::DESPAWN,
    ));
}

fn travel_bullet(
    bullet_query: Query<(&mut Transform, &crate::movement::Velocity), With<Bullet>>,
    time: Res<Time>
) {
    for (mut transform, velocity) in bullet_query {
        transform.translation += (velocity.0 * time.delta_secs()).extend(0.);
    }
}

fn despawn_bullets(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Bullet>>,
) {
    const TOLERANCE: f32 = 4.0;
    let half_width = crate::pixel_grid::RES_WIDTH as f32 / 2.0 + TOLERANCE;
    let half_height = crate::pixel_grid::RES_HEIGHT as f32 / 2.0 + TOLERANCE;

    for (entity, transform) in query.iter() {
        let pos = transform.translation.truncate();
        if pos.x.abs() > half_width || pos.y.abs() > half_height {
            commands.entity(entity).despawn();
        }
    }
}
