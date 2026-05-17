use bevy::prelude::*;

pub fn items(app: &mut App) {
    app
        .init_state::<CharacterMode>()
        .init_resource::<ScoopAngleCcw>()
        .init_resource::<Points>()
        .add_systems(Update, change_character_mode)
        .add_systems(OnExit(CharacterMode::Shooting), hide_aim_visibility)
        .add_systems(OnEnter(CharacterMode::Shooting), show_aim_visibility)
        .add_systems(Update, (catching, rotate_scoop).run_if(in_state(CharacterMode::Catching)));
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CharacterMode {
    #[default]
    Shooting,
    Catching
}

pub fn change_character_mode (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    input_mode: Res<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>,
    character_mode: Res<State<CharacterMode>>,
    mut next_character_mode: ResMut<NextState<CharacterMode>>,
) {
    match **character_mode {
        CharacterMode::Shooting => {
            match *input_mode {
                crate::movement::InputMode::Keyboard => {
                    if keyboard_input.just_pressed(KeyCode::KeyE) {
                        next_character_mode.set(CharacterMode::Catching);
                        eprintln!("New mode is catching");
                    }
                },
                crate::movement::InputMode::Controller => {
                    if let Some(gamepad) = gamepad {
                        if gamepad.just_pressed(GamepadButton::North) {
                            next_character_mode.set(CharacterMode::Catching);
                            eprintln!("New mode is catching");
                        }
                    }
                }
            }
        },
        CharacterMode::Catching => {
            match *input_mode {
                crate::movement::InputMode::Keyboard => {
                    if keyboard_input.just_pressed(KeyCode::KeyE) {
                        next_character_mode.set(CharacterMode::Shooting);
                        eprintln!("New mode is shooting");
                    }
                },
                crate::movement::InputMode::Controller => {
                    if let Some(gamepad) = gamepad {
                        if gamepad.just_pressed(GamepadButton::North) {
                            next_character_mode.set(CharacterMode::Shooting);
                            eprintln!("New mode is shooting");
                        }
                    }
                }
            }
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ScoopAngleCcw(pub f32);

impl Default for ScoopAngleCcw {
    fn default() -> Self {
        Self (90.)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Points(pub u32);

impl Default for Points {
    fn default() -> Self {
        Self (0)
    }
}

pub fn victory_score(
    waves: Res<crate::enemies::Wave>,
    points: Res<Points>,
    remaining_trash: Res<crate::enemies::RemainingTrash>,
    remaining_trawlers: Res<crate::enemies::RemainingTrawlers>,
) {
    if waves.0 == 15 {
        println!("You win, champ! Your GRAND TOTAL is: {}", points.0);
    }
}

pub fn rotate_scoop(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    input_mode: Res<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>,
    mut scoop_angle: ResMut<ScoopAngleCcw>,
    time: Res<Time>
) {
    const RATE_OF_CHANGE_PER_SEC: f32 = 180.; // 360 is a full rotation per second
    let mut angle_change: f32 = 0.;
    match *input_mode {
        crate::movement::InputMode::Controller => {
            if let Some(gamepad) = gamepad {
                if gamepad.pressed(GamepadButton::East) {
                    angle_change -= RATE_OF_CHANGE_PER_SEC * time.delta_secs();
                }
                else if gamepad.pressed(GamepadButton::West) {
                    angle_change += RATE_OF_CHANGE_PER_SEC * time.delta_secs();
                }
            }
        },
        crate::movement::InputMode::Keyboard => {
            if keyboard_input.pressed(KeyCode::PageDown) {
                    angle_change -= RATE_OF_CHANGE_PER_SEC * time.delta_secs();

            }
            else if keyboard_input.pressed(KeyCode::PageUp) {
                    angle_change += RATE_OF_CHANGE_PER_SEC * time.delta_secs();
            }
        }
    };
    if angle_change != 0. {
        scoop_angle.0 = (scoop_angle.0 + angle_change).rem_euclid(360.);
        eprintln!("New angle: {}", scoop_angle.0);
    }
}

// ##################### LAVET AF AI #####################
// Dog tilrettet

pub fn catching(
    scoop_angle: Res<ScoopAngleCcw>,
    player: Single<&Transform, With<crate::player::PlayerAvatar>>,
    mut amount_of_points: ResMut<Points>,
    mut commands: Commands,
    mut gizmos: Gizmos,
    trash_query: Query<(Entity, &Transform), With<crate::enemies::TrashPiece>>,
) {
    const INNER_RADIUS: f32 = 20.;
    const SEGMENT_LENGTH: f32 = 50.;
    const TOLERANCE: f32 = 5.;
    const CATCHING_REWARD: u32 = 25;

    let angle_rad = scoop_angle.0.to_radians();
    let dir = Vec2::new(angle_rad.cos(), angle_rad.sin());
    let origin = player.translation.truncate();
    let seg_start = origin + dir * INNER_RADIUS;
    let seg_end   = origin + dir * (INNER_RADIUS + SEGMENT_LENGTH);

    gizmos.line_2d(seg_start, seg_end, Color::srgb(1., 1., 0.));

    let ab = seg_end - seg_start;

    for (entity, trash_transform) in &trash_query {
        let point = trash_transform.translation.truncate();
        let ap = point - seg_start;
        let t = ap.dot(ab) / ab.dot(ab);

        if t < 0. || t > 1. {
            continue;
        }

        let closest = seg_start + ab * t;
        if point.distance(closest) <= TOLERANCE {
            eprintln!("Trash catched at: {}", trash_transform.translation.truncate());
            commands.entity(entity).despawn();
            amount_of_points.0 += CATCHING_REWARD;
            eprintln!("Total points is: {}", amount_of_points.0);
        }
    }
}

// #######################################################

fn hide_aim_visibility(
    mut visibility: Single<&mut Visibility, With<crate::aim::PlayerAim>>
) {
    **visibility = Visibility::Hidden; // The aim will be invisible upon leaving shooting mode
}

fn show_aim_visibility(
    mut visibility: Single<&mut Visibility, With<crate::aim::PlayerAim>>
) {
    **visibility = Visibility::Visible; // The aim will be invisible upon leaving shooting mode
}

