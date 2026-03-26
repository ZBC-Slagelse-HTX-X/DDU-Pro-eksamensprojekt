use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerAvatar;

pub fn spawn_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    const PLAYER_OUTER_RING: f32 = 5.;
    const PLAYER_INNER_RING: f32 = 2.5;
    const PLAYER_COLOR: Color = Color::hsl(360.0, 0.95, 0.7);
    let player_ring = meshes.add(Ring::new(Circle::new(PLAYER_OUTER_RING), Circle::new(PLAYER_INNER_RING)));
    const AIM_OUTER_RING: f32 = 2.5;
    const AIM_INNER_RING: f32 = 1.25;
    const AIM_COLOR: Color = Color::hsl(240.0, 0.95, 0.5);
    let aim_ring = meshes.add(Ring::new(Circle::new(AIM_OUTER_RING), Circle::new(AIM_INNER_RING))); 
    commands
        .spawn((
            PlayerAvatar,
            Mesh2d(player_ring),
            MeshMaterial2d(materials.add(PLAYER_COLOR)),
            Transform::from_xyz(0., 0., 0.),
            crate::movement::Velocity::default(),
            crate::movement::Acceleration::default(),
            children![(
                crate::aim::PlayerAim,
                Mesh2d(aim_ring),
                MeshMaterial2d(materials.add(AIM_COLOR)),
                Transform::from_xyz(0., 0., 0.),
                crate::movement::Velocity::default(),
                crate::movement::Acceleration::default()
            )]
        ));
}
pub fn player_movement_plugin(app: &mut App) {
    app
        .add_systems(Update, (change_player_acceleration, change_player_velocity, update_player_position));
}

pub fn change_player_acceleration (
    mut current_acceleration: Single<&mut crate::movement::Acceleration, (With<PlayerAvatar>, Without<crate::aim::PlayerAim>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mode: Res<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>
) {
    const SPEED_OF_ACCELERATION: f32 = 3000.;
    let mut weighed_direction: Vec2 = Vec2::ZERO;
    // Checks for current movement; whether to use keyboard or mouse
    match *mode {
        crate::movement::InputMode::Keyboard => {
            if keyboard_input.pressed(KeyCode::KeyW) {
                weighed_direction.y += 0.1;
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                weighed_direction.x -= 0.1;
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                weighed_direction.y -= 0.1;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                weighed_direction.x += 0.1;
            }
        },
        crate::movement::InputMode::Controller => {
            // Uses left stick for movement
            if let Some(gamepad) = gamepad {
                const THRESHOLD: f32 = 0.025;
                const SENSITIVITY: f32 = 0.5;
                let current_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
                let current_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
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

pub fn change_player_velocity (
    mut current_velocity: Single<&mut crate::movement::Velocity, (With<PlayerAvatar>, Without<crate::aim::PlayerAim>)>,
    current_acceleration: Single<&crate::movement::Acceleration, (With<PlayerAvatar>, Without<crate::aim::PlayerAim>)>,
    time: Res<Time>
)  {
    const THERMAL_SPEED: f32 = 125.;
    let mut change_in_velocity: Vec2 = Vec2::ZERO;
    change_in_velocity = current_acceleration.0 * time.delta_secs();
    current_velocity.0 += change_in_velocity;
    current_velocity.0 = current_velocity.0.clamp_length_max(THERMAL_SPEED);
}

pub fn update_player_position(
    player_query: Single<(&mut Transform, &crate::movement::Velocity), (With<PlayerAvatar>, Without<crate::aim::PlayerAim>)>,
    time: Res<Time>
) {
    let (mut position, velocity) = player_query.into_inner();
    let change_in_position = velocity.0 * time.delta_secs();
    position.translation += change_in_position.extend(0.0);
}


