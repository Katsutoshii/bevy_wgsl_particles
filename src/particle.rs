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
        extract_resource::ExtractResource, render_resource::ShaderType, storage::ShaderBuffer,
        RenderApp,
    },
    shader::load_shader_library,
};

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "particle.wesl");
        app.init_resource::<ParticleBuffer>();
    }
}

/// Defines common attributes of a particle.
#[derive(Default, ShaderType, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub color: LinearRgba,
}

#[derive(Resource, ExtractResource, Clone)]
#[extract_app(RenderApp)]
pub struct ParticleBuffer(pub Handle<ShaderBuffer>);
impl ParticleBuffer {
    pub const MAX_PARTICLES: u32 = 16 * 16;
}

impl FromWorld for ParticleBuffer {
    fn from_world(world: &mut World) -> Self {
        Self(world.add_asset(ShaderBuffer::from(vec![
            Particle::default();
            Self::MAX_PARTICLES as usize
        ])))
    }
}
