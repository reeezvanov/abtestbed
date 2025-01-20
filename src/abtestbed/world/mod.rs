use bevy::prelude::*;

mod map;
mod block;
mod bomb;
mod border;
mod brick;
mod explosion;
mod player;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(border::BorderPlugin)
            .add_plugins(block::BlockPlugin)
            .add_plugins(brick::BrickPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(bomb::BombPlugin)
            .add_plugins(explosion::ExplosionPlugin);
    }
}
