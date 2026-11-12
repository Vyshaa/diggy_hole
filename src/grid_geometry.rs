use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::background::TILE_SIZE;

#[derive(Resource, Clone, Copy)]
pub struct GridGeometry {
    window_size: Vec2,
    columns: u32,
    rows: u32,
}

impl GridGeometry {
    pub fn from_window(window: &Window) -> Self {
        let window_size = Vec2::new(window.resolution.width(), window.resolution.height());

        Self {
            window_size,
            columns: (window_size.x / TILE_SIZE).floor() as u32,
            rows: (window_size.y / TILE_SIZE).floor() as u32,
        }
    }

    pub fn columns(self) -> u32 {
        self.columns
    }

    pub fn rows(self) -> u32 {
        self.rows
    }

    /// `row` is counted from the top of the window, as in background setup.
    pub fn tile_center(self, column: u32, row: u32) -> Vec2 {
        Vec2::new(
            -self.window_size.x * 0.5 + (column as f32 + 0.5) * TILE_SIZE,
            self.window_size.y * 0.5 - (row as f32 + 0.5) * TILE_SIZE,
        )
    }

    /// Converts a world position to a top-down grid coordinate.
    /// Returns `None` when the position lies outside the grid.
    pub fn world_to_grid(self, position: Vec2) -> Option<IVec2> {
        let grid = IVec2::new(
            ((position.x + self.window_size.x * 0.5) / TILE_SIZE).floor() as i32,
            ((self.window_size.y * 0.5 - position.y) / TILE_SIZE).floor() as i32,
        );

        (grid.x >= 0 && grid.y >= 0 && grid.x < self.columns as i32 && grid.y < self.rows as i32)
            .then_some(grid)
    }

    /// Tilemap y is bottom-up; setup's `row` coordinate is top-down.
    pub fn to_tile_pos(self, column: u32, row: u32) -> TilePos {
        TilePos {
            x: column,
            y: self.rows - 1 - row,
        }
    }
}

pub trait InBounds<T> {
    fn in_bounds(self, vec: &T) -> bool;
}

impl InBounds<IVec2> for GridGeometry {
    fn in_bounds(self, vec: &IVec2) -> bool {
        vec.x >= 0 && vec.y >= 0 && vec.x < self.columns() as i32 && vec.y < self.rows() as i32
    }
}
impl InBounds<Vec2> for GridGeometry {
    fn in_bounds(self, vec: &Vec2) -> bool {
        let Some(vec) = self.world_to_grid(*vec) else {
            return false;
        };
        self.in_bounds(&vec)
    }
}
