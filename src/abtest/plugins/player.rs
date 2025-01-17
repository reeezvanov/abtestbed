use bevy::{log::tracing_subscriber::fmt::time, prelude::*};
use bevy_rapier2d::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(28.0, 28.0);
const PLAYER_SPEED: f32 = 150.0;
const PLAYER_MASS: f32 = 100.0;
const PLAYER_FRICTION: f32 = 0.13;
const PLAYER_RESTITUTION: f32 = 0.0;

struct ControlKeys {
    move_north: KeyCode,
    move_south: KeyCode,
    move_west: KeyCode,
    move_east: KeyCode,
}

struct InputState(i8, i8);

#[derive(Component)]
pub struct Player {
    controls: ControlKeys,
    inputs: InputState,
}
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(Update, (update_player_input, movement_system).chain());
    }
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Player {
            controls: ControlKeys {
                move_north: KeyCode::ArrowUp,
                move_south: KeyCode::ArrowDown,
                move_west: KeyCode::ArrowLeft,
                move_east: KeyCode::ArrowRight,
            },
            inputs: InputState(0, 0),
        },
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(PLAYER_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(40.0, 0.0, 0.0),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0),
        ColliderMassProperties::Mass(PLAYER_MASS),
        Friction::new(PLAYER_FRICTION),
        Restitution::new(PLAYER_RESTITUTION),
        ExternalForce::default(),
    ));

    commands.spawn((
        Player {
            controls: ControlKeys {
                move_north: KeyCode::KeyW,
                move_south: KeyCode::KeyS,
                move_west: KeyCode::KeyA,
                move_east: KeyCode::KeyD,
            },
            inputs: InputState(0, 0),
        },
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(PLAYER_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(40.0, 80.0, 0.0),
        RigidBody::Dynamic,
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED_Z,
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0),
        ColliderMassProperties::Mass(PLAYER_MASS),
        Friction::new(PLAYER_FRICTION),
        Restitution::new(PLAYER_RESTITUTION),
        ExternalForce::default(),
    ));
}

fn update_player_input(kbd_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Player>) {
    for mut player in &mut query {
        let west = kbd_input.pressed(player.controls.move_west);
        let east = kbd_input.pressed(player.controls.move_east);
        let north = kbd_input.pressed(player.controls.move_north);
        let south = kbd_input.pressed(player.controls.move_south);

        player.inputs.0 = if west {
            -1
        } else if east {
            1
        } else {
            0
        };

        player.inputs.1 = if north {
            1
        } else if south {
            -1
        } else {
            0
        };
    }
}

fn movement_system(
    timestep_mode: Res<TimestepMode>,
    mut query: Query<(&Player, &Velocity, &mut ExternalForce)>,
) {
    for (player, velocity, mut ext_force) in &mut query {
        let desired_vel = Vec2::new(
            player.inputs.0 as f32 * PLAYER_SPEED,
            player.inputs.1 as f32 * PLAYER_SPEED,
        );

        let current_vel = velocity.linvel;
        let vel_delta = desired_vel - current_vel;

        let mut time_delta = 0.0;
        match timestep_mode.as_ref() {
            TimestepMode::Fixed { dt, substeps } => time_delta = *dt,
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
