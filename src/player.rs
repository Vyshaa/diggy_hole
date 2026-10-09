use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Player;

#[derive(Component)]
pub(crate) struct Health(pub(crate) u32);

#[derive(Component)]
pub(crate) struct Velocity(pub(crate) f32);

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    player: Player,
    health: Health,
    velocity: Velocity,
    sprite: Sprite,
    transform: Transform,
}

impl PlayerBundle {
    pub(crate) fn new(asset_server: &AssetServer) -> Self {
        let cat = asset_server.load("cat.png");
        Self {
            player: Player,
            health: Health(100),
            velocity: Velocity(100.),
            sprite: Sprite {
                image: cat.clone(),
                custom_size: Some(Vec2::new(100., 100.)),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
        }
    }
}
