use std::time::Duration;
use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use super::map;
use super::player;
use crate::abtestbed::setup;

pub const DEFAULT_DETONATION_PERIOD: f32 = 3.0;
const SIZE: Vec2 = Vec2::new(32.0, 32.0);
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
                    handle_collision_events,
                    track_planted_bombs,
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
    pub bomb_color: player::PlayerColor,
    pub bomb_cell: map::Cell,
    pub bomb_fire_range: u8,
}

#[derive(Component)]
struct Bomb {
    player_id: Uuid,
    color: player::PlayerColor,
    fire_range: u8,
    explode_at: Duration,
}

fn set_bomb(mut commands: Commands, mut events: EventReader<BombPlanted>, time: Res<Time>) {
    for event in events.read() {
        commands.spawn((
            Bomb {
                player_id: event.player_id,
                color: event.player_color,
                fire_range: event.player_fire_range,
                explode_at: time.elapsed()
                    + Duration::from_secs_f32(event.player_bomb_detonation_period),
            },
            Sprite {
                color: event.player_color.to_bevy_color(),
                custom_size: Some(SIZE),
                ..default()
            },
            event.player_cell.center(),
            RigidBody::Fixed,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED_Z,
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
            bomb_color: b.color,
            bomb_cell: map::Cell::from_transform(t),
            bomb_fire_range: b.fire_range,
        });
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

fn handle_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    players: Query<(), With<player::Player>>,
    bombs: Query<(), With<Bomb>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                if players.get(*e1).is_ok() || bombs.get(*e2).is_ok() {
                    // println!("Collision started: {:?} is Player and {:?} is Bomb", *e1, *e2);
                } else if players.get(*e2).is_ok() || bombs.get(*e1).is_ok() {
                    // println!("Collision started: {:?} is Player and {:?} is Bomb", *e2, *e1);
                } else {
                    ()
                }
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                if players.get(*e1).is_ok() || bombs.get(*e2).is_ok() {
                    // println!("Collision stopped: {:?} is Player and {:?} is Bomb", *e1, *e2);
                    if commands.get_entity(*e2).is_some() {
                        commands.entity(*e2).remove::<Sensor>();
                    }
                } else if players.get(*e2).is_ok() || bombs.get(*e1).is_ok() {
                    // println!("Collision stopped: {:?} is Player and {:?} is Bomb", *e2, *e1);
                    if commands.get_entity(*e1).is_some() {
                        commands.entity(*e1).remove::<Sensor>();
                    }
                } else {
                    return;
                }
            }
        }
    }
}
