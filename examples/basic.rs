//! Basic exmple of a particle system.
//! `cargo run --example basic`
#![allow(dead_code)]
use std::f32::consts::PI;

use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    mesh::MeshVertexBufferLayoutRef,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    render::{
        extract_resource::ExtractResource,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError,
        },
        storage::ShaderBuffer,
        RenderApp,
    },
    shader::ShaderRef,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_wgsl_particles::{
    ComputeShader, MeshBuilder, ParticleBuffer, WgslParticlePlugin, WgslParticlesPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            WgslParticlesPlugin,
            WgslParticlePlugin::<ParticleMaterial, ParticleCompute>::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<BackgroundPlaneAssets>()
        .init_resource::<ParticleAssets>()
        .add_systems(Startup, setup)
        .run();
}

/// Visualize the compute shader output as a sprite.
fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));
    commands.spawn(DirectionalLight::default());
    // commands.spawn((ParticleEffect, Transform::default()));
    commands.spawn((
        BackgroundPlane,
        Transform {
            rotation: Quat::from_rotation_x(PI / 2.0),
            ..default()
        },
    ));
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(
    Name::new("MainCamera"),
    Camera3d::default(),
    Projection::Perspective(PerspectiveProjection {
        fov: PI / 2.0,
        near: 0.1,
        far: 2000.,
        ..default()
    }))]
#[component()]
struct MainCamera;
impl MainCamera {}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Name::new("ParticleEffect"))]
#[component(on_add=Self::on_add)]
struct ParticleEffect;
impl ParticleEffect {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let assets = world.resource::<ParticleAssets>().clone();
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
struct ParticleMaterial {
    #[uniform(0)]
    uniform: ParticleUniforms,

    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,

    #[storage(3, read_only)]
    particles: Handle<ShaderBuffer>,
    alpha_mode: AlphaMode,
}
impl ParticleMaterial {
    const SHADER_ASSET_PATH: &str = "shaders/basic.wesl";
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
struct ParticleCompute {
    #[storage(0, visibility(compute))]
    particles: Handle<ShaderBuffer>,
    #[uniform(1)]
    dt: f32,
}
impl ParticleCompute {
    const SHADER_ASSET_PATH: &str = "shaders/compute.wesl";
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
        UVec3::new(64, 1, 1)
    }
    fn workgroup_count(&self) -> UVec3 {
        UVec3::new(
            ParticleBuffer::MAX_PARTICLES / Self::workgroup_size().x,
            1,
            1,
        )
    }
}

#[derive(Resource, Reflect, Clone)]
struct ParticleAssets {
    material: Handle<ParticleMaterial>,
    mesh: Handle<Mesh>,
}
impl FromWorld for ParticleAssets {
    fn from_world(world: &mut World) -> Self {
        let particles = world.resource::<ParticleBuffer>().0.clone();
        let size = UVec2::new(16, 16);
        Self {
            material: world.add_asset(ParticleMaterial {
                uniform: ParticleUniforms {
                    color: LinearRgba::new(1.0, 1.0, 1.0, 0.3).into(),
                    vertices_per_particle: 4,
                },
                color_texture: world.load_asset("textures/bubble_transparent.png"),
                particles: particles,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
            mesh: world.add_asset(MeshBuilder::grid(size).build()),
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
#[require(Name::new("BackgroundPlane"))]
#[component(on_add=Self::on_add)]
struct BackgroundPlane;
impl BackgroundPlane {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        let assets = world.resource::<BackgroundPlaneAssets>().clone();
        world
            .commands()
            .entity(context.entity)
            .insert((MeshMaterial3d(assets.material), Mesh3d(assets.mesh)))
            .observe(Self::on_click);
    }
    fn on_click(click: On<PointerClick>, mut commands: Commands) {
        if click.button != PointerButton::Primary {
            return;
        }
        let Some(hit) = click.hit.position else {
            return;
        };
        commands.spawn((
            ParticleEffect,
            Transform {
                translation: hit + Vec3::Y * 0.5 / 2.0,
                scale: Vec3::splat(0.2),
                ..default()
            },
        ));
    }
}
#[derive(Resource, Reflect, Clone)]
struct BackgroundPlaneAssets {
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
}
impl FromWorld for BackgroundPlaneAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            material: world.add_asset(StandardMaterial {
                base_color: Color::linear_rgba(0.1, 0.1, 0.1, 0.3),
                unlit: true,
                ..default()
            }),
            mesh: world.add_asset(Plane3d::default().mesh().size(10.0, 10.0)),
        }
    }
}
