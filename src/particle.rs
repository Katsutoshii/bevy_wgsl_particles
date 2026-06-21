use bevy::{
    app::Plugin,
    asset::{DirectAssetAccessExt, Handle},
    color::LinearRgba,
    ecs::{
        resource::Resource,
        world::{FromWorld, World},
    },
    math::Vec3,
    render::{
        extract_resource::ExtractResource, render_resource::ShaderType,
        storage::ShaderStorageBuffer,
    },
    shader::load_shader_library,
};

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "particle.wgsl");
        app.init_resource::<ParticleBuffer>();
    }
}
#[derive(Default, ShaderType, Copy, Clone, Debug)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: LinearRgba,
}

#[derive(Resource, ExtractResource, Clone)]
pub struct ParticleBuffer(pub Handle<ShaderStorageBuffer>);
impl ParticleBuffer {
    pub const MAX_PARTICLES: u32 = 16 * 16;
}

impl FromWorld for ParticleBuffer {
    fn from_world(world: &mut World) -> Self {
        let particles = [Particle::default(); Self::MAX_PARTICLES as usize];
        Self(world.add_asset(ShaderStorageBuffer::from(particles)))
    }
}
