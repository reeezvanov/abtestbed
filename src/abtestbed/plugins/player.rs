use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use uuid::Uuid;

use super::super::common::CollisionMap;
use super::bomb::{BombExploded, BombPlanted};
use super::map::Cell;

const PLAYER_SIZE: Vec2 = Vec2::new(28.0, 28.0);
const PLAYER_MASS: f32 = 100.0;
const PLAYER_FRICTION: f32 = 0.0;
const PLAYER_RESTITUTION: f32 = 0.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players).add_systems(
            Update,
            (update_player_input, movement_system, handle_bomb_exploded).chain(),
        );
    }
}

#[derive(Copy, Clone)]
pub enum PlayerColor {
    WHITE,
    BLACK,
    // RED,
    // GREEN,
    // BLUE,
    // ORANGE,
    // PINK,
    // GRAY,
    // YELLOW,
    // PURPLE,
}

impl PlayerColor {
    pub fn to_bevy_color(&self) -> Color {
        match self {
            PlayerColor::WHITE => Color::srgb(0.85, 0.85, 0.85),
            PlayerColor::BLACK => Color::srgb(0.35, 0.35, 0.35),
            // PlayerColor::RED => Color::srgb(0.9, 0.1, 0.1),
            // PlayerColor::GREEN => Color::srgb(0.1, 0.9, 0.1),
            // PlayerColor::BLUE => Color::srgb(0.1, 0.1, 0.9),
            // PlayerColor::ORANGE => todo!(),
            // PlayerColor::PINK => todo!(),
            // PlayerColor::GRAY => todo!(),
            // PlayerColor::YELLOW => todo!(),
            // PlayerColor::PURPLE => todo!(),
        }
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
            color: PlayerColor::WHITE,
            controls: ControlKeys::default(),
            inputs: InputState::default(),
            bomb_capacity: 1,
            fire_range: 2,
            curr_speed: 100.0,
            bomb_detonation_period: 2.0,
        }
    }
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Player::default(),
        Sprite {
            color: PlayerColor::WHITE.to_bevy_color(),
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Cell(0, 0).center(),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0),
        CollisionGroups::new(
            Group::from_bits(CollisionMap::PLAYER.0).unwrap(),
            Group::from_bits(CollisionMap::PLAYER.1).unwrap(),
        ),
        ColliderMassProperties::Mass(PLAYER_MASS),
        Friction::new(PLAYER_FRICTION),
        Restitution::new(PLAYER_RESTITUTION),
        ExternalForce::default(),
    ));

    commands.spawn((
        Player {
            color: PlayerColor::BLACK,
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
            color: PlayerColor::BLACK.to_bevy_color(),
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Cell(5, 2).center(),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0),
        CollisionGroups::new(
            Group::from_bits(CollisionMap::PLAYER.0).unwrap(),
            Group::from_bits(CollisionMap::PLAYER.1).unwrap(),
        ),
        ColliderMassProperties::Mass(PLAYER_MASS),
        Friction::new(PLAYER_FRICTION),
        Restitution::new(PLAYER_RESTITUTION),
        ExternalForce::default(),
    ));
}

fn update_player_input(
    kbd_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &Transform)>,
    mut events: EventWriter<BombPlanted>,
) {
    for (mut player, transform) in &mut query {
        let move_west = kbd_input.pressed(player.controls.move_west);
        let move_east = kbd_input.pressed(player.controls.move_east);
        let move_north = kbd_input.pressed(player.controls.move_north);
        let move_south = kbd_input.pressed(player.controls.move_south);

        player.inputs.horizontal_direction = move_east as i8 - move_west as i8;
        player.inputs.vertical_direction = move_north as i8 - move_south as i8;

        if kbd_input.just_pressed(player.controls.set_bomb) && player.bomb_capacity > 0 {
            events.send(BombPlanted {
                player_id: player.id,
                player_color: player.color,
                player_cell: Cell::from_transform(transform),
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
            PLAYER_MASS * vel_delta.x / time_delta,
            PLAYER_MASS * vel_delta.y / time_delta,
        );

        ext_force.force = desired_force;
    }
}

fn handle_bomb_exploded(mut events: EventReader<BombExploded>, mut query: Query<&mut Player>) {
    for event in events.read() {
        for mut player in &mut query {
            if player.id == event.player_id {
                player.bomb_capacity += 1;
                break;
            }
        }
    }
}
