import bevy_wgsl_particles::particle::{ComputeInput, Particle, frand3, pcg_hash, seed};

@group(0) @binding(0)
var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1)
var<uniform> dt: f32;

const half3 = vec3<f32>(0.5, 0.5, 0.5);
const white = vec4<f32>(1.0, 1.0, 1.0, 1.0);
const drag: f32 = 1.0;
const alpha_fade: f32 = 1.0;
const position_spawn_r: f32 = 10.0;
const velocity_factor: f32 = 2.0;
const effect_seed: u32 = 12345;

// Update the features of each particle.
@compute @workgroup_size(constants::WORKGROUP_SIZE_X)
fn update(in: ComputeInput) {
    let i = in.id.x;

    if particles[i].lifetime <= 0.0 {
        return;
    }

    if particles[i].initialized == 0 {
        seed = pcg_hash(i ^ effect_seed);

        particles[i].position = position_spawn_r * (frand3() - half3);
        particles[i].velocity = velocity_factor * particles[i].position;
        particles[i].color = white;
        particles[i].lifetime = 1000.0;
        particles[i].initialized = 1;
    }

    particles[i].velocity -= drag * particles[i].velocity * dt;
    particles[i].position += particles[i].velocity * dt;
    particles[i].color.a -= alpha_fade * dt;
    particles[i].lifetime -= dt;
}
