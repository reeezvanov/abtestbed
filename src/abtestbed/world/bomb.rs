use std::collections::HashSet;
use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use super::explosion;
use super::map;
use super::player;
use crate::abtestbed::setup;

pub const DEFAULT_DETONATION_PERIOD: f32 = 2.0;
const SIZE: Vec2 = Vec2::new(40.0, 36.0);
const MASS: f32 = 100.0;
const FRICTION: f32 = 0.0;
const RESTITUTION: f32 = 0.0;

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlantedBombs::default())
            .add_event::<BombPlanted>()
            .add_event::<BombExploded>()
            .add_systems(
                Update,
                (
                    set_bomb,
                    explode_bomb,
                    track_planted_bombs,
                    track_explosion_bombs,
                    track_player_gone,
                )
                    .chain(),
            );
    }
}

#[derive(Resource, Default)]
pub struct PlantedBombs {
    pub set: HashSet<map::Cell>,
}

#[derive(Event)]
pub struct BombPlanted {
    pub player_id: Uuid,
    pub player_color: player::PlayerColor,
    pub player_cell: map::Cell,
    pub player_fire_range: u8,
    pub player_bomb_detonation_period: f32,
}

#[derive(Event)]
pub struct BombExploded {
    pub player_id: Uuid,
    pub player_color: player::PlayerColor,
    pub bomb_cell: map::Cell,
    pub bomb_fire_range: u8,
}

#[derive(Component)]
pub struct Bomb {
    pub player_id: Uuid,
    pub player_color: player::PlayerColor,
    pub fire_range: u8,
    players_at_bomb_count: u8,
    explode_at: Duration,
}

fn set_bomb(mut commands: Commands, mut events: EventReader<BombPlanted>, time: Res<Time>) {
    for event in events.read() {
        commands.spawn((
            Bomb {
                player_id: event.player_id,
                player_color: event.player_color,
                fire_range: event.player_fire_range,
                players_at_bomb_count: 0,
                explode_at: time.elapsed()
                    + Duration::from_secs_f32(event.player_bomb_detonation_period),
            },
            Sprite {
                color: event.player_color.to_bevy_color(),
                custom_size: Some(SIZE),
                ..default()
            },
            event.player_cell.center(),
            RigidBody::Dynamic,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            LockedAxes::TRANSLATION_LOCKED | LockedAxes::ROTATION_LOCKED,
            Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
            CollisionGroups::new(
                Group::from_bits(setup::collision::policy::BOMB.0).unwrap(),
                Group::from_bits(setup::collision::policy::BOMB.1).unwrap(),
            ),
            ColliderMassProperties::Mass(MASS),
            Friction::new(FRICTION),
            Restitution::new(RESTITUTION),
            ExternalForce::default(),
        ));
    }
}

fn explode_bomb(
    mut commands: Commands,
    query: Query<(Entity, &Bomb, &Transform)>,
    time: Res<Time>,
    mut events: EventWriter<BombExploded>,
) {
    for (e, b, t) in &query {
        if time.elapsed() < b.explode_at {
            continue;
        }

        commands.entity(e).despawn();

        events.send(BombExploded {
            player_id: b.player_id,
            player_color: b.player_color,
            bomb_cell: map::Cell::from_transform(t),
            bomb_fire_range: b.fire_range,
        });
    }
}

fn track_player_gone(
    mut commands: Commands, 
    mut collision_events: EventReader<CollisionEvent>,
    mut bombs: Query<(Entity, &mut Bomb), With<Sensor>>,
    players: Query<(), With<player::Player>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let bomb_entity;

                if players.get(*e1).is_ok() && bombs.get(*e2).is_ok() {
                    bomb_entity = e2;
                } else if players.get(*e2).is_ok() && bombs.get(*e1).is_ok() {
                    bomb_entity = e1;
                } else {
                    return;
                }

                for (e, mut b) in &mut bombs {
                    if e.index() == bomb_entity.index() {
                        b.players_at_bomb_count += 1;
                    }
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                let bomb_entity;

                if players.get(*e1).is_ok() && bombs.get(*e2).is_ok() {
                    bomb_entity = e2;
                } else if players.get(*e2).is_ok() && bombs.get(*e1).is_ok() {
                    bomb_entity = e1;
                } else {
                    return;
                }

                for (e, mut b) in &mut bombs {
                    if e.index() == bomb_entity.index() {
                        b.players_at_bomb_count -= 1;

                        if b.players_at_bomb_count == 0 {
                            commands.entity(*bomb_entity).remove::<Sensor>();
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn track_planted_bombs(
    mut bomb_planted_events: EventReader<BombPlanted>,
    mut bomb_exploded_events: EventReader<BombExploded>,
    mut planted_bombs: ResMut<PlantedBombs>,
) {
    for bp_event in bomb_planted_events.read() {
        planted_bombs.set.insert(bp_event.player_cell);
    }

    for be_event in bomb_exploded_events.read() {
        planted_bombs.set.remove(&be_event.bomb_cell);
    }
}

fn track_explosion_bombs(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut bomb_exploded_events: EventWriter<BombExploded>,
    explosions: Query<(), With<explosion::Explosion>>,
    bombs: Query<(Entity, &Bomb, &Transform)>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let bomb_entity;

                if explosions.get(*e1).is_ok() && bombs.get(*e2).is_ok() {
                    bomb_entity = e2;
                } else if explosions.get(*e2).is_ok() && bombs.get(*e1).is_ok() {
                    bomb_entity = e1;
                } else {
                    return;
                }

                for (e, b, t) in &bombs {
                    if e.index() == bomb_entity.index() {
                        bomb_exploded_events.send(BombExploded {
                            player_id: b.player_id,
                            player_color: b.player_color,
                            bomb_cell: map::Cell::from_transform(t),
                            bomb_fire_range: b.fire_range,
                        });
                    }
                }

                commands.entity(*bomb_entity).despawn();
            }
            _ => {}
        }
    }
}
