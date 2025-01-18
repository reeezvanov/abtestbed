use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const MAP_SIZE: Vec2 = Vec2::new(600.0, 392.0);

const NET_SIZE: (u8, u8) = (15, 11);
const CELL_SIZE: Vec2 = Vec2::new(40.0, 36.0);
const CELL_BALL_RADIUS: f32 = 18.0;

const MAP_FRICTION: f32 = 0.0;

const BORDER_HOR_SIZE: Vec2 = Vec2::new(608.0, 4.0);
const BORDER_VER_SIZE: Vec2 = Vec2::new(4.0, 392.0);

const BORDER_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
const BLOCK_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_borders)
            .add_systems(Startup, spawn_borders)
            .add_systems(Startup, spawn_scheme);
    }
}

fn spawn_borders(mut commands: Commands) {
    // Spawn top border
    commands.spawn((
        Sprite {
            color: BORDER_COLOR,
            custom_size: Some(BORDER_HOR_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(
            0.0,
            (MAP_SIZE.y / 2.0) + (BORDER_HOR_SIZE.y / 2.0),
            0.0,
        ),
        RigidBody::Fixed,
        Collider::cuboid(BORDER_HOR_SIZE.x / 2.0, BORDER_HOR_SIZE.y / 2.0),
        Friction::new(MAP_FRICTION),
    ));

    // Spawn bottom border
    commands.spawn((
        Sprite {
            color: BORDER_COLOR,
            custom_size: Some(BORDER_HOR_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(
            0.0,
            -(MAP_SIZE.y / 2.0) - (BORDER_HOR_SIZE.y / 2.0),
            0.0,
        ),
        RigidBody::Fixed,
        Collider::cuboid(BORDER_HOR_SIZE.x / 2.0, BORDER_HOR_SIZE.y / 2.0),
        Friction::new(MAP_FRICTION),
    ));

    // Spawn left border
    commands.spawn((
        Sprite {
            color: BORDER_COLOR,
            custom_size: Some(BORDER_VER_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(
            -(MAP_SIZE.x / 2.0) - (BORDER_VER_SIZE.x / 2.0),
            0.0,
            0.0,
        ),
        RigidBody::Fixed,
        Collider::cuboid(BORDER_VER_SIZE.x / 2.0, BORDER_VER_SIZE.y / 2.0),
        Friction::new(MAP_FRICTION),
    ));

    // Spawn right border
    commands.spawn((
        Sprite {
            color: BORDER_COLOR,
            custom_size: Some(BORDER_VER_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(
            (MAP_SIZE.x / 2.0) + (BORDER_VER_SIZE.x / 2.0),
            0.0,
            0.0,
        ),
        RigidBody::Fixed,
        Collider::cuboid(BORDER_VER_SIZE.x / 2.0, BORDER_VER_SIZE.y / 2.0),
        Friction::new(MAP_FRICTION),
    ));
}

fn spawn_scheme(mut commands: Commands) {
    let cell_start_pos = Vec2::new(
        -(MAP_SIZE.x / 2.0) + (CELL_SIZE.x / 2.0),
        (MAP_SIZE.y / 2.0) - (CELL_SIZE.y / 2.0),
    );

    for j in 0..NET_SIZE.1 as u8 {
        if j % 2 == 0 {
            continue;
        }

        for i in 0..NET_SIZE.0 {
            if i % 2 == 0 {
                continue;
            }

            commands.spawn((
                Sprite {
                    color: BLOCK_COLOR,
                    custom_size: Some(CELL_SIZE),
                    ..Default::default()
                },
                Transform::from_xyz(
                    cell_start_pos.x + (i as f32 * CELL_SIZE.x),
                    cell_start_pos.y - (j as f32 * CELL_SIZE.y),
                    0.0,
                ),
                RigidBody::Fixed,
                Collider::ball(CELL_BALL_RADIUS),
                Friction::new(MAP_FRICTION),
            ));
        }
    }
}
