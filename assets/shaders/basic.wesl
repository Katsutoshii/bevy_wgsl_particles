import bevy_pbr::render::{
    mesh_functions::{get_world_from_local, mesh_position_local_to_clip},
    mesh_view_bindings::globals
};
import bevy_wgsl_particles::particle::Particle;

struct ParticleUniforms {
    color: vec4<f32>,
    vertices_per_particle: u32,
}

struct Vertex {
    @builtin(vertex_index) index: u32,
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @interpolate(flat)
    @location(1) i: u32,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0)
var<uniform> uniforms: ParticleUniforms;
@group(constants::MATERIAL_BIND_GROUP) @binding(1)
var color_texture: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(2)
var color_texture_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(3)
var<storage, read> particles: array<Particle>;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let i = vertex.index / uniforms.vertices_per_particle;
    let offset = particles[i].position;
    let position = offset + vec3<f32>(vertex.position.xy, 0.0);

    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
            get_world_from_local(vertex.instance_index),
            vec4<f32>(position, 1.0),
        );
    out.uv = vertex.uv;
    out.i = i;
    return out;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let i = input.i;
    return particles[i].color * uniforms.color
        * textureSample(color_texture, color_texture_sampler, input.uv);
}
