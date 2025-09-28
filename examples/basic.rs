//! Example to demonstrate reading texture data back to CPU from a compute shader.
//! Press Space to cycle through different inputs to the shader to demonstrate reactivity.
#![allow(dead_code)]
use std::f32::consts::PI;

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
        storage::ShaderStorageBuffer,
    },
};
use bevy_wgsl_particles::{
    ComputeShader, ComputeShaderPlugin, MeshBuilder, ParticleBuffer, WgslParticlePlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WgslParticlePlugin,
            MaterialPlugin::<ParticleMaterial>::default(),
            ComputeShaderPlugin::<ParticleCompute>::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

/// Visualize the compute shader output as a sprite.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticleMaterial>>,
    particles: Res<ParticleBuffer>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            fov: PI / 2.0,
            near: 0.1,
            far: 2000.,
            ..default()
        }),
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));
    commands.spawn(DirectionalLight::default());
    let size = UVec2::new(16, 16);
    commands.spawn((
        Mesh3d(meshes.add(MeshBuilder::grid(size).build())),
        MeshMaterial3d(materials.add(ParticleMaterial {
            color: LinearRgba::new(1.0, 1.0, 1.0, 0.3).into(),
            vertices_per_particle: 4,
            color_texture: asset_server.load("textures/bubble_transparent.png"),
            alpha_mode: AlphaMode::Blend,
            particles: particles.0.clone(),
            ..default()
        })),
        Transform {
            scale: Vec3::splat(5.0),
            ..default()
        },
    ));
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Default, Debug, Clone, Resource, ExtractResource)]
struct ParticleMaterial {
    #[uniform(0)]
    color: LinearRgba,

    #[uniform(1)]
    vertices_per_particle: u32,

    #[texture(2)]
    #[sampler(3)]
    color_texture: Handle<Image>,

    #[storage(4, read_only)]
    particles: Handle<ShaderStorageBuffer>,

    alpha_mode: AlphaMode,
}
impl ParticleMaterial {
    const SHADER_ASSET_PATH: &str = "shaders/basic.wgsl";
}
impl Material for ParticleMaterial {
    fn vertex_shader() -> ShaderRef {
        Self::SHADER_ASSET_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        Self::SHADER_ASSET_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
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

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Resource, ExtractResource)]
struct ParticleCompute {
    #[storage(0, visibility(compute))]
    particles: Handle<ShaderStorageBuffer>,
    #[uniform(1)]
    dt: f32,
}
impl ParticleCompute {
    const SHADER_ASSET_PATH: &str = "shaders/compute.wgsl";
}
impl FromWorld for ParticleCompute {
    fn from_world(world: &mut World) -> Self {
        Self {
            dt: 0.005,
            particles: world.resource::<ParticleBuffer>().0.clone(),
        }
    }
}
impl ComputeShader for ParticleCompute {
    fn compute_shader() -> ShaderRef {
        Self::SHADER_ASSET_PATH.into()
    }
    fn workgroup_size() -> UVec3 {
        UVec3::new(16, 1, 1)
    }
}
