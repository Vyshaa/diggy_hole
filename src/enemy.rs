use crate::{
    background::Tile,
    player::{Hitbox, Player, Velocity},
};
use bevy::prelude::*;

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

fn move_enemy(
    time: Res<Time>,
    player_query: Single<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<(&mut Transform, &mut Hitbox, &Velocity), With<Enemy>>,
    tile_query: Query<&Hitbox, (With<Tile>, Without<Enemy>)>,
) {
    let player_pos = player_query.into_inner().translation;
    for (mut transform, mut hitbox, velocity) in enemy_query.iter_mut() {
        let dt = velocity.0 * time.delta_secs();
        let direction = player_pos - transform.translation;
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
