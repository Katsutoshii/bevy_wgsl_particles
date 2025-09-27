//! Example to demonstrate reading texture data back to CPU from a compute shader.
//! Press Space to cycle through different inputs to the shader to demonstrate reactivity.

use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, prelude::*};
use bevy_wgsl_particles::{MeshBuilder, WgslParticlePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WgslParticlePlugin))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

/// Visualize the compute shader output as a sprite.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    let size = UVec2::new(4, 4);
    commands.spawn((
        Mesh3d(meshes.add(MeshBuilder::grid(size).build())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::Srgba(WHITE),
            base_color_texture: Some(asset_server.load("textures/bubble_transparent.png")),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        Transform {
            scale: Vec3::splat(5.0),
            ..default()
        },
    ));
}
