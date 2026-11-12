use crate::{
    background::Tile,
    grid_geometry::{GridGeometry, InBounds},
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
    fn new(pos: f32) -> Self {
        Self {
            enemy: Enemy,
            velocity: Velocity(80.),
            sprite: Sprite {
                color: Color::srgb(1., 0., 0.),
                custom_size: Some(Vec2::new(64., 64.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(pos, 0., 0.),
            hitbox: Hitbox(Aabb2d::new(Vec2::new(pos, 0.), Vec2::new(32.0, 32.0))),
        }
    }
}

fn setup_enemies(mut commands: Commands) {
    commands.spawn(EnemyBundle::new(-600.));
    commands.spawn(EnemyBundle::new(600.));
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

fn find_path(
    start: IVec2,
    goal: IVec2,
    enemy_world_pos: Vec2,
    enemy_hitbox: Aabb2d,
    tile_hitboxes: &[Aabb2d],
    geometry: &Res<GridGeometry>,
) -> Option<Vec<IVec2>> {
    let hitbox = |next: &IVec2| {
        let candidate_center = geometry.tile_center(next.x as u32, next.y as u32);
        enemy_hitbox.translated_by(candidate_center - enemy_world_pos)
    };
    astar(
        &start,
        |&pos| {
            [
                pos + IVec2::X,
                pos - IVec2::X,
                pos + IVec2::Y,
                pos - IVec2::Y,
            ]
            .into_iter()
            .filter(|next| {
                geometry.in_bounds(next)
                    && !tile_hitboxes
                        .iter()
                        .any(|tile| hitbox(next).intersects(tile))
            })
            .map(|next| (next, 1))
        },
        |&pos| pos.x.abs_diff(goal.x) + pos.y.abs_diff(goal.y),
        |&pos| pos == goal,
    )
    .map(|(path, _cost)| path)
}

fn move_enemy(
    time: Res<Time>,
    player_query: Single<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &mut Hitbox, &Velocity), With<Enemy>>,
    tile_query: Query<(&Hitbox, &Transform), (With<Tile>, Without<Enemy>)>,
    geometry: Res<GridGeometry>,
) {
    let player_pos = player_query.into_inner().translation;
    let Some(player_pos) = geometry.world_to_grid(player_pos.xy()) else {
        return;
    };
    let tile_hitboxes: Vec<Aabb2d> = tile_query.iter().map(|(hitbox, _)| hitbox.0).collect();

    for (mut transform, mut hitbox, velocity) in enemy_query.iter_mut() {
        let dt = velocity.0 * time.delta_secs();

        let Some(enemy_pos) = geometry.world_to_grid(transform.translation.xy()) else {
            continue;
        };

        let Some(path) = find_path(
            enemy_pos,
            player_pos,
            transform.translation.xy(),
            hitbox.0,
            &tile_hitboxes,
            &geometry,
        ) else {
            continue;
        };
        let Some(next) = path.get(1) else { continue };

        let direction = Vec2::new(
            (next.x - enemy_pos.x) as f32,
            -(next.y - enemy_pos.y) as f32,
        );
        let movement = direction.normalize_or_zero() * dt;

        transform.translation += movement.extend(0.);
        hitbox.0.translate_by(movement);
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_enemies)
            .add_systems(Update, (move_enemy, player_collision).chain());
    }
}
