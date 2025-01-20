use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::map;

const HOR_SIZE: Vec2 = Vec2::new(608.0, 4.0);
const VER_SIZE: Vec2 = Vec2::new(4.0, 392.0);
const MAIN_COLOR: Color = Color::srgb(0.2, 0.8, 0.2);
const FRICTION: f32 = 0.0;

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_borders);
    }
}

fn spawn_borders(mut commands: Commands) {
    // Spawn top border
    commands.spawn((
        Sprite {
            color: MAIN_COLOR,
            custom_size: Some(HOR_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(0.0, (map::SIZE.y / 2.0) + (HOR_SIZE.y / 2.0), 0.0),
        RigidBody::Fixed,
        Collider::cuboid(HOR_SIZE.x / 2.0, HOR_SIZE.y / 2.0),
        Friction::new(FRICTION),
    ));

    // Spawn bottom border
    commands.spawn((
        Sprite {
            color: MAIN_COLOR,
            custom_size: Some(HOR_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(0.0, -(map::SIZE.y / 2.0) - (HOR_SIZE.y / 2.0), 0.0),
        RigidBody::Fixed,
        Collider::cuboid(HOR_SIZE.x / 2.0, HOR_SIZE.y / 2.0),
        Friction::new(FRICTION),
    ));

    // Spawn left border
    commands.spawn((
        Sprite {
            color: MAIN_COLOR,
            custom_size: Some(VER_SIZE),
            ..Default::default()
        },
        Transform::from_xyz(-(map::SIZE.x / 2.0) - (VER_SIZE.x / 2.0), 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(VER_SIZE.x / 2.0, VER_SIZE.y / 2.0),
        Friction::new(FRICTION),
    ));

    // Spawn right border
    commands.spawn((
        Sprite {
            color: MAIN_COLOR,
            custom_size: Some(VER_SIZE),
            ..Default::default()
        },
        Transform::from_xyz((map::SIZE.x / 2.0) + (VER_SIZE.x / 2.0), 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(VER_SIZE.x / 2.0, VER_SIZE.y / 2.0),
        Friction::new(FRICTION),
    ));
}