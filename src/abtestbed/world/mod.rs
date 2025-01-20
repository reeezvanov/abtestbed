use bevy::prelude::*;

mod map;
mod border;
mod block;
mod brick;
mod player;
mod bomb;
mod explosion;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(border::BorderPlugin)
            .add_plugins(block::BlockPlugin)
            .add_plugins(brick::BrickPlugin)
            .add_plugins(explosion::ExplosionPlugin)
            .add_plugins(bomb::BombPlugin)
            .add_plugins(player::PlayerPlugin);
    }
}
