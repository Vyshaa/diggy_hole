use crate::{
    background::{TILE_SIZE, Tile},
    player::{Hitbox, Player, Velocity},
};
use bevy::prelude::*;
use pathfinding::prelude::astar;

#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    velocity: Velocity,
    sprite: Sprite,
    transform: Transform,
    hitbox: Hitbox,
}

impl EnemyBundle {
    fn new() -> Self {
        Self {
            enemy: Enemy,
            velocity: Velocity(100.),
            sprite: Sprite {
                color: Color::srgb(1., 0., 0.),
                custom_size: Some(Vec2::new(64., 64.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-600., 0., 0.),
            hitbox: Hitbox(Aabb2d::new(Vec2::new(-600., 0.), Vec2::new(32.0, 32.0))),
        }
    }
}

fn setup_enemies(mut commands: Commands) {
    commands.spawn(EnemyBundle::new());
}

fn player_collision(
    player_query: Single<(Entity, &Hitbox), With<Player>>,
    enemy_query: Query<&Hitbox, With<Enemy>>,
    mut commands: Commands,
) {
    let (player, player_hitbox) = player_query.into_inner();
    for enemy_hitbox in &enemy_query {
        if enemy_hitbox.0.intersects(&player_hitbox.0) {
            commands.entity(player).despawn();
        }
    }
}

//grid starts at (0,0) in the top left, world starts at (0, 0) center of the screen
fn world_to_grid(world_pos: Vec2, window: &Window) -> (i32, i32) {
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    let grid_x = ((win_w / 2. + world_pos.x) / TILE_SIZE).floor() as i32;
    let grid_y = ((win_h / 2. - world_pos.y) / TILE_SIZE).floor() as i32;
    (grid_x, grid_y)
}

fn grid_to_world((x, y): (i32, i32), window: &Window) -> Vec2 {
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    Vec2::new(
        -win_w / 2.0 + (x as f32 + 0.5) * TILE_SIZE,
        win_h / 2.0 - (y as f32 + 0.5) * TILE_SIZE,
    )
}

fn find_path(start: (i32, i32), goal: (i32, i32)) -> Option<Vec<(i32, i32)>> {
    astar(
        &start,
        |&(x, y)| {
            [
                (x, y + 1),
                (x, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x + 1, y + 1),
                (x - 1, y + 1),
                (x + 1, y - 1),
                (x - 1, y - 1),
            ]
            .into_iter()
            .map(|p| (p, 1)) // each move costs 1
        },
        |&(x, y)| x.abs_diff(goal.0) + y.abs_diff(goal.1), // Manhattan distance
        |&p| p == goal,
    )
    .map(|(path, _cost)| path)
}

fn move_enemy(
    time: Res<Time>,
    player_query: Single<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &mut Hitbox, &Velocity), With<Enemy>>,
    tile_query: Query<&Hitbox, (With<Tile>, Without<Enemy>)>,
    window: Single<&Window>,
) {
    let player_pos = player_query.into_inner().translation;
    for (mut transform, mut hitbox, velocity) in enemy_query.iter_mut() {
        let dt = velocity.0 * time.delta_secs();

        let enemy_pos = world_to_grid(transform.translation.xy(), &window);
        let direction = find_path(enemy_pos, world_to_grid(player_pos.xy(), &window));
        let direction = direction
            .and_then(|path| path.get(1).copied())
            .map(|(x, y)| Vec2::new((x - enemy_pos.0) as f32, -(y - enemy_pos.1) as f32))
            .unwrap()
            .extend(0.);
        let movement = direction.normalize_or_zero() * dt;
        if tile_query.iter().any(|tile_hitbox| {
            hitbox
                .0
                .translated_by(movement.xy())
                .intersects(&tile_hitbox.0)
        }) {
            continue;
        }

        transform.translation += movement;
        hitbox.0.translate_by(movement.xy());
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, (move_enemy, player_collision).chain());
    }
}
