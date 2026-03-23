use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerAim;

pub fn change_aim_acceleration(
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
                weighed_direction.y += 0.1;
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                weighed_direction.x -= 0.1;
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                weighed_direction.y -= 0.1;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                weighed_direction.x += 0.1;
            }
        },
        crate::movement::InputMode::Controller => {
            // Uses left stick for movement
            if let Some(gamepad) = gamepad {
                let current_stick_x = gamepad.get(GamepadAxis::RightStickX).unwrap();
                let current_stick_y = gamepad.get(GamepadAxis::RightStickY).unwrap();
                weighed_direction.x += current_stick_x;
                weighed_direction.y += current_stick_y;
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
    const THERMAL_SPEED: f32 = 75.;
    const FRICTION: f32 = 150.;
    let mut change_in_velocity: Vec2 = Vec2::ZERO;
    change_in_velocity = current_acceleration.0 * time.delta_secs();
    current_velocity.0 += change_in_velocity;
    current_velocity.0.clamp_length_max(THERMAL_SPEED);
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

pub fn update_aim_position (
    mut query: Single<(&mut Transform, &crate::movement::Velocity), (With<PlayerAim>, Without<crate::player::PlayerAvatar>)>,
    time: Res<Time>
) {
    let (mut aim_position, velocity) = query.into_inner();
    let change_in_position = velocity.0 * time.delta_secs();
    aim_position.translation += change_in_position.extend(0.0);
}
