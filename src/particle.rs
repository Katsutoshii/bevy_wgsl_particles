use bevy::{
    app::Plugin, color::LinearRgba, math::Vec3, render::render_resource::ShaderType,
    shader::load_shader_library,
};

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "particle.wesl");
    }
}

/// Defines common attributes of a particle.
#[derive(Default, ShaderType, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifetime: f32,
    pub repeats: i32,
    pub color: LinearRgba,
}
