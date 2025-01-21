use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::map;

const MAIN_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
const FRICTION: f32 = 0.0;
const RADIUS: f32 = 18.0;


pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blocks);
    }
}

fn spawn_blocks(mut commands: Commands, map_state: Res<map::MapState>) {
    for j in 0..map::NET_SIZE.1 {
        for i in 0..map::NET_SIZE.0 {
            if map_state.scheme[j as usize][i as usize] != map::legend::BLOCK {
                continue;
            }

            commands.spawn((
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
                Collider::ball(RADIUS),
                Friction::new(FRICTION),
            ));
        }
    }
}