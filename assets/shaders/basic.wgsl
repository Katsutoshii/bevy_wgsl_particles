#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::{mesh_view_bindings::globals};
#import bevy_wgsl_particles::particle::Particle;

@group(2) @binding(0) var<uniform> color: vec4<f32>;
@group(2) @binding(1) var<uniform> vertices_per_particle: u32;
@group(2) @binding(2) var color_texture: texture_2d<f32>;
@group(2) @binding(3) var color_texture_sampler: sampler;
@group(2) @binding(4) var<storage, read> particles: array<Particle>;

struct Vertex {
    @builtin(vertex_index) index: u32,
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let i = vertex.index / vertices_per_particle;
    let offset = particles[i].position;
    let position = vertex.position + offset;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(position, 1.0),
    );
    out.uv = vertex.uv;
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let color_mult = vec4<f32>(1.0, 1.0, 1.0, 0.2);
    return color_mult * color * textureSample(color_texture, color_texture_sampler, input.uv);
}
