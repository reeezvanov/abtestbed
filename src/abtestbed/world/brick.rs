use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::map;

const FRICTION: f32 = 0.0;
const SIZE: Vec2 = Vec2::new(40.0, 36.0);
const MAIN_COLOR: Color = Color::srgb(0.1, 0.1, 0.6);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bricks);
    }
}

#[derive(Component)]
pub struct Brick;

fn spawn_bricks(mut commands: Commands, map_state: Res<map::MapState>) {
    for j in 0..map::NET_SIZE.1 {
        for i in 0..map::NET_SIZE.0 {
            if map_state.scheme[j as usize][i as usize] != map::legend::BRICK {
                continue;
            }

            commands.spawn((
                Brick,
                Sprite {
                    color: MAIN_COLOR,
                    custom_size: Some(map::CELL_SIZE),
                    ..Default::default()
                },
                Transform::from_xyz(
                    map::CELL_START_POS.x + (i as f32 * map::CELL_SIZE.x),
                    map::CELL_START_POS.y - (j as f32 * map::CELL_SIZE.y),
                    0.0,
                ),
                RigidBody::Fixed,
                ActiveEvents::COLLISION_EVENTS,
                Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
                Friction::new(FRICTION),
            ));
        }
    }
}
