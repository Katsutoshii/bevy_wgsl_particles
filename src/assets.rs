use std::marker::PhantomData;

use bevy::{
    asset::{DirectAssetAccessExt, Handle},
    color::LinearRgba,
    ecs::{
        resource::Resource,
        world::{FromWorld, World},
    },
    material::AlphaMode,
    math::UVec2,
    mesh::Mesh,
    reflect::Reflect,
    render::storage::ShaderBuffer,
};

use crate::{material::ParticleUniforms, Effect, MeshBuilder, Particle, ParticleMaterial};

#[derive(Resource, Reflect, Clone)]
pub struct ParticleAssets<E: Effect> {
    pub buffer: Handle<ShaderBuffer>,
    pub material: Handle<ParticleMaterial<E>>,
    pub mesh: Handle<Mesh>,
}
impl<E: Effect> FromWorld for ParticleAssets<E> {
    fn from_world(world: &mut World) -> Self {
        let particles: Vec<Particle> =
            vec![Particle::default(); E::particles_per_instance() * E::max_instances() as usize];
        let buffer = world.add_asset(ShaderBuffer::from(particles));
        let size = UVec2::new(
            E::particles_per_instance() as u32,
            E::max_instances() as u32,
        );
        Self {
            buffer: buffer.clone(),
            material: world.add_asset(ParticleMaterial {
                uniform: ParticleUniforms {
                    color: LinearRgba::new(1.0, 1.0, 1.0, 0.3).into(),
                    vertices_per_particle: 4,
                },
                color_texture: world.load_asset(E::texture_path()),
                particles: buffer,
                alpha_mode: AlphaMode::Blend,
                _marker: PhantomData,
            }),
            mesh: world.add_asset(MeshBuilder::grid(size).build()),
        }
    }
}
