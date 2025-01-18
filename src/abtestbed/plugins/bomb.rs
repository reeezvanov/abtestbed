use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use super::super::common::CollisionMap;
use super::player::PlayerColor;

const BOMB_SIZE: Vec2 = Vec2::new(28.0, 28.0);
const BOMB_MASS: f32 = 100.0;
const BOMB_FRICTION: f32 = 0.0;
const BOMB_RESTITUTION: f32 = 0.0;

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BombSetRequested>()
            .add_event::<BombExploded>()
            .add_systems(Update, (set_bomb, explode_bomb).chain());
    }
}

#[derive(Event)]
pub struct BombSetRequested {
    pub player_id: Uuid,
    pub player_color: PlayerColor,
    pub player_transform: Transform,
    pub player_fire_range: u8,
    pub player_bomb_detonation_period: f32,
}

#[derive(Event)]
pub struct BombExploded {
    pub player_id: Uuid,
    pub bomb_color: PlayerColor,
    pub bomb_transform: Transform,
    pub bomb_fire_range: u8,
}

#[derive(Component)]
struct Bomb {
    player_id: Uuid,
    color: PlayerColor,
    fire_range: u8,
    transform: Transform,
    explode_at: Duration,
}

fn set_bomb(mut commands: Commands, mut events: EventReader<BombSetRequested>, time: Res<Time>) {
    for event in events.read() {
        commands.spawn((
            Bomb {
                player_id: event.player_id,
                color: event.player_color,
                fire_range: event.player_fire_range,
                transform: event.player_transform, // Make transform from map point wher player set bomb
                explode_at: time.elapsed()
                    + Duration::from_secs_f32(event.player_bomb_detonation_period),
            },
            Sprite {
                color: event.player_color.to_bevy_color(),
                custom_size: Some(BOMB_SIZE),
                ..Default::default()
            },
            event.player_transform, // Make transform from map point wher player set bomb
            RigidBody::Dynamic,
            Velocity::zero(),
            LockedAxes::ROTATION_LOCKED_Z,
            Collider::cuboid(BOMB_SIZE.x / 2.0, BOMB_SIZE.y / 2.0),
            CollisionGroups::new(
                Group::from_bits(CollisionMap::BOMB.0).unwrap(),
                Group::from_bits(CollisionMap::BOMB.1).unwrap(),
            ),
            ColliderMassProperties::Mass(BOMB_MASS),
            Friction::new(BOMB_FRICTION),
            Restitution::new(BOMB_RESTITUTION),
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
            bomb_transform: t.clone(),
            bomb_fire_range: b.fire_range,
        });
    }
}
