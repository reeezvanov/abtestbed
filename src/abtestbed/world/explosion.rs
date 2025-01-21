use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::bomb;
use super::map;

const SIZE: Vec2 = Vec2::new(36.0, 32.0);
const DEFAULT_EXPLOSIAN_PERIOD: f32 = 0.8;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_explosion, extingush_explosion));
    }
}

#[derive(Component, Default)]
pub struct Explosion {
    extinguish_at: Duration,
}

fn spawn_explosion(
    mut commands: Commands,
    mut bomb_exploded_events: EventReader<bomb::BombExploded>,
    map_state: Res<map::MapState>,
) {
    for be_event in bomb_exploded_events.read() {
        let mut cells = Vec::<map::Cell>::new();

        // Generate middle
        cells.push(be_event.bomb_cell);

        // Generate north side
        let x = std::cmp::max(
            0,
            be_event.bomb_cell.1 as i8 - be_event.bomb_fire_range as i8,
        ) as u8;
        let y = be_event.bomb_cell.1;
        for j in (x..y).rev() {
            let cell = map::Cell(be_event.bomb_cell.0, j);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BLOCK {
                break;
            }

            cells.push(cell);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BRICK {
                break;
            }
        }

        // Generate south side
        let x = be_event.bomb_cell.1 + 1;
        let y = std::cmp::min(
            map::NET_SIZE.1 - 1,
            be_event.bomb_cell.1 + be_event.bomb_fire_range,
        ) + 1;
        for j in x..y {
            let cell = map::Cell(be_event.bomb_cell.0, j);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BLOCK {
                break;
            }

            cells.push(cell);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BRICK {
                break;
            }
        }

        // Generate west side
        let x = std::cmp::max(
            0,
            be_event.bomb_cell.0 as i8 - be_event.bomb_fire_range as i8,
        ) as u8;
        let y = be_event.bomb_cell.0;
        for i in (x..y).rev() {
            let cell = map::Cell(i, be_event.bomb_cell.1);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BLOCK {
                break;
            }

            cells.push(cell);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BRICK {
                break;
            }
        }

        // Generate east side
        let x = be_event.bomb_cell.0 + 1;
        let y = std::cmp::min(
            map::NET_SIZE.0 - 1,
            be_event.bomb_cell.0 + be_event.bomb_fire_range,
        ) + 1;
        for i in x..y {
            let cell = map::Cell(i, be_event.bomb_cell.1);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BLOCK {
                break;
            }

            cells.push(cell);

            if map_state.scheme[cell.0 as usize][cell.1 as usize] == map::legend::BRICK {
                break;
            }
        }

        for cell in cells {
            commands.spawn((
                Explosion::default(),
                Sprite {
                    color: be_event.player_color.to_bevy_color(),
                    custom_size: Some(SIZE),
                    ..default()
                },
                cell.center(),
                RigidBody::Dynamic,
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
            ));
        }
    }
}

fn extingush_explosion(
    mut commands: Commands,
    query: Query<(Entity, &Explosion)>,
    time: Res<Time>,
) {
    for (e, expl) in &query {
        if time.elapsed() < expl.extinguish_at {
            continue;
        }

        commands.entity(e).despawn();
    }
}
