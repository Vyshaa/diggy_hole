use bevy::{
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*,
};

#[derive(Component)]
pub(crate) struct Player;

#[derive(Component)]
pub(crate) struct Velocity(f32);

#[derive(Component)]
pub(crate) struct Hitbox(pub(crate) Aabb2d);

#[derive(Bundle)]
pub(crate) struct PlayerBundle {
    player: Player,
    velocity: Velocity,
    sprite: Sprite,
    transform: Transform,
    hitbox: Hitbox,
}

impl PlayerBundle {
    pub(crate) fn new(asset_server: &AssetServer) -> Self {
        let cat = asset_server.load("cat.png");
        Self {
            player: Player,
            velocity: Velocity(100.),
            sprite: Sprite {
                image: cat.clone(),
                custom_size: Some(Vec2::new(100., 100.)),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.),
            hitbox: Hitbox(Aabb2d::new(Vec2::ZERO, Vec2::new(50.0, 50.0))),
        }
    }
}

#[derive(Component)]
struct MainCamera;

pub(crate) fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));
    commands.spawn(PlayerBundle::new(&asset_server));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    Up,
    Down,
    Left,
    Right,
}

fn pressed(input: &ButtonInput<KeyCode>, action: Action) -> bool {
    match action {
        Action::Up => input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]),
        Action::Down => input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]),
        Action::Left => input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]),
        Action::Right => input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]),
    }
}

pub(crate) fn move_player(
    time: Res<Time>,
    query: Single<(&mut Sprite, &mut Transform, &mut Hitbox, &Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut sprite, mut transform, mut hitbox, velocity) = query.into_inner();
    let dt = velocity.0 * time.delta_secs();
    let mut direction = Vec2::ZERO;

    if pressed(&keyboard_input, Action::Right) {
        direction.x = 1.;
        sprite.flip_x = true;
    } else if pressed(&keyboard_input, Action::Left) {
        direction.x = -1.;
        sprite.flip_x = false;
    }

    if pressed(&keyboard_input, Action::Up) {
        direction.y = 1.;
    } else if pressed(&keyboard_input, Action::Down) {
        direction.y = -1.;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }
    transform.translation += (direction * dt).extend(0.);
    *hitbox = Hitbox(Aabb2d::new(transform.translation.xy(), Vec2::new(50., 50.)));
}

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, move_player);
    }
}
