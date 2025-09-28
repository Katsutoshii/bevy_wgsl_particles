#import bevy_wgsl_particles::particle::{Particle, ComputeInput, particles};

@group(0) @binding(1) var<uniform> dt: f32;


// Update the position of each particle.
@compute @workgroup_size(1)
fn init(in: ComputeInput) {
    let velocity = vec3<f32>(0.1, 0.0, 0.0);
    let i = in.id.x;
    particles[i].velocity = velocity;
}

// Update the position of each particle.
@compute @workgroup_size(1)
fn update(in: ComputeInput) {
    let i = in.id.x;
    particles[i].position += particles[i].velocity * dt;
}
