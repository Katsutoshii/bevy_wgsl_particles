use std::fmt::Debug;
use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin},
    asset::{Asset, AssetPath, Handle},
    ecs::{
        component::Component,
        lifecycle::HookContext,
        name::Name,
        resource::Resource,
        world::{DeferredWorld, FromWorld, World},
    },
    material::AlphaMode,
    math::UVec3,
    mesh::Mesh3d,
    pbr::{MaterialPlugin, MeshMaterial3d},
    reflect::{Reflect, TypePath},
    render::{
        extract_resource::ExtractResource, render_resource::AsBindGroup, storage::ShaderBuffer,
        RenderApp,
    },
    shader::ShaderRef,
};

mod assets;
mod compute;
mod material;
mod mesh;
mod particle;
mod spawner;

use crate::spawner::ParticleSpawnerPlugin;
pub use crate::{
    assets::ParticleAssets,
    compute::{ComputeShader, ComputeShaderPlugin},
    material::ParticleMaterial,
    mesh::MeshBuilder,
    particle::Particle,
    spawner::ParticleSpawner,
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
        app.add_plugins(MaterialPlugin::<material::ParticleMaterial<E>>::default())
            .init_resource::<ParticleAssets<E>>()
            .add_plugins(ComputeShaderPlugin::<ParticleCompute<E>>::default())
            .add_plugins(ParticleSpawnerPlugin::<E>::default());
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
