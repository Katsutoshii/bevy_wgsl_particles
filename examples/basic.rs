//! Basic exmple of a particle system.
//! `cargo run --example basic`
#![allow(dead_code)]
use std::f32::consts::PI;

use bevy::{
    asset::AssetPath,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
    shader::ShaderRef,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_wgsl_particles::{Effect, ParticleEffect, WgslParticleEffectPlugin, WgslParticlesPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MeshPickingPlugin,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            WgslParticlesPlugin,
            WgslParticleEffectPlugin::<BubbleEffect>::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<BackgroundPlaneAssets>()
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
    fn texture_path() -> AssetPath<'static> {
        AssetPath::from("textures/bubble_transparent.png")
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
            ParticleEffect::<BubbleEffect>::default(),
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
