use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::{
    asset::{Asset, Handle},
    color::LinearRgba,
    ecs::resource::Resource,
    image::Image,
    material::{
        descriptor::RenderPipelineDescriptor, specialize::SpecializedMeshPipelineError, AlphaMode,
    },
    mesh::{Mesh, MeshVertexBufferLayoutRef},
    pbr::{Material, MaterialPipeline, MaterialPipelineKey},
    reflect::TypePath,
    render::{
        extract_resource::ExtractResource,
        render_resource::{AsBindGroup, ShaderType},
        storage::ShaderBuffer,
        RenderApp,
    },
    shader::ShaderRef,
};

use crate::Effect;

/// Particle material uniforms.
#[derive(ShaderType, Default, Debug, Clone)]
pub struct ParticleUniforms {
    pub color: LinearRgba,
    pub vertices_per_particle: u32,
}

/// Particle material.
#[derive(Asset, TypePath, AsBindGroup, Default, Debug, Clone, Resource, ExtractResource)]
#[extract_app(RenderApp)]
pub struct ParticleMaterial<E: Effect> {
    #[uniform(0)]
    pub uniform: ParticleUniforms,

    #[texture(1)]
    #[sampler(2)]
    pub color_texture: Handle<Image>,

    #[storage(3, read_only)]
    pub particles: Handle<ShaderBuffer>,
    pub alpha_mode: AlphaMode,
    pub _marker: PhantomData<E>,
}
impl<E: Effect> Material for ParticleMaterial<E> {
    fn vertex_shader() -> ShaderRef {
        E::material_shader()
    }
    fn fragment_shader() -> ShaderRef {
        E::material_shader()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
