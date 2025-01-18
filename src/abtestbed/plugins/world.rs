use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const FPS: f32 = 40.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup_world)
            .insert_resource(TimestepMode::Fixed {
                dt: 1.0 / FPS,
                substeps: 1,
            });
    }
}

fn setup_world(mut commands: Commands, mut rapier_config: Query<&mut RapierConfiguration>) {
    commands.spawn(Camera2d::default());

    let mut rapier_config = rapier_config.single_mut();
    rapier_config.gravity = Vec2::ZERO;
}
