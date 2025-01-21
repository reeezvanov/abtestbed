use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::explosion;
use super::map;

const FRICTION: f32 = 0.0;
const SIZE: Vec2 = Vec2::new(36.0, 36.0);
const MAIN_COLOR: Color = Color::srgb(0.1, 0.1, 0.7);

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bricks)
            .add_systems(Update, track_explosion_bricks);
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
                    custom_size: Some(SIZE),
                    ..default()
                },
                Transform::from_xyz(
                    map::CELL_START_POS.x + (i as f32 * map::CELL_SIZE.x),
                    map::CELL_START_POS.y - (j as f32 * map::CELL_SIZE.y),
                    0.0,
                ),
                RigidBody::KinematicPositionBased,
                Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
                ActiveEvents::COLLISION_EVENTS,
                Friction::new(FRICTION),
            ));
        }
    }
}

fn track_explosion_bricks(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    explosions: Query<(), With<explosion::Explosion>>,
    bricks: Query<(), With<Brick>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let brick_entity;

                if explosions.get(*e1).is_ok() && bricks.get(*e2).is_ok() {
                    brick_entity = e2;
                } else if explosions.get(*e2).is_ok() && bricks.get(*e1).is_ok() {
                    brick_entity = e1;
                } else {
                    return;
                }

                commands.entity(*brick_entity).despawn();
            }
            _ => {}
        }
    }
}
