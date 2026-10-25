use bevy::{
    app::Plugin,
    color::{Color, LinearRgba},
    math::Vec3,
    render::render_resource::ShaderType,
    shader::load_shader_library,
};

pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        load_shader_library!(app, "particle.wesl");
    }
}

/// Defines common attributes of a particle.
#[derive(ShaderType, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub lifetime: f32,
    pub initialized: u32,
    pub color: LinearRgba,
}
impl Default for Particle {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            lifetime: 0.0,
            initialized: true as u32, // Prevent inactive particles from initializing themselves
            color: Color::WHITE.to_linear(),
        }
    }
}
