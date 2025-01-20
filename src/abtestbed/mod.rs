use bevy::prelude::*;

mod setup;
mod world;

pub fn main() {
    App::new()
        .add_plugins(setup::SetupPlugin)
        .add_plugins(world::WorldPlugin)
        .run();
}
