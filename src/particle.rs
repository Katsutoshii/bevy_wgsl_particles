#![allow(dead_code)]

use bevy::{
    app::Plugin,
    asset::{load_internal_asset, weak_handle, DirectAssetAccessExt, Handle},
    ecs::{
        resource::Resource,
        world::{FromWorld, World},
    },
    math::Vec3,
    render::{
        extract_resource::ExtractResource,
        render_resource::{Shader, ShaderType},
        storage::ShaderStorageBuffer,
    },
};

pub const PARTICLE_SHADER_HANDLE: Handle<Shader> =
    weak_handle!("289b61ff-cdfe-449f-bd0d-d72d1ca9615c");

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<ParticleBuffer>();
        load_internal_asset!(
            app,
            PARTICLE_SHADER_HANDLE,
            "particle.wgsl",
            Shader::from_wgsl
        );
    }
}
#[derive(Default, ShaderType, Copy, Clone, Debug)]
pub struct Particle {
    pub position: Vec3,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct ParticleBuffer(pub Handle<ShaderStorageBuffer>);

impl FromWorld for ParticleBuffer {
    fn from_world(world: &mut World) -> Self {
        let particles = [Particle::default(); 16];
        Self(world.add_asset(ShaderStorageBuffer::from(particles)))
    }
}
