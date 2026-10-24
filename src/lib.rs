use std::{hash::Hash, marker::PhantomData};

use bevy::{
    app::{App, Plugin},
    pbr::{Material, MaterialPlugin},
};

mod compute;
mod mesh;
mod particle;

pub use crate::{
    compute::{ComputeShader, ComputeShaderPlugin},
    mesh::MeshBuilder,
    particle::{Particle, ParticleBuffer},
};

pub struct WgslParticlesPlugin;
impl Plugin for WgslParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(particle::ParticlePlugin);
    }
}

pub struct WgslParticlePlugin<M: Material, C: ComputeShader> {
    _m: PhantomData<M>,
    _c: PhantomData<C>,
}
impl<M: Material, C: ComputeShader> Plugin for WgslParticlePlugin<M, C>
where
    M::Data: PartialEq + Eq + Clone + Hash,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<M>::default())
            .add_plugins(ComputeShaderPlugin::<C>::default());
    }
}
impl<M: Material, C: ComputeShader> Default for WgslParticlePlugin<M, C> {
    fn default() -> Self {
        Self {
            _m: PhantomData::default(),
            _c: PhantomData::default(),
        }
    }
}
