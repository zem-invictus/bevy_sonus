use bevy::prelude::*;
use bevy_sonus::{AcousticMaterialPreset, SonusAudioPlugin, SonusListener};
use bevy_sonus::ecs::SonusEmitterFromConfigTemplate;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SonusAudioPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    let obstacle_transform = Transform::from_xyz(0.0, 1.5, 2.5);

    // Spatial audio listener on player camera
    commands.spawn_scene(bsn! {
        Camera3d
        SonusListener
        Transform::from_xyz(0.0, 1.5, 5.0).looking_at(obstacle_transform.translation, obstacle_transform.up())
    });

    // Spatial sound emitter with occlusion and distance attenuation
    commands.spawn_scene(bsn! {
        ~SonusEmitterFromConfigTemplate {
            source: "air_raid.wav",
            occlusion: true,
            min_dist: 2.0,
            max_dist: 20.0,
        }
        Transform::from_xyz(0.0, 1.0, 0.0)
    });

    // Obstacle wall with Concrete preset
    commands.spawn_scene(bsn! {
        Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 0.5)))
        MeshMaterial3d<StandardMaterial>(asset_value(StandardMaterial::from_color(Color::BLACK)))
        AcousticMaterialPreset::Concrete
        obstacle_transform
    });
}