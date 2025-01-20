use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const FPS: f32 = 40.0;

pub mod collision {

    pub mod group {
        pub const PLAYER_GROUP: u32 = 0b00000001;
        pub const BOMB_GROUP: u32 = 0b00000010;
    }

    pub mod policy {
        pub const PLAYER: (u32, u32) = (super::group::PLAYER_GROUP, super::group::BOMB_GROUP);
        pub const BOMB: (u32, u32) = (
            super::group::BOMB_GROUP,
            super::group::PLAYER_GROUP | super::group::BOMB_GROUP,
        );
    }
}

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_system)
            .insert_resource(TimestepMode::Fixed {
                dt: 1.0 / FPS,
                substeps: 1,
            });
    }
}

fn setup_system(mut commands: Commands, mut rapier_config: Query<&mut RapierConfiguration>) {
    commands.spawn(Camera2d::default());

    let mut rapier_config = rapier_config.single_mut();
    rapier_config.gravity = Vec2::ZERO;
}
