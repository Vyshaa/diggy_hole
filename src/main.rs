use std::time::Duration;

use bevy::prelude::*;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TileType {
    Dirt,
    Empty,
}

#[derive(Component)]
struct Tile {
    tile_type: TileType,
    grid_x: usize,
    grid_y: usize,
}

const TILE_SIZE: f32 = 32.;
const GRID_WIDTH: usize = 20;
const GRID_HEIGHT: usize = 15;

#[derive(Component)]
struct Health(u32);

#[derive(Component)]
struct Velocity(f32);

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    health: Health,
    velocity: Velocity,
    sprite: Sprite,
    transform: Transform,
}

impl PlayerBundle {
    fn new(asset_server: &AssetServer) -> Self {
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

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct TimerDisplay;

#[derive(Component)]
struct HealthDisplay;

#[derive(Resource)]
struct GameTimer(Timer);

impl GameTimer {
    fn just_finished(&mut self, delta: Duration) -> bool {
        self.0.tick(delta).just_finished()
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Startup, (set_scene, background_setup));
        app.add_systems(
            Update,
            (
                update_timer,
                update_health_bar,
                move_player,
                detect_dig_collision,
            )
                .chain(),
        );
    }
}

fn set_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, MainCamera));
    commands.spawn(PlayerBundle::new(&asset_server));

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((Text::new("0"), TimerDisplay));
            parent.spawn((Text::new("0"), HealthDisplay));
        });
}

fn background_setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Calculate grid dimensions
    let grid_width_pixels = GRID_WIDTH as f32 * TILE_SIZE;
    let grid_height_pixels = GRID_HEIGHT as f32 * TILE_SIZE;
    let start_x = -grid_width_pixels / 2.;
    let start_y = grid_height_pixels / 2.;

    // Spawn tiles in a grid
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let world_x = start_x + (x as f32) * TILE_SIZE + TILE_SIZE / 2.;
            let world_y = start_y - (y as f32) * TILE_SIZE - TILE_SIZE / 2.;

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.6, 0.4, 0.2),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform::from_xyz(world_x, world_y, -1.),
                Tile {
                    tile_type: TileType::Dirt,
                    grid_x: x,
                    grid_y: y,
                },
            ));
        }
    }
}

fn detect_dig_collision(
    player_query: Single<&Transform, With<Player>>,
    mut tile_query: Query<(Entity, &mut Tile, &Transform)>,
    mut commands: Commands,
) {
    let player_pos = player_query.into_inner().translation.xy();
    let player_radius = Vec2::new(50., 50.); // Half the player sprite size

    for (entity, mut tile, tile_transform) in tile_query.iter_mut() {
        let delta = (player_pos - tile_transform.translation.xy()).abs();
        let threshold = player_radius + Vec2::splat(TILE_SIZE / 2.);

        // If player overlaps tile and it's dirt, dig it
        if delta.cmplt(threshold).all() && tile.tile_type == TileType::Dirt {
            tile.tile_type = TileType::Empty;
            commands.entity(entity).despawn();
        }
    }
}

fn move_player(
    time: Res<Time>,
    query: Single<(&mut Sprite, &mut Transform, &Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let (mut sprite, mut transform, velocity) = query.into_inner();
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
}

fn update_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    query: Single<&mut Text, With<TimerDisplay>>,
) {
    let mut text = query.into_inner();

    if timer.just_finished(time.delta()) {
        text.0 = format!("{}", time.elapsed().as_secs());
    }
}

fn update_health_bar(
    ui_query: Single<&mut Text, With<HealthDisplay>>,
    health_query: Single<&Health, With<Player>>,
) {
    let mut text = ui_query.into_inner();
    text.0 = health_query.into_inner().0.to_string();
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .run();
}
