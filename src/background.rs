use bevy::prelude::*;

use crate::player::Player;

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

            commands.spawn((
                Sprite {
                    color: Color::srgb(0.6, 0.4, 0.2),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..Default::default()
                },
                Transform::from_xyz(world_x, world_y, -1.),
                Tile,
            ));
        }
    }
}

pub(crate) fn detect_dig_collision(
    player_query: Single<&Transform, With<Player>>,
    mut tile_query: Query<(Entity, &Transform), With<Tile>>,
    mut commands: Commands,
) {
    let player_pos = player_query.into_inner().translation.xy();
    let player_radius = Vec2::new(50., 50.);
    for (entity, tile_transform) in tile_query.iter_mut() {
        let delta = (player_pos - tile_transform.translation.xy()).abs();
        let threshold = player_radius;

        if delta.cmplt(threshold).all() {
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
