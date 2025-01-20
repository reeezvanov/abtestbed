use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BrickPlugin;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Event)]
struct BrickDestroyed;