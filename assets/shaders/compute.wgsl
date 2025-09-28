#import bevy_wgsl_particles::particle::{Particle, ComputeInput, particles, frand3, seed, pcg_hash};

@group(0) @binding(1) var<uniform> dt: f32;

const half3 = vec3<f32>(0.5, 0.5, 0.5);
const white = vec4<f32>(1.0, 1.0, 1.0, 1.0);
const drag: f32 = 1.0;
const alpha_fade: f32 = 1.0;
const position_spawn_r: f32 = 10.0;
const velocity_factor: f32 = 2.0;
const effect_seed: u32 = 12345;


// Initialize the velocity of each particle.
@compute @workgroup_size(16)
fn init(in: ComputeInput) {
    let i = in.id.x;
    seed = pcg_hash(i ^ effect_seed);

    particles[i].position = position_spawn_r * (frand3() - half3);
    particles[i].velocity = velocity_factor * particles[i].position;
    particles[i].color = white;
}

// Update the features of each particle.
@compute @workgroup_size(16)
fn update(in: ComputeInput) {
    let i = in.id.x;

    particles[i].velocity -= drag * particles[i].velocity * dt;
    particles[i].position += particles[i].velocity * dt;
    particles[i].color.a -= alpha_fade * dt;
}
