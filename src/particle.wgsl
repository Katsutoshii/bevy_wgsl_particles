#define_import_path bevy_wgsl_particles::particle


@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

struct Particle {
    position: vec3<f32>,
    velocity: vec3<f32>,
};

struct ComputeInput {
    @builtin(global_invocation_id) id: vec3<u32>,
};
