use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use super::bomb;
use super::explosion;
use super::map;
use crate::abtestbed::setup;

const DEFAULT_SPEED: f32 = 70.0;
const DEFAULT_FIRE_RANGE: u8 = 2;
const DEFAULT_BOMB_CAPACITY: u8 = 1;

const SIZE: Vec2 = Vec2::new(27.0, 27.0);
const MASS: f32 = 100.0;
const FRICTION: f32 = 0.0;
const RESTITUTION: f32 = 0.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players).add_systems(
            Update,
            (
                update_player_input,
                movement_system,
                handle_bomb_exploded,
                track_explosion_players,
            ),
        );
    }
}
struct ControlKeys {
    move_north: KeyCode,
    move_south: KeyCode,
    move_west: KeyCode,
    move_east: KeyCode,
    set_bomb: KeyCode,
}

impl std::default::Default for ControlKeys {
    fn default() -> Self {
        ControlKeys {
            move_north: KeyCode::ArrowUp,
            move_south: KeyCode::ArrowDown,
            move_west: KeyCode::ArrowLeft,
            move_east: KeyCode::ArrowRight,
            set_bomb: KeyCode::Space,
        }
    }
}

#[derive(Default)]
pub struct InputState {
    horizontal_direction: i8,
    vertical_direction: i8,
}

#[derive(Copy, Clone)]
pub enum PlayerColor {
    White,
    Black,
}

impl PlayerColor {
    pub fn to_bevy_color(&self) -> Color {
        match self {
            PlayerColor::White => Color::srgb(0.85, 0.85, 0.85),
            PlayerColor::Black => Color::srgb(0.35, 0.35, 0.35),
        }
    }
}

#[derive(Component)]
pub struct Player {
    controls: ControlKeys,

    id: Uuid,
    color: PlayerColor,
    inputs: InputState,

    curr_speed: f32,

    bomb_capacity: u8,
    fire_range: u8,
    bomb_detonation_period: f32,
}

impl std::default::Default for Player {
    fn default() -> Self {
        Player {
            id: Uuid::new_v4(),
            color: PlayerColor::White,
            controls: ControlKeys::default(),
            inputs: InputState::default(),
            bomb_capacity: DEFAULT_BOMB_CAPACITY,
            fire_range: DEFAULT_FIRE_RANGE,
            curr_speed: DEFAULT_SPEED,
            bomb_detonation_period: bomb::DEFAULT_DETONATION_PERIOD,
        }
    }
}

fn spawn_players(mut commands: Commands) {
    let collision_groups = CollisionGroups::new(
        Group::from_bits(setup::collision::policy::PLAYER.0).unwrap(),
        Group::from_bits(setup::collision::policy::PLAYER.1).unwrap(),
    );

    commands.spawn((
        Player::default(),
        Sprite {
            color: PlayerColor::White.to_bevy_color(),
            custom_size: Some(SIZE),
            ..default()
        },
        map::Cell(0, 0).center(),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
        collision_groups,
        ColliderMassProperties::Mass(MASS),
        Friction::new(FRICTION),
        Restitution::new(RESTITUTION),
        ExternalForce::default(),
    ));

    commands.spawn((
        Player {
            color: PlayerColor::Black,
            controls: ControlKeys {
                move_north: KeyCode::KeyW,
                move_south: KeyCode::KeyS,
                move_west: KeyCode::KeyA,
                move_east: KeyCode::KeyD,
                set_bomb: KeyCode::KeyV,
            },
            ..default()
        },
        Sprite {
            color: PlayerColor::Black.to_bevy_color(),
            custom_size: Some(SIZE),
            ..default()
        },
        map::Cell(2, 2).center(),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(SIZE.x / 2.0, SIZE.y / 2.0),
        collision_groups,
        ColliderMassProperties::Mass(MASS),
        Friction::new(FRICTION),
        Restitution::new(RESTITUTION),
        ExternalForce::default(),
    ));
}

fn update_player_input(
    kbd_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &Transform)>,
    mut events: EventWriter<bomb::BombPlanted>,
    planted_bombs: Res<bomb::PlantedBombs>,
) {
    for (mut player, transform) in &mut query {
        let move_west = kbd_input.pressed(player.controls.move_west);
        let move_east = kbd_input.pressed(player.controls.move_east);
        let move_north = kbd_input.pressed(player.controls.move_north);
        let move_south = kbd_input.pressed(player.controls.move_south);

        player.inputs.horizontal_direction = move_east as i8 - move_west as i8;
        player.inputs.vertical_direction = move_north as i8 - move_south as i8;

        let player_cell = map::Cell::from_transform(transform);

        if kbd_input.just_pressed(player.controls.set_bomb)
            && player.bomb_capacity > 0
            && !planted_bombs.set.contains(&player_cell)
        {
            events.send(bomb::BombPlanted {
                player_id: player.id,
                player_color: player.color,
                player_cell: player_cell,
                player_fire_range: player.fire_range,
                player_bomb_detonation_period: player.bomb_detonation_period,
            });

            player.bomb_capacity -= 1;
        }
    }
}

fn movement_system(
    timestep_mode: Res<TimestepMode>,
    mut query: Query<(&Player, &Velocity, &mut ExternalForce)>,
) {
    for (player, velocity, mut ext_force) in &mut query {
        let desired_vel = Vec2::new(
            player.inputs.horizontal_direction as f32 * player.curr_speed,
            player.inputs.vertical_direction as f32 * player.curr_speed,
        );

        let current_vel = velocity.linvel;
        let vel_delta = desired_vel - current_vel;

        let mut time_delta = 0.0;
        match timestep_mode.as_ref() {
            TimestepMode::Fixed { dt, substeps: _ } => time_delta = dt.clone(),
            _ => (),
        }

        let desired_force = Vec2::new(
            // [Н] = [кг] * [м/с] / [с]
            MASS * vel_delta.x / time_delta,
            MASS * vel_delta.y / time_delta,
        );

        ext_force.force = desired_force;
    }
}

fn handle_bomb_exploded(
    mut events: EventReader<bomb::BombExploded>,
    mut query: Query<&mut Player>,
) {
    for event in events.read() {
        for mut player in &mut query {
            if player.id == event.player_id {
                player.bomb_capacity += 1;
                break;
            }
        }
    }
}

fn track_explosion_players(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    explosions: Query<(), With<explosion::Explosion>>,
    players: Query<((), &Player)>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                let player_entity;

                if explosions.get(*e1).is_ok() && players.get(*e2).is_ok() {
                    player_entity = e2;
                } else if explosions.get(*e2).is_ok() && players.get(*e1).is_ok() {
                    player_entity = e1;
                } else {
                    return;
                }
                println!("Player collision event");
                commands.entity(*player_entity).despawn();
            }
            _ => {}
        }
    }
}
