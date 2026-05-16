use bevy::prelude::*;

pub fn wrap_non_wrap(app: &mut App) {
    app
        .add_systems(Update, (wrap_around_map, clamp_to_map));
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self (Vec2::ZERO)
    }
}

#[derive(Component)]
pub struct Acceleration(pub Vec2);

impl Default for Acceleration {
    fn default() -> Self {
        Self (Vec2::ZERO)
    }
}

#[derive(Resource, Default, PartialEq, Eq)]
pub enum InputMode {
    #[default]
    Keyboard,
    Controller
}

pub fn change_inputmode(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mode: ResMut<crate::movement::InputMode>,
    gamepad: Option<Single<&Gamepad>>,
) {
    match *mode {
        crate::movement::InputMode::Keyboard => {
            if let Some(gamepad) = gamepad {
                *mode = crate::movement::InputMode::Controller;
            } 
        },
        crate::movement::InputMode::Controller => {
            if gamepad.is_none() {
                *mode = crate::movement::InputMode::Keyboard;
            } 
        }
    }
}

#[derive(Component)]
pub struct Wrappable;

#[derive(Component)]
pub struct NonWrappable;

pub fn wrap_around_map(
    mut query: Query<&mut Transform, With<Wrappable>>,
) {
    let half_width = crate::pixel_grid::RES_WIDTH as f32 / 2.0;
    let half_height = crate::pixel_grid::RES_HEIGHT as f32 / 2.0;

    for mut transform in query.iter_mut() {
        if transform.translation.x > half_width {
            transform.translation.x = -half_width;
        } else if transform.translation.x < -half_width {
            transform.translation.x = half_width;
        }

        if transform.translation.y > half_height {
            transform.translation.y = -half_height;
        } else if transform.translation.y < -half_height {
            transform.translation.y = half_height;
        }
    }
}

pub fn clamp_to_map(
    mut query: Query<(&mut Transform, &Sprite), With<NonWrappable>>,
) {
    let half_width = crate::pixel_grid::RES_WIDTH as f32 / 2.0;
    let half_height = crate::pixel_grid::RES_HEIGHT as f32 / 2.0;

    for (mut transform, sprite) in query.iter_mut() {
        let size = sprite.custom_size.unwrap_or(Vec2::splat(8.0));
        let half_w = size.x / 2.0;
        let half_h = size.y / 2.0;

        transform.translation.x = transform.translation.x.clamp(-half_width + half_w, half_width - half_w);
        transform.translation.y = transform.translation.y.clamp(-half_height + half_h, half_height - half_h);
    }
}

