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
            shooting
        ));
}

pub fn change_aim_acceleration (
    mut current_acceleration: Single<&mut crate::movement::Acceleration, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mode: Res<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>
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
            // Uses right stick for aim
            if let Some(gamepad) = gamepad {
                const THRESHOLD: f32 = 0.025;
                const SENSITIVITY: f32 = 0.5;
                let current_stick_x = gamepad.get(GamepadAxis::RightStickX).unwrap();
                let current_stick_y = gamepad.get(GamepadAxis::RightStickY).unwrap();
                if current_stick_x.abs() > THRESHOLD {
                    weighed_direction.x += current_stick_x * SENSITIVITY;
                }
                if current_stick_y.abs() > THRESHOLD {
                    weighed_direction.y += current_stick_y * SENSITIVITY;
                }
            } 
        }
    }
    current_acceleration.0 = weighed_direction.clamp_length_max(1.0) * SPEED_OF_ACCELERATION;
}

pub fn change_aim_velocity (
    mut current_velocity: Single<&mut crate::movement::Velocity, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    current_acceleration: Single<&crate::movement::Acceleration, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    time: Res<Time>
)  {
    const THERMAL_SPEED: f32 = 150.;
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

pub fn update_aim_position (
    query: Single<(&mut Transform, &crate::movement::Velocity), (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    time: Res<Time>
) {
    let (mut aim_position, velocity) = query.into_inner();
    let change_in_position = velocity.0 * time.delta_secs();
    aim_position.translation += change_in_position.extend(0.0);
}

fn shooting (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mode: Res<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>,
    current_shooting_pos: Single<&Transform, (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    current_player_pos: Single<&Transform, (With<crate::player::PlayerAvatar>, Without<PlayerAim>)>
) {
    match *mode {
        crate::movement::InputMode::Keyboard => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                eprintln!("Shot at: {}", current_shooting_pos.translation);
                eprintln!("Shot from: {}", current_player_pos.translation);
            }
        },
        crate::movement::InputMode::Controller => {
            if let Some(gamepad) = gamepad {
                const THRESHOLD: f32 = 0.5;
                let trigger_button = gamepad.get(GamepadButton::RightTrigger2).unwrap();
                if trigger_button > THRESHOLD {
                    eprintln!("Shot at: {}", current_shooting_pos.translation);
                    eprintln!("Shot from: {}", current_player_pos.translation);
                }
            };
        }

    }
}
