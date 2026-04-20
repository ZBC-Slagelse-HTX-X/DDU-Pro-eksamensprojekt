pub fn spawn_monster(
    mut commands: Commands, asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    const AIM_OUTER_RING: f32 = 2.5;
    const AIM_INNER_RING: f32 = 1.25;
    const AIM_COLOR: Color = Color::hsl(240.0, 0.95, 0.5);
    let aim_ring = meshes.add(Ring::new(Circle::new(AIM_OUTER_RING), Circle::new(AIM_INNER_RING)));
    // The sample sprite that will be rendered to the pixel-perfect canvas
    commands.spawn((
        PlayerAvatar,
        Sprite::from_image(asset_server.load("sprites/boat/syd_øst.png")),
        Transform::from_xyz(0., 0., 1.),
        crate::movement::Velocity::default(),
        crate::movement::Acceleration::default(),
        crate::pixel_grid::PIXEL_PERFECT_LAYERS,
    ));
}
