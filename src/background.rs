use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::player::{Hitbox, Player};

#[derive(Component)]
pub(crate) struct Tile;

const TILE_SIZE: f32 = 16.;

pub(crate) fn setup_background(mut commands: Commands, window: Single<&Window>) {
    // Calculate grid dimensions
    let grid_width_pixels = window.resolution.width() * 0.9;
    let grid_height_pixels = window.resolution.height() * 0.9;
    let grid_w = grid_width_pixels / TILE_SIZE;
    let grid_h = grid_height_pixels / TILE_SIZE;
    let start_x = -grid_width_pixels / 2.;
    let start_y = grid_height_pixels / 2.;

    // Spawn tiles in a grid
    for y in 0..grid_h as i32 {
        for x in 0..grid_w as i32 {
            let world_x = start_x + (x as f32) * TILE_SIZE + TILE_SIZE / 2.;
            let world_y = start_y - (y as f32) * TILE_SIZE - TILE_SIZE / 2.;
            if (-50..50).contains(&(world_y as i32)) {
                continue;
            }

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.6, 0.4, 0.2),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Hitbox(Aabb2d::new(
                    Vec2::new(world_x, world_y),
                    Vec2::new(TILE_SIZE / 2., TILE_SIZE / 2.),
                )),
                Transform::from_xyz(world_x, world_y, -1.),
                Tile,
            ));
        }
    }
}

pub(crate) fn detect_dig_collision(
    player_query: Single<&Hitbox, With<Player>>,
    mut tile_query: Query<(Entity, &Hitbox), With<Tile>>,
    mut commands: Commands,
) {
    let player_hitbox = player_query.into_inner().0;
    for (entity, tile_hitbox) in tile_query.iter_mut() {
        if player_hitbox.intersects(&tile_hitbox.0) {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .add_systems(Update, detect_dig_collision);
    }
}
