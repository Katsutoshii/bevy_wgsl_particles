# Bevy WGSL Particles

[![License: MIT or Apache 2.0](https://img.shields.io/badge/License-MIT%20or%20Apache2-blue.svg)](./LICENSE-MIT)
[![Doc](https://docs.rs/bevy_wgsl_particles/badge.svg)](https://docs.rs/bevy_wgsl_particles)
[![Crate](https://img.shields.io/crates/v/bevy_wgsl_particles.svg)](https://crates.io/crates/bevy_wgsl_particles)
[![Build Status](https://github.com/katsutoshii/bevy_wgsl_particles/actions/workflows/ci.yaml/badge.svg)](https://github.com/katsutoshii/bevy_wgsl_particles/actions/workflows/ci.yaml)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-v0.20-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

## Usage

Create a material for your particle and spawn it.

```rs
// Define your own effect via shaders.
#[derive(Clone, Debug, Reflect)]
struct BubbleEffect;
impl Effect for BubbleEffect {
    fn material_shader() -> ShaderRef {
        "shaders/basic.wesl".into()
    }
    fn compute_shader() -> ShaderRef {
        "shaders/compute.wesl".into()
    }
    fn max_instances() -> usize {
        16
    }
    fn particles_per_instance() -> usize {
        64
    }
}

// Add WgslParticlesPlugin and WgslParticleEffectPlugin
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WgslParticlesPlugin,
            WgslParticleEffectPlugin<BubbleEffect>::default()
        ))
        .add_systems(Startup, setup)
        .run();
}

/// Spawn particles wherever you like.
fn setup(commands: Commands) {
    commands.spawn(
        ParticleEffect<BubbleEffect>(),
        Transform::default(),
    );
}
```

## How does it work?

Each particle effect is one multi-mesh (collection of quads).
Each particle effect type is managed via one large `ShaderBuffer` that stores all of the state of each particle.
The index into the array is computed based on the vertex ID, which is global per draw call.
All particles of the same effect time happen in the same draw call, so the indexing scheme is sound.
When we spawn a new particle effect, we write all of the new particles to a new empty partition of the shared buffer.

## TODO

- [ ] Support multiple effects.
- [ ] Support spawning effects dynamically.

## Bevy support table

| bevy | bevy_spatialgrid2d |
| ---- | ------------------ |
| 0.20 | 0.20.0             |
| 0.19 | 0.19.0             |
| 0.18 | 0.18.0             |
