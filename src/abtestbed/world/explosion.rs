use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::bomb;
use super::map;

const SIZE: Vec2 = Vec2::new(28.0, 28.0);
const DEFAULT_EXPLOSIAN_PERIOD: f32 = 4.0;

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
    time: Res<Time>,
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
        for j in x..y {
            cells.push(map::Cell(be_event.bomb_cell.0, j));
        }

        // Generate south side
        let x = be_event.bomb_cell.1 + 1;
        let y = std::cmp::min(
            map::NET_SIZE.1 - 1,
            be_event.bomb_cell.1 + be_event.bomb_fire_range,
        ) + 1;
        for j in x..y {
            cells.push(map::Cell(be_event.bomb_cell.0, j));
        }

        // Generate west side
        let x = std::cmp::max(
            0,
            be_event.bomb_cell.0 as i8 - be_event.bomb_fire_range as i8,
        ) as u8;
        let y = be_event.bomb_cell.0;
        for i in x..y {
            cells.push(map::Cell(i, be_event.bomb_cell.1));
        }

        // Generate east side
        let x = be_event.bomb_cell.0 + 1;
        let y = std::cmp::min(
            map::NET_SIZE.0 - 1,
            be_event.bomb_cell.0 + be_event.bomb_fire_range,
        ) + 1;
        for i in x..y {
            cells.push(map::Cell(i, be_event.bomb_cell.1));
        }
        
        for cell in cells {
            commands.spawn((
                Explosion {
                    extinguish_at: time.elapsed()
                        + Duration::from_secs_f32(DEFAULT_EXPLOSIAN_PERIOD),
                },
                Sprite {
                    color: be_event.bomb_color.to_bevy_color(),
                    custom_size: Some(SIZE),
                    ..default()
                },
                cell.center(),
                RigidBody::Fixed,
                Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
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
