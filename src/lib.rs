use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin},
    asset::{Asset, AssetPath, DirectAssetAccessExt, Handle},
    color::LinearRgba,
    ecs::{
        component::Component,
        lifecycle::HookContext,
        name::Name,
        resource::Resource,
        world::{DeferredWorld, FromWorld, World},
    },
    image::Image,
    material::{
        descriptor::RenderPipelineDescriptor, specialize::SpecializedMeshPipelineError, AlphaMode,
    },
    math::{UVec2, UVec3},
    mesh::{Mesh, Mesh3d, MeshVertexBufferLayoutRef},
    pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin, MeshMaterial3d},
    reflect::{Reflect, TypePath},
    render::{
        extract_resource::ExtractResource,
        render_resource::{AsBindGroup, ShaderType},
        storage::ShaderBuffer,
        RenderApp,
    },
    shader::ShaderRef,
};

mod compute;
mod mesh;
mod particle;

pub use crate::{
    compute::{ComputeShader, ComputeShaderPlugin},
    mesh::MeshBuilder,
    particle::Particle,
};

pub struct WgslParticlesPlugin;
impl Plugin for WgslParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(particle::ParticlePlugin);
    }
}

pub struct WgslParticleEffectPlugin<E: Effect> {
    _marker: PhantomData<E>,
}
impl<E: Effect> Plugin for WgslParticleEffectPlugin<E> {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ParticleMaterial<E>>::default())
            .init_resource::<ParticleAssets<E>>()
            .add_plugins(ComputeShaderPlugin::<ParticleCompute<E>>::default());
    }
}
impl<E: Effect> Default for WgslParticleEffectPlugin<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

/// Effect to be overriden by the client.
pub trait Effect: Clone + TypePath + Sync + Send + Debug {
    /// Fragment and vertex shader.
    fn material_shader() -> ShaderRef;
    /// Compute shader.
    fn compute_shader() -> ShaderRef;
    /// Number of particles per instance.
    fn particles_per_instance() -> usize;
    /// Max number of effect instances.
    fn max_instances() -> usize;
    /// Max number of particles to process.
    fn max_particles() -> usize {
        Self::particles_per_instance() * Self::max_instances()
    }
    /// Texture path.
    fn texture_path() -> AssetPath<'static>;
    /// Alpha mode for the material, defaults to Blend.
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Name::new("ParticleEffect"))]
#[component(on_add=Self::on_add)]
pub struct ParticleEffect<E: Effect> {
    _marker: PhantomData<E>,
}
impl<E: Effect> Default for ParticleEffect<E> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl<E: Effect> ParticleEffect<E> {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let assets = world.resource::<ParticleAssets<E>>().clone();
        world
            .commands()
            .entity(context.entity)
            .insert((MeshMaterial3d(assets.material), Mesh3d(assets.mesh)));
    }
}

/// Particle material uniforms.
#[derive(ShaderType, Default, Debug, Clone)]
struct ParticleUniforms {
    color: LinearRgba,
    vertices_per_particle: u32,
}

/// Particle material.
#[derive(Asset, TypePath, AsBindGroup, Default, Debug, Clone, Resource, ExtractResource)]
#[extract_app(RenderApp)]
struct ParticleMaterial<E: Effect> {
    #[uniform(0)]
    uniform: ParticleUniforms,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,

    #[storage(3, read_only)]
    particles: Handle<ShaderBuffer>,
    alpha_mode: AlphaMode,
    _marker: PhantomData<E>,
}

#[derive(Resource, Reflect, Clone)]
pub struct ParticleAssets<E: Effect> {
    buffer: Handle<ShaderBuffer>,
    material: Handle<ParticleMaterial<E>>,
    mesh: Handle<Mesh>,
}
impl<E: Effect> FromWorld for ParticleAssets<E> {
    fn from_world(world: &mut World) -> Self {
        let particles: Vec<Particle> =
            vec![Particle::default(); E::particles_per_instance() * E::max_instances() as usize];
        let buffer = world.add_asset(ShaderBuffer::from(particles));
        let size = UVec2::new(16, 16);
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

/// Compute shader inputs.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Resource, ExtractResource)]
#[extract_app(RenderApp)]
struct ParticleCompute<E: Effect> {
    #[storage(0, visibility(compute))]
    particles: Handle<ShaderBuffer>,
    #[uniform(1)]
    dt: f32,
    _marker: PhantomData<E>,
}
impl<E: Effect> ParticleCompute<E> {
    const SHADER_ASSET_PATH: &str = "shaders/compute.wesl";
}
impl<E: Effect> FromWorld for ParticleCompute<E> {
    fn from_world(world: &mut World) -> Self {
        Self {
            dt: 0.005,
            particles: world.resource::<ParticleAssets<E>>().buffer.clone(),
            _marker: PhantomData,
        }
    }
}
impl<E: Effect> ComputeShader for ParticleCompute<E> {
    fn compute_shader() -> ShaderRef {
        Self::SHADER_ASSET_PATH.into()
    }
    fn workgroup_size() -> UVec3 {
        UVec3::new(64, 1, 1)
    }
    fn workgroup_count(&self) -> UVec3 {
        UVec3::new(E::max_particles() as u32 / Self::workgroup_size().x, 1, 1)
    }
}
