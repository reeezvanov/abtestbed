use bevy::prelude::*;

mod common;
mod plugins;

pub fn main() {
    App::new()
        .add_plugins(plugins::world::WorldPlugin)
        .add_plugins(plugins::map::MapPlugin)
        .add_plugins(plugins::player::PlayerPlugin)
        .add_plugins(plugins::bomb::BombPlugin)
        .run();
}
