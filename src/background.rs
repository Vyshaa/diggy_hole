use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::player::{Hitbox, Player};

#[derive(Component)]
pub struct Tile;

pub(crate) const TILE_SIZE: f32 = 16.;

pub fn setup_background(mut commands: Commands, window: Single<&Window>) {
    let win_w = window.resolution.width();
    let win_h = window.resolution.height();
    let width = (win_w / TILE_SIZE) as u32;
    let height = (win_h / TILE_SIZE) as u32;

    let map_size = TilemapSize {
        x: width,
        y: height,
    };
    let map_entity = commands.spawn_empty().id();
    let mut storage = TileStorage::empty(map_size);

    for row in 0..height {
        let world_y = win_h / 2. - (row as f32 + 0.5) * TILE_SIZE;

        if (-32..=32).contains(&(world_y as i32)) {
            continue;
        }

        for x in 0..width {
            let world_x = -win_w / 2. + (x as f32 + 0.5) * TILE_SIZE;
            let position = TilePos {
                x,
                y: height - 1 - row,
            };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position,
                        tilemap_id: TilemapId(map_entity),
                        ..Default::default()
                    },
                    Sprite {
                        color: Color::srgb(0.6, 0.4, 0.2),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..Default::default()
                    },
                    Hitbox(Aabb2d::new(
                        Vec2::new(world_x, world_y),
                        Vec2::splat(TILE_SIZE / 2.),
                    )),
                    Transform::from_xyz(world_x, world_y, -1.),
                    Tile,
                ))
                .id();

            storage.set(&position, tile_entity);
        }
    }

    let tile_size = TilemapTileSize {
        x: TILE_SIZE,
        y: TILE_SIZE,
    };
    // Center the map while keeping its tiles aligned with the old grid.
    let map_center = Vec3::new(
        -win_w / 2. + width as f32 * TILE_SIZE / 2.,
        win_h / 2. - height as f32 * TILE_SIZE / 2.,
        -1.,
    );

    commands.entity(map_entity).insert((TilemapBundle {
        grid_size: tile_size.into(),
        map_type: TilemapType::default(),
        size: map_size,
        storage,
        tile_size,
        anchor: TilemapAnchor::Center,
        transform: Transform::from_translation(map_center),
        ..Default::default()
    },));
}

pub fn detect_dig_collision(
    player_query: Single<&Hitbox, With<Player>>,
    mut tile_query: Query<(Entity, &Hitbox), With<Tile>>,
    mut commands: Commands,
) {
    let player_hitbox = player_query.into_inner().0;
    for (entity, tile_hitbox) in tile_query.iter_mut() {
        if player_hitbox.intersects(&tile_hitbox.0) {
            commands
                .entity(entity)
                .remove::<Hitbox>()
                .insert(Visibility::Hidden);
        }
    }
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
            .add_systems(Update, detect_dig_collision);
    }
}
