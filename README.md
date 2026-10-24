# Bevy WGSL Particles

[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](./LICENSE-MIT)
[![Doc](https://docs.rs/bevy_wgsl_particles/badge.svg)](https://docs.rs/bevy_wgsl_particles)
[![Crate](https://img.shields.io/crates/v/bevy_wgsl_particles.svg)](https://crates.io/crates/bevy_wgsl_particles)
[![Build Status](https://github.com/katsutoshii/bevy_wgsl_particles/actions/workflows/ci.yaml/badge.svg)](https://github.com/katsutoshii/bevy_wgsl_particles/actions/workflows/ci.yaml)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-v0.20-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

## Usage

Create a material for your particle and spawn it.

```rs
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WgslParticlesPlugin,
            WgslParticlePlugin<ParticleMaterial, ParticleCompute>::default()
        ))
        .add_systems(Startup, setup)
        .run();
}

fn update(commands: Commands) {
    commands.spawn(
        Mesh3d(meshes.add(MeshBuilder::grid(size).build())),
        MeshMaterial3d(materials.add(ParticleMaterial {
            uniform: ParticleUniforms {
                color: LinearRgba::new(1.0, 1.0, 1.0, 0.3).into(),
                vertices_per_particle: 4,
            },
            color_texture: asset_server.load("textures/bubble_transparent.png"),
            particles: particles.0.clone(),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }))
    );
}
```

Next, we 

```rs
```

## TODO

- [ ] Support multiple effects.
- [ ] Support spawning effects dynamically.

## Bevy support table

| bevy | bevy_spatialgrid2d |
| ---- | ------------------ |
| 0.20 | 0.20.0             |
| 0.19 | 0.19.0             |
| 0.18 | 0.18.0             |
