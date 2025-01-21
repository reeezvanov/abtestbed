use bevy::prelude::*;
use bevy_rapier2d::na::ComplexField;

pub const SIZE: Vec2 = Vec2::new(600.0, 392.0);
pub const NET_SIZE: (u8, u8) = (15, 11);
pub const CELL_SIZE: Vec2 = Vec2::new(40.0, 36.0);
pub const CELL_START_POS: Vec2 = Vec2::new(
    -(SIZE.x / 2.0) + (CELL_SIZE.x / 2.0),
    (SIZE.y / 2.0) - (CELL_SIZE.y / 2.0),
);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapState::default());
    }
}

#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq)]
pub struct Cell(pub u8, pub u8);

impl Cell {
    pub fn from_transform(transform: &Transform) -> Self {
        let position = Vec2::new(
            ComplexField::round((transform.translation.x - CELL_START_POS.x) / CELL_SIZE.x),
            ComplexField::round((-transform.translation.y + CELL_START_POS.y) / CELL_SIZE.y),
        );

        Cell(position.x as u8, position.y as u8)
    }

    pub fn center(&self) -> Transform {
        Transform::from_xyz(
            CELL_START_POS.x + (self.0 as f32 * CELL_SIZE.x),
            CELL_START_POS.y - (self.1 as f32 * CELL_SIZE.y),
            0.0,
        )
    }
}

pub mod legend {
    pub const EMPTY: u8 = 0;
    pub const BLOCK: u8 = 1;
    pub const BRICK: u8 = 2;
}

#[derive(Resource)]
pub struct MapState {
    pub scheme: [[u8; 15]; 11],
}

impl Default for MapState {
    fn default() -> Self {
        MapState {
            scheme: [
                [0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [0, 1, 0, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [2, 1, 0, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
                [2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2],
                [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            ],
        }
    }
}
