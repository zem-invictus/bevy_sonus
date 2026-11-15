# bevy_sonus

[![Crates.io](https://img.shields.io/crates/v/bevy_sonus)](https://crates.io/crates/bevy_sonus)
[![Documentation](https://docs.rs/bevy_sonus/badge.svg)](https://docs.rs/bevy_sonus)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

Spatial audio plugin for [Bevy Engine](https://bevyengine.org).

<video src="https://github.com/user-attachments/assets/a381ec43-c59d-41bb-8e9f-bdd30557eab0" controls autoplay loop muted width="100%">
</video>

## Features

* **Multi-ray occlusion**: 5-ray cross pattern around emitter radius.
* **3-band crossover filtering**: Frequency-dependent low, mid, and high attenuation based on material transmission.
* **Material presets**: Transmission coefficients for Wood, Stone, Concrete, Glass, Metal, and ThinPlaster.
* **Sound diffraction**: Perceived direction bending around obstacle edges.
* **Distance attenuation**: Linear and inverse-distance models.
* **Blender & Skein workflow**: Spawn audio emitters and materials directly from GLTF scenes.
* **Lock-free DSP**: Atomic parameters shared between Bevy ECS and audio threads.

## Quickstart

Add `bevy_sonus` to `Cargo.toml`:

```toml
[dependencies]
bevy = "0.20"
bevy_sonus = "0.2"
```

### Spawning Emitter and Listener

```rust
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
```

## Debug Gizmos

Enable debug gizmos to view attenuation spheres, rays, and wall AABBs:

```rust
App::new()
    .add_plugins((
        DefaultPlugins,
        SonusAudioPlugin::default().with_debug(),
    ));
```

### Indicators
* **Green ray**: Clear line of sight.
* **Orange ray**: Partial obstruction.
* **Red ray**: Blocked ray.
* **Green sphere**: Minimum distance ($d_{\text{min}}$) with zero attenuation.
* **Yellow sphere**: Maximum distance ($d_{\text{max}}$) cutoff point.

## Blender & Skein Workflow

`bevy_sonus` integrates with [bevy_skein](https://crates.io/crates/bevy_skein) to deserialize components directly from GLTF custom properties:

1. Attach `SonusEmitterConfig` to empty nodes or mesh objects in Blender.
2. Attach `AcousticMaterialPreset` or custom `AcousticMaterial` to obstacle meshes.
3. Export the scene as `.gltf` or `.glb`.
4. Load the scene using `bevy_skein` to automatically instantiate sound emitters and acoustic obstacles at runtime.

## License

Licensed under either of:
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
