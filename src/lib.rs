use bevy::app::{App, Plugin};

mod compute;
mod mesh;
mod particle;

pub use crate::{
    compute::{ComputeShader, ComputeShaderPlugin},
    mesh::MeshBuilder,
    particle::{Particle, ParticleBuffer},
};

pub struct WgslParticlePlugin;
impl Plugin for WgslParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(particle::ParticlePlugin);
    }
}
